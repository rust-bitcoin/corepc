#![cfg(feature = "std")]

extern crate bitreq;
extern crate tiny_http;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Once};
use std::thread;
use std::time::Duration;

use self::tiny_http::{Header, Method, Response, Server, StatusCode};

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        let server = Arc::new(Server::http("localhost:35562").unwrap());
        for _ in 0..16 {
            let server = server.clone();

            thread::spawn(move || loop {
                let mut request = {
                    if let Ok(request) = server.recv() {
                        request
                    } else {
                        continue; // If .recv() fails, just try again.
                    }
                };
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).ok();
                let headers = Vec::from(request.headers());

                let url = String::from(request.url().split('#').next().unwrap());
                macro_rules! respond {
                    ($response: expr) => {{
                        let mut response = $response;
                        use core::hash::{BuildHasher, Hasher};
                        // Get a random value using the only std API to do so - the DefaultHasher
                        let rand_val =
                            std::collections::hash_map::RandomState::new().build_hasher().finish();
                        if rand_val % 2 == 0 {
                            // Include a Connection: close header on 1 in 4 responses.
                            // Sadly tinny_httpd doesn't let us actually close the connection, but
                            // our response handling does it as well.
                            response.add_header(Header::from_str("Connection: close").unwrap());
                        } else {
                            let header = Header::from_str("Connection: keep-alive").unwrap();
                            response.add_header(header);
                        }
                        if rand_val % 3 == 1 {
                            // In 1 of 8 responses (but never if we include a Connection: close),
                            // include a Keep-Alive header with a maximum number of requests and a
                            // timeout.
                            let header = Header::from_str("Keep-alive: timeout=1, max=5").unwrap();
                            response.add_header(header);
                        }
                        request.respond(response).unwrap();
                    }};
                }
                match request.method() {
                    Method::Get if url == "/header_pong" => {
                        for header in headers {
                            if header.field.as_str() == "Ping" {
                                let response = Response::from_string(format!("{}", header.value));
                                respond!(response);
                                return;
                            }
                        }
                        respond!(Response::from_string("No header!"));
                    }

                    Method::Get if url == "/slow_a" => {
                        thread::sleep(Duration::from_secs(2));
                        let response = Response::from_string(format!("j: {}", content));
                        respond!(response);
                    }

                    Method::Get if url == "/a" => {
                        let response = Response::from_string(format!("j: {}", content));
                        respond!(response);
                    }
                    Method::Post if url == "/a" => {
                        let response = Response::from_string("POST to /a is not valid.");
                        respond!(response);
                    }

                    Method::Get if url == "/long_header" => {
                        let mut long_header = String::with_capacity(1000);
                        long_header += "Very-Long-Header: ";
                        for _ in 0..1000 - long_header.len() {
                            long_header += ".";
                        }
                        let long_header = Header::from_str(&long_header).unwrap();
                        let response = Response::empty(200).with_header(long_header);
                        respond!(response);
                    }
                    Method::Get if url == "/massive_content_length" => {
                        let status = StatusCode(200);
                        let body = std::io::empty();
                        let length = 1_000_000_000_000_000;
                        let response = Response::new(status, vec![], body, Some(length), None)
                            .with_chunked_threshold(2 * length);
                        respond!(response);
                    }
                    Method::Get if url == "/long_status_line" => {
                        respond!(Response::empty(203));
                    }

                    Method::Get if url == "/redirect-baz" => {
                        let response = Response::empty(301).with_header(
                            Header::from_str("Location: http://localhost:35562/a#baz").unwrap(),
                        );
                        respond!(response);
                    }

                    Method::Get if url == "/redirect" => {
                        let response = Response::empty(301).with_header(
                            Header::from_bytes(&b"Location"[..], &b"http://localhost:35562/a"[..])
                                .unwrap(),
                        );
                        respond!(response);
                    }
                    Method::Post if url == "/redirect" => {
                        let response = Response::empty(303).with_header(
                            Header::from_bytes(&b"Location"[..], &b"http://localhost:35562/a"[..])
                                .unwrap(),
                        );
                        respond!(response);
                    }

                    Method::Get if url == "/infiniteredirect" => {
                        let response = Response::empty(301).with_header(
                            Header::from_bytes(
                                &b"Location"[..],
                                &b"http://localhost:35562/redirectpong"[..],
                            )
                            .unwrap(),
                        );
                        respond!(response);
                    }
                    Method::Get if url == "/redirectpong" => {
                        let response = Response::empty(301).with_header(
                            Header::from_bytes(
                                &b"Location"[..],
                                &b"http://localhost:35562/infiniteredirect"[..],
                            )
                            .unwrap(),
                        );
                        respond!(response);
                    }
                    Method::Get if url == "/relativeredirect" => {
                        let response = Response::empty(303)
                            .with_header(Header::from_bytes(&b"Location"[..], &b"/a"[..]).unwrap());
                        respond!(response);
                    }

                    Method::Post if url == "/echo" => {
                        respond!(Response::from_string(content));
                    }

                    Method::Head if url == "/b" => {
                        respond!(Response::empty(418));
                    }
                    Method::Post if url == "/c" => {
                        let response = Response::from_string(format!("l: {}", content));
                        respond!(response);
                    }
                    Method::Put if url == "/d" => {
                        let response = Response::from_string(format!("m: {}", content));
                        respond!(response);
                    }
                    Method::Delete if url == "/e" => {
                        let response = Response::from_string(format!("n: {}", content));
                        respond!(response);
                    }
                    Method::Trace if url == "/f" => {
                        let response = Response::from_string(format!("o: {}", content));
                        respond!(response);
                    }
                    Method::Options if url == "/g" => {
                        let response = Response::from_string(format!("p: {}", content));
                        respond!(response);
                    }
                    Method::Connect if url == "/h" => {
                        let response = Response::from_string(format!("q: {}", content));
                        respond!(response);
                    }
                    Method::Patch if url == "/i" => {
                        let response = Response::from_string(format!("r: {}", content));
                        respond!(response);
                    }

                    _ => {
                        respond!(Response::from_string("Not Found").with_status_code(404));
                    }
                }
            });
        }
    });
}

