#![cfg(all(feature = "std", feature = "async"))]

extern crate bitreq;

use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinSet;

/// Spawns a TCP server that replies to every request with an empty `Content-Length: 0` response
/// carrying `Keep-Alive: max=1`, advertising that the connection must be closed after a single
/// request. Returns the bound address.
async fn spawn_keep_alive_max_one_server() -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let (mut sock, _) = match listener.accept().await {
                Ok(x) => x,
                Err(_) => return,
            };
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let mut acc: Vec<u8> = Vec::new();
                loop {
                    let n = match sock.read(&mut buf).await {
                        Ok(0) | Err(_) => return,
                        Ok(n) => n,
                    };
                    acc.extend_from_slice(&buf[..n]);
                    while let Some(end) = find_double_crlf(&acc) {
                        acc.drain(..end);
                        let response = b"HTTP/1.1 200 OK\r\n\
                             Content-Length: 0\r\n\
                             Keep-Alive: max=1\r\n\
                             Connection: keep-alive\r\n\
                             \r\n";
                        if sock.write_all(response).await.is_err() {
                            return;
                        }
                    }
                }
            });
        }
    });
    addr
}

fn find_double_crlf(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n").map(|i| i + 4)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "enable once fixed"]
async fn pipelined_requests_on_keep_alive_max_one() {
    // Number of pipelined requests to fire after the priming request.
    const PIPELINED_REQUESTS: usize = 100;
    // Maximum number of cached connections in the client's pool.
    const POOL_SIZE: usize = 20;

    let addr = spawn_keep_alive_max_one_server().await;
    let url = format!("http://{}/", addr);

    let client = bitreq::Client::new(POOL_SIZE);

    // Prime the connection pool with one non-pipelined request. The server's `Keep-Alive: max=1`
    // header retires the cached connection after this single use, so the pipelined batch below
    // must complete on a fresh connection without hanging.
    let _ = client
        .send_async(bitreq::Request::new(bitreq::Method::Get, &url))
        .await
        .expect("priming request succeeds");

    let mut set = JoinSet::new();
    for i in 0..PIPELINED_REQUESTS {
        let client = client.clone();
        let url = url.clone();
        set.spawn(async move {
            println!("Launching request {}", i);
            let req = bitreq::Request::new(bitreq::Method::Get, url).with_pipelining();
            let res = client.send_async(req).await.expect("pipelined request succeeds");
            println!("Got response {}", i);
            res
        });
    }

    let collect = async {
        let mut results = Vec::with_capacity(PIPELINED_REQUESTS);
        while let Some(res) = set.join_next().await {
            results.push(res.expect("task panicked"));
        }
        results
    };

    let results =
        tokio::time::timeout(Duration::from_secs(10), collect).await.unwrap_or_else(|_| {
            panic!("{PIPELINED_REQUESTS} pipelined requests did not finish within 10s")
        });

    assert_eq!(results.len(), PIPELINED_REQUESTS);
}
