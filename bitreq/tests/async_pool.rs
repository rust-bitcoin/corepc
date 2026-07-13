#![cfg(feature = "async")]

use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const EMPTY_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";

async fn read_request(stream: &mut TcpStream) -> Vec<u8> {
    let mut request = Vec::new();
    let mut buf = [0; 1024];
    loop {
        let read = stream.read(&mut buf).await.unwrap();
        if read == 0 {
            return request;
        }
        request.extend_from_slice(&buf[..read]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") {
            return request;
        }
    }
}

async fn spawn_counting_server(
    expected_requests: usize,
) -> (String, tokio::task::JoinHandle<usize>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let mut accepted = 0;
        let mut served = 0;
        while served < expected_requests {
            let (mut stream, _) = listener.accept().await.unwrap();
            accepted += 1;
            loop {
                if read_request(&mut stream).await.is_empty() {
                    break;
                }
                served += 1;
                stream.write_all(EMPTY_RESPONSE).await.unwrap();
                if served == expected_requests {
                    return accepted;
                }
            }
        }
        accepted
    });
    (format!("http://{addr}/"), server)
}

#[tokio::test]
async fn pool_hits_refresh_lru_order() {
    let (url_a, server_a) = spawn_counting_server(3).await;
    let (url_b, server_b) = spawn_counting_server(1).await;
    let (url_c, server_c) = spawn_counting_server(1).await;
    let client = bitreq::Client::new(2);

    for url in [&url_a, &url_b, &url_a, &url_c, &url_a] {
        assert_eq!(client.send_async(bitreq::get(url)).await.unwrap().status_code, 200);
    }

    let accepted_a = tokio::time::timeout(Duration::from_secs(5), server_a).await.unwrap().unwrap();
    let accepted_b = server_b.await.unwrap();
    let accepted_c = server_c.await.unwrap();
    assert_eq!(accepted_a, 1, "the recently used connection was evicted");
    assert_eq!((accepted_b, accepted_c), (1, 1));
}
