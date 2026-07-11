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

fn response_lazy(response: &[u8]) -> Result<Vec<u8>, Error> {
    let (url, server) = spawn_server(response.to_vec());
    let result = bitreq::get(url).send_lazy().and_then(|mut response| {
        let mut body = Vec::new();
        response.read_to_end(&mut body).map_err(Error::IoError)?;
        Ok(body)
    });
    server.join().unwrap();
    result
}

fn check_body_paths(response: &[u8], check: impl Fn(Result<Vec<u8>, Error>)) {
    check(response_sync(response).map(Response::into_bytes));
    check(response_lazy(response));
    #[cfg(feature = "async")]
    check(response_async(response).map(Response::into_bytes));
}

fn response_for_method(method: Method, response: &[u8]) -> Result<Response, Error> {
    let (url, server) = spawn_server(response.to_vec());
    let result = Request::new(method, url).send();
    server.join().unwrap();
    result
}

// RFC 9112 Sections 6-8: message bodies, transfer codings, and incomplete messages.

#[test]
fn section_6_3_request_body_has_content_length() {
    // RFC 9112 Section 6.3: a user agent sending a body MUST send a valid Content-Length or use
    // chunked transfer coding.
    let (response, request) =
        capture_request(|base| bitreq::delete(base).with_body(b"five!".to_vec()));
    response.unwrap();
    assert!(request_text(&request).contains("\r\nContent-Length: 5\r\n"));
    assert!(request.ends_with(b"five!"));
}

#[test]
#[ignore = "TODO: reject Content-Length with Transfer-Encoding (RFC 9112 Section 6.2)"]
fn section_6_2_sender_rejects_content_length_with_transfer_encoding() {
    // RFC 9112 Section 6.2: a sender MUST NOT send Content-Length in a message containing
    // Transfer-Encoding.
    let (response, request) = capture_request(|base| {
        bitreq::post(base).with_body("hello").with_header("Transfer-Encoding", "chunked")
    });
    assert!(response.is_err(), "ambiguous request framing must be rejected");
    let text = request_text(&request).to_ascii_lowercase();
    assert!(!(text.contains("content-length:") && text.contains("transfer-encoding:")));
}

#[test]
#[ignore = "TODO: reject repeated chunked transfer coding (RFC 9112 Section 6.1)"]
fn section_6_1_sender_rejects_repeated_chunked_coding() {
    // RFC 9112 Section 6.1: a sender MUST NOT apply chunked transfer coding more than once.
    let (response, request) = capture_request(|base| {
        bitreq::post(base).with_header("Transfer-Encoding", "chunked, chunked")
    });
    assert!(response.is_err(), "repeated chunked coding must be rejected");
    assert!(!request_text(&request).to_ascii_lowercase().contains("chunked, chunked"));
}

#[test]
#[ignore = "TODO: exclude chunked from TE requests (RFC 9112 Section 7.4)"]
fn section_7_4_te_does_not_list_chunked() {
    // RFC 9112 Section 7.4: a client MUST NOT send the chunked coding name in TE.
    let (response, request) =
        capture_request(|base| bitreq::get(base).with_header("TE", "trailers, chunked"));
    assert!(response.is_err(), "TE must not advertise chunked");
    assert!(!request_text(&request).to_ascii_lowercase().contains("te: trailers, chunked"));
}

#[test]
#[ignore = "TODO: add the TE connection option when sending TE (RFC 9112 Section 7.4)"]
fn section_7_4_te_has_connection_option() {
    // RFC 9112 Section 7.4: a sender of TE MUST also send a TE connection option.
    let (response, request) =
        capture_request(|base| bitreq::get(base).with_header("TE", "trailers"));
    response.unwrap();
    assert!(request_text(&request).lines().any(|line| line.eq_ignore_ascii_case("Connection: TE")));
}

#[test]
fn section_6_3_head_204_and_304_have_no_message_body() {
    // RFC 9112 Section 6.3: HEAD responses and 204/304 responses end after the field section.
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
    assert!(response_for_method(Method::Head, response).unwrap().as_bytes().is_empty());
    for status in ["204 No Content", "304 Not Modified"] {
        let response = format!("HTTP/1.1 {status}\r\nContent-Length: 5\r\n\r\nhello");
        assert!(response_sync(response.as_bytes()).unwrap().as_bytes().is_empty());
    }
}

