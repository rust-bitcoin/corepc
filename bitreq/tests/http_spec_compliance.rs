//! HTTP/1.1 wire-compliance tests for RFC 9112.
//!
//! This file is intentionally organized in RFC order. Requirements on servers and forwarding
//! intermediaries do not apply to bitreq. Sections 10-13, the registries, and the appendices do not
//! describe protocol behavior implemented by bitreq. TLS record framing and closure alerts from
//! Sections 9.7 and 9.8 are delegated to the selected TLS backend.

#![cfg(feature = "std")]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use bitreq::{Error, Method, Request, Response};

const EMPTY_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";

fn header_end(bytes: &[u8]) -> Option<usize> {
    bytes.windows(4).position(|window| window == b"\r\n\r\n").map(|i| i + 4)
}

fn content_length(head: &[u8]) -> usize {
    String::from_utf8_lossy(head)
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
        .unwrap_or(0)
}

fn read_request(stream: &mut TcpStream) -> Vec<u8> {
    stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    let mut request = Vec::new();
    let mut buf = [0u8; 1024];
    loop {
        let read = stream.read(&mut buf).unwrap();
        if read == 0 {
            break;
        }
        request.extend_from_slice(&buf[..read]);
        if let Some(end) = header_end(&request) {
            if request.len() >= end + content_length(&request[..end]) {
                break;
            }
        }
    }
    request
}

fn spawn_server(response: Vec<u8>) -> (String, thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let request = read_request(&mut stream);
        stream.write_all(&response).unwrap();
        request
    });
    (format!("http://{addr}"), handle)
}

fn capture_request(build: impl FnOnce(String) -> Request) -> (Result<Response, Error>, Vec<u8>) {
    let (base_url, server) = spawn_server(EMPTY_RESPONSE.to_vec());
    let response = build(base_url).send();
    (response, server.join().unwrap())
}

fn response_sync(response: &[u8]) -> Result<Response, Error> {
    let (url, server) = spawn_server(response.to_vec());
    let result = bitreq::get(url).send();
    server.join().unwrap();
    result
}

#[cfg(feature = "async")]
fn response_async(response: &[u8]) -> Result<Response, Error> {
    let (url, server) = spawn_server(response.to_vec());
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let result = runtime.block_on(bitreq::get(url).send_async());
    server.join().unwrap();
    result
}

fn check_response_paths(response: &[u8], check: impl Fn(Result<Response, Error>)) {
    check(response_sync(response));
    #[cfg(feature = "async")]
    check(response_async(response));
}

fn request_text(request: &[u8]) -> &str { std::str::from_utf8(request).unwrap() }

// RFC 9112 Sections 2-5: message syntax, request targets, status lines, and field lines.

#[test]
fn section_2_1_request_uses_http_message_format() {
    // RFC 9112 Section 2.1: a message is a start-line, CRLF-delimited fields, an empty line,
    // and an optional body.
    let (response, request) = capture_request(|base| {
        bitreq::post(format!("{base}/resource?answer=42"))
            .with_header("X-Test", "yes")
            .with_body(b"body\r\n".to_vec())
    });
    response.unwrap();
    assert!(request.starts_with(b"POST /resource?answer=42 HTTP/1.1\r\n"));
    let end = header_end(&request).expect("request has a terminating empty line");
    assert_eq!(&request[end..], b"body\r\n");
    assert!(request[..end].windows(2).all(|pair| pair != b"\n\n"));
    assert!(request_text(&request).contains("Content-Length: 6\r\n"));
}

#[test]
fn section_2_2_request_has_no_extra_crlf() {
    // RFC 9112 Section 2.2: a user agent MUST NOT preface or follow a request with an extra CRLF.
    let (response, request) = capture_request(|base| bitreq::post(base).with_body("content"));
    response.unwrap();
    assert!(request.starts_with(b"POST / HTTP/1.1\r\n"));
    assert_eq!(request.matches(b"\r\n\r\n").count(), 1);
    assert!(request.ends_with(b"content"));
}

trait ByteMatches {
    fn matches<'a>(&'a self, needle: &'a [u8]) -> Box<dyn Iterator<Item = usize> + 'a>;
}

impl ByteMatches for [u8] {
    fn matches<'a>(&'a self, needle: &'a [u8]) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(
            self.windows(needle.len())
                .enumerate()
                .filter_map(move |(i, window)| (window == needle).then_some(i)),
        )
    }
}