pub fn url(req: &str) -> String { format!("http://localhost:35562{}", req) }

#[cfg(feature = "async")]
static CLIENT: std::sync::OnceLock<bitreq::Client> = std::sync::OnceLock::new();
#[cfg(feature = "async")]
static RUNTIME: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();

pub async fn maybe_make_request(
    request: bitreq::Request,
    _slow_request: bool,
) -> Result<bitreq::Response, bitreq::Error> {
    let response = request.clone().send();
    let lazy_response = request.clone().send_lazy();
    match (&response, lazy_response) {
        (Ok(resp), Ok(mut lazy_resp)) => {
            assert_eq!(lazy_resp.status_code, resp.status_code);
            assert_eq!(lazy_resp.reason_phrase, resp.reason_phrase);
            let mut lazy_bytes = Vec::new();
            lazy_resp.read_to_end(&mut lazy_bytes).unwrap();
            assert_eq!(lazy_bytes, resp.as_bytes());
        }
        (Err(e), Err(lazy_e)) => assert_eq!(format!("{e:?}"), format!("{lazy_e:?}")),
        (res, lazy_res) => panic!("{res:?} != {}", lazy_res.is_err()),
    }

    #[cfg(feature = "async")]
    {
        if let Ok(resp) = &response {
            if resp.url.starts_with("https") && !cfg!(feature = "async-https") {
                return response;
            }
        } else {
            // Assume its not HTTPS or async-https is set
        }
        let async_future = request.clone().send_async();
        let lazy_async_future = request.clone().send_lazy_async();
        let client_future = async move {
            if !_slow_request {
                // In order to ensure that clients are able to continue doing things after the tokio
                // runtime of other tests has been shut down, we spawn them on a global runtime.
                let client = CLIENT.get_or_init(|| bitreq::Client::new(100)).clone();
                let client_runtime = RUNTIME.get_or_init(|| {
                    tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap()
                });
                // Get a random value using the only std API to do so - the DefaultHasher
                use std::hash::{BuildHasher, Hasher};
                let rand_val =
                    std::collections::hash_map::RandomState::new().build_hasher().finish();
                let mut request = request;
                if rand_val % 2 == 0 {
                    // With some probability, pipeline the request
                    request = request.with_pipelining();
                }
                client_runtime.spawn(async move { client.send_async(request).await }).await.unwrap()
            } else {
                request.send_async().await
            }
        };
        let (async_response, lazy_async_response, client_response) =
            tokio::join!(async_future, lazy_async_future, client_future);

        match (&response, &async_response, lazy_async_response, client_response) {
            (Ok(resp), Ok(async_resp), Ok(mut lazy_resp), Ok(client_resp)) => {
                assert_eq!(async_resp.status_code, resp.status_code);
                assert_eq!(async_resp.reason_phrase, resp.reason_phrase);
                assert_eq!(async_resp.as_bytes(), resp.as_bytes());

                assert_eq!(client_resp.status_code, resp.status_code);
                assert_eq!(client_resp.reason_phrase, resp.reason_phrase);
                assert_eq!(client_resp.as_bytes(), resp.as_bytes());

                assert_eq!(lazy_resp.status_code, resp.status_code);
                assert_eq!(lazy_resp.reason_phrase, resp.reason_phrase);
                let mut lazy_bytes = Vec::new();
                lazy_resp.read_to_end(&mut lazy_bytes).unwrap();
                assert_eq!(lazy_bytes, resp.as_bytes());
            }
            (Err(e), Err(async_e), Err(lazy_e), Err(client_e)) => {
                assert_eq!(format!("{e:?}"), format!("{async_e:?}"));
                assert_eq!(format!("{e:?}"), format!("{lazy_e:?}"));
                assert_eq!(format!("{e:?}"), format!("{client_e:?}"));
            }
            (res, async_res, lazy_res, client_res) => {
                panic!("{res:?} != {async_res:?} != {} != {client_res:?}", lazy_res.is_ok());
            }
        }
    }
    response
}

pub async fn make_request(request: bitreq::Request) -> bitreq::Response {
    maybe_make_request(request, false).await.unwrap()
}

pub async fn get_body(request: bitreq::Request) -> String {
    let response = make_request(request).await;
    String::from(response.as_str().unwrap())
}

pub async fn get_status_code(request: bitreq::Request) -> i32 {
    let response = make_request(request).await;
    response.status_code
}