#[test]
#[ignore = "TODO: consume informational responses before the final response (RFC 9112 Section 6.3)"]
fn section_6_3_informational_response_precedes_final_response() {
    // RFC 9112 Sections 6.3 and 9.2: 1xx responses have no body and precede the final response for
    // the same request.
    let response = b"HTTP/1.1 100 Continue\r\n\r\n\
                     HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
    check_body_paths(response, |result| assert_eq!(result.unwrap(), b"hello"));
}

#[test]
#[ignore = "TODO: treat successful CONNECT responses as tunnels (RFC 9112 Section 6.3)"]
fn section_6_3_successful_connect_ignores_framing_fields() {
    // RFC 9112 Section 6.3: a client receiving a successful CONNECT response MUST ignore
    // Content-Length and Transfer-Encoding because the connection becomes a tunnel.
    let response = b"HTTP/1.1 200 Connection Established\r\nContent-Length: 5\r\n\r\nhello";
    let response = response_for_method(Method::Connect, response).unwrap();
    assert!(response.as_bytes().is_empty());
}

#[test]
fn section_6_3_chunked_takes_precedence_over_content_length() {
    // RFC 9112 Section 6.3: Transfer-Encoding overrides Content-Length when both are received.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\
                     Content-Length: 999\r\n\r\n5\r\nhello\r\n0\r\n\r\n";
    check_body_paths(response, |result| assert_eq!(result.unwrap(), b"hello"));
}

#[test]
#[ignore = "TODO: recognize chunked in transfer-coding lists (RFC 9112 Section 6.3)"]
fn section_6_3_final_chunked_coding_frames_response() {
    // RFC 9112 Sections 6.1 and 6.3: when chunked is the final transfer coding, it determines the
    // response body length even when another coding precedes it.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip, chunked\r\n\r\n\
                     5\r\nhello\r\n0\r\n\r\n";
    check_response_paths(response, |result| {
        let response = result.unwrap();
        assert_eq!(response.as_bytes(), b"hello");
        assert_eq!(response.headers.get("transfer-encoding").map(String::as_str), Some("gzip"));
    });
}

#[test]
#[ignore = "TODO: let non-chunked Transfer-Encoding override Content-Length (RFC 9112 Section 6.3)"]
fn section_6_3_nonchunked_transfer_encoding_is_close_delimited() {
    // RFC 9112 Section 6.3: a response whose final transfer coding is not chunked is delimited by
    // connection close, regardless of Content-Length.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip\r\nContent-Length: 2\r\n\r\nhello";
    check_body_paths(response, |result| assert_eq!(result.unwrap(), b"hello"));
}

#[test]
#[ignore = "TODO: reject Transfer-Encoding in HTTP/1.0 responses (RFC 9112 Section 6.1)"]
fn section_6_1_http_1_0_transfer_encoding_is_faulty() {
    // RFC 9112 Section 6.1: a client receiving Transfer-Encoding in HTTP/1.0 MUST treat framing as
    // faulty and close the connection.
    let response = b"HTTP/1.0 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n0\r\n\r\n";
    check_body_paths(response, |result| assert!(result.is_err()));
}

#[test]
fn section_6_3_malformed_content_length_is_rejected() {
    // RFC 9112 Section 6.3: invalid Content-Length framing is an unrecoverable response error.
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: nope\r\n\r\n";
    check_body_paths(response, |result| assert!(result.is_err()));
}

#[test]
#[ignore = "TODO: accept identical Content-Length lists (RFC 9112 Section 6.3)"]
fn section_6_3_identical_content_length_list_is_accepted() {
    // RFC 9112 Section 6.3: a comma-separated Content-Length list is valid when every value is
    // valid and identical.
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 5, 5\r\n\r\nhello";
    check_body_paths(response, |result| assert_eq!(result.unwrap(), b"hello"));
}

#[test]
#[ignore = "TODO: reject conflicting Content-Length fields (RFC 9112 Section 6.3)"]
fn section_6_3_conflicting_content_lengths_are_rejected() {
    // RFC 9112 Section 6.3: differing Content-Length values make response framing invalid and the
    // user agent MUST discard the response.
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nContent-Length: 6\r\n\r\nhello!";
    check_body_paths(response, |result| assert!(result.is_err()));
}

