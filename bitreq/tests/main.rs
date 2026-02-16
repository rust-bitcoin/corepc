#![cfg(feature = "std")]

extern crate bitreq;
mod setup;

use std::io;

use self::setup::*;

#[tokio::test]
#[cfg(feature = "rustls")]
async fn test_https() {
    // TODO: Implement this locally.
    assert_eq!(get_status_code(bitreq::get("https://example.com")).await, 200);
    // Test reusing the existing connection in client:
    assert_eq!(get_status_code(bitreq::get("https://example.com")).await, 200);
}

#[tokio::test]
#[cfg(feature = "rustls")]
async fn test_https_with_client() {
    setup();
    let client = bitreq::Client::new(1);
    let response = client.send_async(bitreq::get("https://example.com")).await.unwrap();
    assert_eq!(response.status_code, 200);
}

#[tokio::test]
#[cfg(feature = "rustls")]
async fn test_https_with_client_builder() {
    setup();
    let client = bitreq::Client::builder().build();
    let response = client.send_async(bitreq::get("https://example.com")).await.unwrap();
    assert_eq!(response.status_code, 200);
}

#[tokio::test]
#[cfg(feature = "rustls")]
async fn test_https_with_client_builder_and_cert() {
    setup();
    let cert_der = include_bytes!("test_cert.der");
    let client = bitreq::Client::builder().with_root_certificate(cert_der.as_slice()).build();
    let response = client.send_async(bitreq::get("https://example.com")).await.unwrap();
    assert_eq!(response.status_code, 200);
}

#[tokio::test]
#[cfg(feature = "json-using-serde")]
async fn test_json_using_serde() {
    const JSON_SRC: &str = r#"{
        "str": "Json test",
        "num": 42
    }"#;

    setup();
    let original_json: serde_json::Value = serde_json::from_str(JSON_SRC).unwrap();
    let response =
        make_request(bitreq::post(url("/echo")).with_json(&original_json).unwrap()).await;
    let actual_json: serde_json::Value = response.json().unwrap();
    assert_eq!(actual_json, original_json);
}

