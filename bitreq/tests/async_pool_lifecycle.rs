//! Regression test for the async [`Client`](bitreq::Client) pool's LRU
//! bookkeeping: a cache hit must move the entry to the most-recently-used
//! slot, otherwise capacity-driven eviction drops still-warm keys.

#![cfg(feature = "async")]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn bind_ephemeral() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let base_url = format!("http://127.0.0.1:{}", port);
    (listener, base_url)
}

/// Reads bytes from `stream` until the HTTP header terminator `\r\n\r\n`
/// is seen. Returns the accumulated buffer. Assumes no request body, which
/// is true for the GETs issued by this test.
async fn read_request_headers(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(512);
    let mut chunk = [0u8; 256];
    loop {
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            return Ok(buf);
        }
    }
}

const KEEP_ALIVE_RESPONSE: &[u8] =
    b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\nConnection: keep-alive\r\nKeep-Alive: timeout=60\r\n\r\nok\n";

#[tokio::test]
async fn pool_hit_refreshes_lru_position() {
    // Capacity = 2, three distinct hosts (= three distinct `ConnectionKey`s
    // because the port differs). Request order: a, b, a, c, a.
    //
    // A correct LRU refresh-on-hit moves `a` to the most-recent slot at
    // step 3, so step 4's capacity-driven eviction drops `b`, and step 5
    // is a cache hit on `a` — three TCP accepts total. A pool that does
    // not refresh LRU on hit still has `a` as the oldest entry after
    // step 3, so step 4 evicts `a` instead, and step 5 is a miss —
    // four TCP accepts total.
    async fn run_server(listener: TcpListener, accepts: Arc<AtomicUsize>) {
        loop {
            let (mut stream, _) = match listener.accept().await {
                Ok(s) => s,
                Err(_) => return,
            };
            accepts.fetch_add(1, Ordering::SeqCst);
            tokio::spawn(async move {
                loop {
                    if read_request_headers(&mut stream).await.is_err() {
                        return;
                    }
                    if stream.write_all(KEEP_ALIVE_RESPONSE).await.is_err() {
                        return;
                    }
                }
            });
        }
    }

    let (listener_a, url_a) = bind_ephemeral().await;
    let (listener_b, url_b) = bind_ephemeral().await;
    let (listener_c, url_c) = bind_ephemeral().await;

    let accepts_a = Arc::new(AtomicUsize::new(0));
    let accepts_b = Arc::new(AtomicUsize::new(0));
    let accepts_c = Arc::new(AtomicUsize::new(0));

    let srv_a = tokio::spawn(run_server(listener_a, Arc::clone(&accepts_a)));
    let srv_b = tokio::spawn(run_server(listener_b, Arc::clone(&accepts_b)));
    let srv_c = tokio::spawn(run_server(listener_c, Arc::clone(&accepts_c)));

    let client = bitreq::Client::new(2);
    for url in [&url_a, &url_b, &url_a, &url_c, &url_a] {
        let response = client.send_async(bitreq::get(format!("{}/x", url))).await.unwrap();
        assert_eq!(response.status_code, 200);
        assert_eq!(response.as_bytes(), b"ok\n");
    }

    srv_a.abort();
    srv_b.abort();
    srv_c.abort();
    let _ = tokio::join!(srv_a, srv_b, srv_c);

    let total = accepts_a.load(Ordering::SeqCst)
        + accepts_b.load(Ordering::SeqCst)
        + accepts_c.load(Ordering::SeqCst);
    assert_eq!(
        total, 3,
        "request sequence a,b,a,c,a with capacity=2 must refresh a's LRU \
         position on the second hit, keeping it warm past the c-driven \
         eviction — expected 3 accepts (miss a, miss b, miss c), got {}",
        total,
    );
    assert_eq!(accepts_a.load(Ordering::SeqCst), 1, "a must be reused, not re-opened");
}