#[test]
#[ignore = "TODO: reject bare CR in request protocol elements (RFC 9112 Section 2.2)"]
fn section_2_2_sender_rejects_bare_cr() {
    // RFC 9112 Section 2.2: a sender MUST NOT generate a bare CR in protocol elements.
    let (response, request) =
        capture_request(|base| bitreq::get(base).with_header("X-Test", "one\rtwo"));
    assert!(response.is_err(), "a bare CR must be rejected before transmission");
    assert!(!request.windows(7).any(|window| window == b"one\rtwo"));
}

#[test]
#[ignore = "TODO: reject invalid request field names (RFC 9112 Sections 2.2 and 5)"]
fn section_2_2_sender_rejects_whitespace_before_first_field() {
    // RFC 9112 Section 2.2: a sender MUST NOT place whitespace between the start-line and the
    // first field line.
    let (response, request) = capture_request(|base| bitreq::get(base).with_header(" Bad", "yes"));
    assert!(response.is_err(), "an invalid field name must not be transmitted");
    assert!(!request_text(&request).contains("\r\n Bad: yes\r\n"));
}

#[test]
#[ignore = "TODO: validate custom method tokens (RFC 9112 Section 3.1)"]
fn section_3_request_line_uses_a_valid_method_token() {
    // RFC 9112 Sections 3 and 3.1: the request-line uses single SP separators and method is a
    // case-sensitive token.
    let injected = "GET /injected HTTP/1.1\r\nX-Injected: yes";
    let (response, request) = capture_request(|base| {
        Request::new(Method::Custom(injected.to_owned()), format!("{base}/expected"))
    });
    assert!(response.is_err(), "a non-token custom method must be rejected");
    assert!(!request_text(&request).contains("X-Injected"));
}

#[test]
fn section_3_2_host_matches_authority() {
    // RFC 9112 Section 3.2: every HTTP/1.1 request MUST contain Host identical to the target
    // authority, excluding userinfo.
    let (base_url, server) = spawn_server(EMPTY_RESPONSE.to_vec());
    let authority = base_url.strip_prefix("http://").unwrap();
    bitreq::get(format!("http://user:password@{authority}/path")).send().unwrap();
    let request = server.join().unwrap();
    let text = request_text(&request);
    assert!(text.contains(&format!("\r\nHost: {authority}\r\n")));
    assert!(!text.contains("user"));
}

#[test]
#[ignore = "TODO: reject conflicting Host fields (RFC 9112 Section 3.2)"]
fn section_3_2_user_host_cannot_conflict_with_authority() {
    // RFC 9112 Section 3.2: the generated Host value MUST identify the target authority.
    let (response, request) =
        capture_request(|base| bitreq::get(base).with_header("Host", "attacker.example"));
    assert!(response.is_err(), "a conflicting Host field must be rejected");
    assert!(!request_text(&request).contains("Host: attacker.example"));
}

#[test]
fn section_3_2_1_direct_request_uses_origin_form() {
    // RFC 9112 Section 3.2.1: a direct request MUST contain only absolute path and query, use `/`
    // for an empty path, and omit the fragment.
    let (response, request) =
        capture_request(|base| bitreq::get(format!("{base}?query=yes#fragment")));
    response.unwrap();
    let line = request_text(&request).lines().next().unwrap();
    assert_eq!(line, "GET /?query=yes HTTP/1.1");
}

#[test]
#[ignore = "TODO: serialize CONNECT authority-form (RFC 9112 Section 3.2.3)"]
fn section_3_2_3_connect_uses_authority_form() {
    // RFC 9112 Section 3.2.3: CONNECT used to establish a tunnel MUST use authority-form.
    let (response, request) = capture_request(|base| bitreq::connect(format!("{base}/tunnel")));
    response.unwrap();
    let authority =
        request_text(&request).lines().find_map(|line| line.strip_prefix("Host: ")).unwrap();
    assert_eq!(
        request_text(&request).lines().next().unwrap(),
        format!("CONNECT {authority} HTTP/1.1")
    );
}

#[cfg(feature = "proxy")]
fn capture_proxy_exchange(target: &str) -> (Result<Response, Error>, Vec<u8>, Vec<u8>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let proxy_addr = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let connect = read_request(&mut stream);
        stream.write_all(EMPTY_RESPONSE).unwrap();
        let tunneled = read_request(&mut stream);
        stream.write_all(EMPTY_RESPONSE).unwrap();
        (connect, tunneled)
    });
    let proxy = bitreq::Proxy::new_http(proxy_addr.to_string()).unwrap();
    let response = bitreq::get(target).with_proxy(proxy).send();
    let (connect, tunneled) = server.join().unwrap();
    (response, connect, tunneled)
}