#[tokio::test]
async fn test_timeout_too_low() {
    setup();
    let request = bitreq::get(url("/slow_a")).with_body("Q".to_string()).with_timeout(1);
    let result = maybe_make_request(request, true).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_timeout_high_enough() {
    setup();
    let request = bitreq::get(url("/slow_a")).with_body("Q".to_string()).with_timeout(3);
    let result = maybe_make_request(request, true).await.unwrap();
    assert_eq!(result.as_str().unwrap(), "j: Q");
}

#[tokio::test]
async fn test_headers() {
    setup();
    let body = get_body(bitreq::get(url("/header_pong")).with_header("Ping", "Qwerty")).await;
    assert_eq!("Qwerty", body);
}

#[tokio::test]
async fn test_custom_method() {
    use bitreq::Method;
    setup();
    let body =
        get_body(bitreq::Request::new(Method::Custom("GET".to_string()), url("/a")).with_body("Q"))
            .await;
    assert_eq!("j: Q", body);
}

#[tokio::test]
async fn test_get() {
    setup();
    let body = get_body(bitreq::get(url("/a")).with_body("Q")).await;
    assert_eq!(body, "j: Q");
}

#[tokio::test]
async fn test_redirect_get() {
    setup();
    let body = get_body(bitreq::get(url("/redirect")).with_body("Q")).await;
    assert_eq!(body, "j: Q");
}

#[tokio::test]
async fn test_redirect_post() {
    setup();
    // POSTing to /redirect should return a 303, which means we should
    // make a GET request to the given location. This test relies on
    // the fact that the test server only responds to GET requests on
    // the /a path.
    let body = get_body(bitreq::post(url("/redirect")).with_body("Q")).await;
    assert_eq!(body, "j: Q");
}

#[tokio::test]
async fn test_redirect_with_fragment() {
    setup();
    let original_url = url("/redirect#foo");
    let res = make_request(bitreq::get(original_url)).await;
    // Fragment should stay the same, otherwise redirected
    assert_eq!(res.url.as_str(), url("/a#foo"));
}

#[tokio::test]
async fn test_redirect_with_overridden_fragment() {
    setup();
    let original_url = url("/redirect-baz#foo");
    let res = make_request(bitreq::get(original_url)).await;
    // This redirect should provide its own fragment, overriding the initial one
    assert_eq!(res.url.as_str(), url("/a#baz"));
}

#[tokio::test]
async fn test_infinite_redirect() {
    setup();
    let body = maybe_make_request(bitreq::get(url("/infiniteredirect")), false).await;
    assert!(body.is_err());
}

#[tokio::test]
async fn test_relative_redirect_get() {
    setup();
    let body = get_body(bitreq::get(url("/relativeredirect")).with_body("Q")).await;
    assert_eq!(body, "j: Q");
}

#[tokio::test]
async fn test_head() {
    setup();
    assert_eq!(get_status_code(bitreq::head(url("/b"))).await, 418);
}

#[tokio::test]
async fn test_post() {
    setup();
    let body = get_body(bitreq::post(url("/c")).with_body("E")).await;
    assert_eq!(body, "l: E");
}

#[tokio::test]
async fn test_put() {
    setup();
    let body = get_body(bitreq::put(url("/d")).with_body("R")).await;
    assert_eq!(body, "m: R");
}

#[tokio::test]
async fn test_delete() {
    setup();
    assert_eq!(get_body(bitreq::delete(url("/e"))).await, "n: ");
}

#[tokio::test]
async fn test_trace() {
    setup();
    assert_eq!(get_body(bitreq::trace(url("/f"))).await, "o: ");
}

#[tokio::test]
async fn test_options() {
    setup();
    let body = get_body(bitreq::options(url("/g")).with_body("U")).await;
    assert_eq!(body, "p: U");
}

#[tokio::test]
async fn test_connect() {
    setup();
    let body = get_body(bitreq::connect(url("/h")).with_body("I")).await;
    assert_eq!(body, "q: I");
}

#[tokio::test]
async fn test_patch() {
    setup();
    let body = get_body(bitreq::patch(url("/i")).with_body("O")).await;
    assert_eq!(body, "r: O");
}

#[tokio::test]
async fn tcp_connect_timeout() {
    let _listener = std::net::TcpListener::bind("127.0.0.1:32162").unwrap();
    let request =
        bitreq::Request::new(bitreq::Method::Get, "http://127.0.0.1:32162").with_timeout(1);
    let resp = maybe_make_request(request, true).await;
    assert!(resp.is_err());
    if let Some(bitreq::Error::IoError(err)) = resp.err() {
        assert_eq!(err.kind(), io::ErrorKind::TimedOut);
    } else {
        panic!("timeout test request did not return an error");
    }
}

#[tokio::test]
async fn test_header_cap() {
    setup();
    let res =
        maybe_make_request(bitreq::get(url("/long_header")).with_max_headers_size(999), false)
            .await;
    assert!(res.is_err());
    assert!(matches!(res.err(), Some(bitreq::Error::HeadersOverflow)));

    make_request(bitreq::get(url("/long_header")).with_max_headers_size(1500)).await;
}

#[tokio::test]
async fn test_status_line_cap() {
    setup();
    let expected_status_line = "HTTP/1.1 203 Non-Authoritative Information";

    let request = bitreq::get(url("/long_status_line"))
        .with_max_status_line_length(expected_status_line.len() + 1);
    let resp = maybe_make_request(request, false).await;
    assert!(resp.is_err());
    assert!(matches!(resp.err(), Some(bitreq::Error::StatusLineOverflow)));

    let request = bitreq::get(url("/long_status_line"))
        .with_max_status_line_length(expected_status_line.len() + 2);
    make_request(request).await;
}

#[tokio::test]
async fn test_massive_content_length() {
    setup();
    #[cfg(feature = "async")]
    tokio::spawn(bitreq::get(url("/massive_content_length")).send_async());
    std::thread::spawn(|| {
        // If bitreq trusts Content-Length, this should crash pretty much straight away.
        let _ = bitreq::get(url("/massive_content_length")).send();
    });
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    // If it were to crash, it would have at this point. Pass!
}

#[tokio::test]
#[cfg(feature = "async")]
async fn test_future_drop_doesnt_hang() {
    // Test that if a pipelined request on a connection isn't read (by dropping the `Future`) later
    // requests on the same connection immediately get retried on a fresh connection.
    use std::time::{Duration, Instant};

    setup();
    let client = bitreq::Client::new(2);

    // First build a connection so that when we spawn both requests simultaneously below we don't
    // try to connect twice.
    // TODO: Use a local HTTP server that supports pipelining and make the responses take long
    // enough that this test still durably detects cancellation safety issues.
    let _init_connect = client.send_async(bitreq::get("http://example.com")).await;

    // By sending two requests, one which we time out manually ~immediately (just long enough to
    // send the request, but not long enough to have read a response) and one which we simply set a
    // high timeout on, we should have a pending request which goes out but then the `Future` for
    // it is dropped, leaving the second request on the same connection with no ability to read its
    // response.
    // Here our cancellation detection should kick in, allowing the second request to open a fresh
    // connection and get a response immediately.
    let timesout = client.send_async(bitreq::get("http://example.com").with_pipelining());
    let request =
        client.send_async(bitreq::get("http://example.com").with_timeout(10).with_pipelining());

    let start = Instant::now();
    let (timedout, response) =
        tokio::join!(tokio::time::timeout(Duration::from_micros(10), timesout), request);
    assert!(timedout.is_err(), "There's no way we get a response in 10 mics");
    assert!(response.is_ok());
    assert!(
        start.elapsed() < Duration::from_secs(5),
        "Request should complete quickly, and certainly not have to wait for its timeout to try again"
    );
}