#[test]
#[ignore = "TODO: detect premature EOF in Content-Length bodies (RFC 9112 Sections 6.3 and 8)"]
fn section_6_3_content_length_response_requires_all_octets() {
    // RFC 9112 Sections 6.3 and 8: EOF before Content-Length octets are received makes the response
    // incomplete and MUST close the connection.
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nshort";
    check_body_paths(response, |result| assert!(result.is_err()));
}

#[test]
#[ignore = "TODO: treat clean async EOF as close-delimited completion (RFC 9112 Section 8)"]
fn section_6_3_close_delimited_response_is_complete() {
    // RFC 9112 Sections 6.3 and 8: a response without length fields is complete when a clean
    // connection close follows an intact field section.
    let response = b"HTTP/1.1 200 OK\r\nX-Test: yes\r\n\r\nclose-delimited";
    check_body_paths(response, |result| assert_eq!(result.unwrap(), b"close-delimited"));
}

#[test]
fn section_7_1_chunked_is_decoded_and_extensions_are_ignored() {
    // RFC 9112 Sections 7.1 and 7.1.1: recipients MUST decode chunked and ignore unrecognized
    // chunk extensions.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
                     5;unknown=value\r\nhello\r\n0\r\n\r\n";
    check_response_paths(response, |result| {
        let response = result.unwrap();
        assert_eq!(response.as_bytes(), b"hello");
        assert_eq!(response.headers.get("content-length").map(String::as_str), Some("5"));
        assert!(!response.headers.contains_key("transfer-encoding"));
    });
}

#[test]
fn section_7_1_chunk_size_overflow_is_rejected() {
    // RFC 9112 Section 7.1: recipients MUST anticipate large hexadecimal chunk sizes and prevent
    // integer overflow or precision loss.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
                     FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF\r\n";
    check_body_paths(response, |result| assert!(result.is_err()));
}

#[test]
#[ignore = "TODO: reject empty chunk-size lines (RFC 9112 Section 7.1)"]
fn section_7_1_empty_chunk_size_is_rejected() {
    // RFC 9112 Section 7.1: chunk-size is one or more hexadecimal digits; an empty line is not a
    // terminating zero chunk.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\r\n";
    check_body_paths(response, |result| assert!(result.is_err()));
}

#[test]
fn section_7_1_chunk_data_requires_crlf() {
    // RFC 9112 Section 7.1: every chunk's data MUST be followed by CRLF.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
                     5\r\nhelloX\r\n0\r\n\r\n";
    check_body_paths(response, |result| assert!(result.is_err()));
}

#[test]
#[ignore = "TODO: require the terminal zero chunk (RFC 9112 Section 8)"]
fn section_8_chunked_response_requires_terminal_zero_chunk() {
    // RFC 9112 Section 8: a chunked response is incomplete until its terminating zero-sized chunk
    // has been received.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n";
    check_body_paths(response, |result| assert!(result.is_err()));
}

#[test]
#[ignore = "TODO: do not merge unmergeable trailers into headers (RFC 9112 Section 7.1.2)"]
fn section_7_1_2_unmergeable_trailer_is_discarded() {
    // RFC 9112 Section 7.1.2: a recipient MUST NOT merge a trailer into headers unless that field's
    // definition explicitly permits and defines safe merging.
    let response = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
                     5\r\nhello\r\n0\r\nX-Unknown: trailer\r\n\r\n";
    check_response_paths(response, |result| {
        let response = result.unwrap();
        assert!(!response.headers.contains_key("x-unknown"));
    });
}

#[test]
#[ignore = "TODO: reject response metadata truncated before the empty line (RFC 9112 Section 8)"]
fn section_8_incomplete_field_section_is_rejected() {
    // RFC 9112 Section 8: EOF before the empty line terminating the field section records an
    // incomplete response rather than accepting truncated metadata.
    let response = b"HTTP/1.1 200 OK\r\nX-Test: yes\r\n";
    check_body_paths(response, |result| assert!(result.is_err()));
}