#[cfg(feature = "proxy")]
#[test]
#[ignore = "TODO: send Host in proxy CONNECT and preserve explicit ports (RFC 9112 Section 3.2)"]
fn section_3_2_3_proxy_connect_has_authority_target_and_host() {
    // RFC 9112 Sections 3.2 and 3.2.3: a proxy CONNECT request uses authority-form and still sends
    // a Host field identical to that authority, including an explicitly supplied default port.
    let (response, connect, tunneled) = capture_proxy_exchange("http://example.com:80/resource");
    response.unwrap();
    let connect = request_text(&connect);
    assert_eq!(connect.lines().next().unwrap(), "CONNECT example.com:80 HTTP/1.1");
    assert!(connect.contains("\r\nHost: example.com:80\r\n"));
    assert_eq!(request_text(&tunneled).lines().next().unwrap(), "GET /resource HTTP/1.1");
    assert!(request_text(&tunneled).contains("\r\nHost: example.com:80\r\n"));
}

#[test]
fn section_4_valid_status_line_is_parsed() {
    // RFC 9112 Section 4: status-line is HTTP-version, SP, three digits, SP, and an optional reason.
    check_response_paths(b"HTTP/1.1 204 \r\nDate: now\r\n\r\n", |result| {
        let response = result.unwrap();
        assert_eq!(response.status_code, 204);
        assert_eq!(response.reason_phrase, "");
    });
}

#[test]
#[ignore = "TODO: validate response status-line grammar (RFC 9112 Sections 2.3 and 4)"]
fn section_4_invalid_status_lines_are_rejected() {
    // RFC 9112 Sections 2.3 and 4: HTTP is case-sensitive and status-code is exactly three digits.
    for line in ["http/1.1 200 OK", "HTTP/1.1 20 OK", "HTTP/1.1 200OK"] {
        let response = format!("{line}\r\nContent-Length: 0\r\n\r\n");
        check_response_paths(response.as_bytes(), |result| {
            assert!(result.is_err(), "invalid status-line was accepted: {line}");
        });
    }
}

#[test]
#[ignore = "TODO: reject or replace bare CR in responses (RFC 9112 Section 2.2)"]
fn section_2_2_bare_cr_in_response_is_rejected_or_replaced() {
    // RFC 9112 Section 2.2: a recipient MUST reject a bare CR or replace it with SP.
    let response = b"HTTP/1.1 200 OK\r\nX-Test: one\rtwo\r\nContent-Length: 0\r\n\r\n";
    check_response_paths(response, |result| {
        if let Ok(response) = result {
            assert_eq!(response.headers.get("x-test").map(String::as_str), Some("one two"));
        }
    });
}

#[test]
#[ignore = "TODO: reject or ignore whitespace-prefixed fields (RFC 9112 Section 2.2)"]
fn section_2_2_whitespace_prefixed_fields_are_rejected_or_ignored() {
    // RFC 9112 Section 2.2: whitespace-prefixed lines after the status-line MUST cause rejection or
    // be consumed without processing.
    let response = b"HTTP/1.1 200 OK\r\n Injected: yes\r\nGood: yes\r\nContent-Length: 0\r\n\r\n";
    check_response_paths(response, |result| {
        if let Ok(response) = result {
            assert!(!response.headers.contains_key(" injected"));
            assert_eq!(response.headers.get("good").map(String::as_str), Some("yes"));
        }
    });
}

#[test]
#[ignore = "TODO: replace obs-fold before interpreting fields (RFC 9112 Section 5.2)"]
fn section_5_2_obsolete_folding_is_replaced() {
    // RFC 9112 Section 5.2: a user agent receiving obs-fold MUST replace it with SP before
    // interpreting the field value.
    let response = b"HTTP/1.1 200 OK\r\nX-Test: first\r\n second\r\nContent-Length: 0\r\n\r\n";
    check_response_paths(response, |result| {
        let response = result.unwrap();
        assert_eq!(response.headers.get("x-test").map(String::as_str), Some("first second"));
    });
}

#[test]
#[ignore = "TODO: parse response framing as octets (RFC 9112 Section 2.2)"]
fn section_2_2_response_is_parsed_as_octets() {
    // RFC 9112 Sections 2.2 and 5: response parsing operates on octets and field values can carry
    // obs-text; it must not parse the whole message as Unicode first.
    let response = b"HTTP/1.1 200 OK\r\nX-Bytes: \x80\r\nContent-Length: 0\r\n\r\n";
    check_response_paths(response, |result| {
        assert!(result.is_ok(), "an obs-text field value must not invalidate message framing");
    });
}
