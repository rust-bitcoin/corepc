//! # Bitreq
//!
//! Simple, minimal-dependency HTTP client.  The library has a very
//! minimal API, so you'll probably know everything you need to after
//! reading a few examples.
//!
//! Note: as a minimal library, bitreq has been written with the
//! assumption that servers are well-behaved. This should be fine for
//! nearly any HTTP(S) you find using standard HTTP(S) servers, but
//! some truly ancient servers may cause spurious failures, especially
//! while using pipelining.
//!
//! # Additional features
//!
//! Since the crate is supposed to be minimal in terms of
//! dependencies, only the `std` feature is enabled by default.
//! Additional functionality can be enabled by specifying features for
//! the `bitreq` dependency in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bitreq = { version = "0.2.0", features = ["https"] }
//! ```
//!
//! Below is the list of all available features.
//!
//! ## `https` or `https-rustls`
//!
//! This feature uses the (very good)
//! [`rustls`](https://crates.io/crates/rustls) crate to secure the
//! connection when needed. It uses `webpki-roots` to load certificate
//! authorities to trust.
//!
//! Note that if no HTTPS feature is enabled (and none are by default),
//! requests to urls that start with `https://` will fail and return an
//! [`HttpsFeatureNotEnabled`](enum.Error.html#variant.HttpsFeatureNotEnabled)
//! error. `https` was the name of this feature until the other https
//! feature variants were added, and is now an alias for
//! `https-rustls`.
//!
//! ## `https-rustls-probe`
//!
//! Like `https-rustls`, but uses the
//! [`rustls-native-certs`](https://crates.io/crates/rustls-native-certs)
//! crate to auto-detect certificate authorities installed in common
//! locations.
//!
//! ## `https-native-tls`
//!
//! Uses the [`native-tls`](https://crates.io/crates/native-tls) crate
//! to secure the connection when needed. This loads the system-native
//! TLS library rather than a Rust-specific one.
//!
//! ## `async`
//!
//! This feature enables asynchronous HTTP requests using tokio. It provides
//! [`send_async()`](struct.Request.html#method.send_async) and
//! [`send_lazy_async()`](struct.Request.html#method.send_lazy_async) methods
//! that return futures for non-blocking operation.
//!
//! It also enables [`Client`](struct.Client.html) to reuse TCP connections
//! across requests.
//!
//! ## `async-https` or `async-https-rustls`
//!
//! Like `https` or `https-rustls` but also uses
//! [`tokio-rustls`](https://crates.io/crates/tokio-rustls) (provided by the
//! `rustls` team) to provide HTTPS support for async connections. Uses
//! `webpki-roots` to load certificate authorities.
//!
//! ## `async-https-rustls-probe`
//!
//! The above except the equivalent of `https-rustls-probe` - this uses
//! [`rustls-native-certs`](https://crates.io/crates/rustls-native-certs)
//! to load certificate authorities.
//!
//! ## `async-https-native-tls`
//!
//! Like `https-native-tls` but also uses the
//! [`tokio-native-tls`](https://crates.io/crates/tokio-native-tls) crate
//! (provided by the `tokio` team) to provide HTTPS support for async
//! connections.
//!
//! ## `proxy`
//!
//! This feature enables HTTP proxy support.
//!
//! # Examples
//!
//! ## Get
//!
//! This is a simple example of sending a GET request and checking the
//! response's body, status code, and reason phrase. The `?` are
//! needed because the server could return invalid UTF-8 in the body,
//! or something could go wrong while sending the request or receiving
//! the response.
//!
//! ```
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! # use std::thread;
//! # use tiny_http::{Response, Server};
//! #
//! # let server = Server::http("127.0.0.1:0")?;
//! # let addr = server.server_addr().to_ip().expect("IP listen addr");
//! # let server_thread = thread::spawn(move || {
//! #     let request = server.recv().expect("server recv");
//! #     let response = Response::from_string("<html></html>");
//! #     let _ = request.respond(response);
//! # });
//! #
//! # let url = format!("http://{addr}/");
//! let response = bitreq::get(&url).with_timeout(10).send()?;
//! assert!(response.as_str()?.contains("</html>"));
//! assert_eq!(200, response.status_code);
//! assert_eq!("OK", response.reason_phrase);
//! # server_thread.join().expect("server thread join");
//! # Ok(()) }
//! # #[cfg(not(feature = "std"))]
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> { Ok(()) }
//! ```
//!
//! Note: you could change the `get` function to `head` or `put` or
//! any other HTTP request method: the api is the same for all of
//! them, it just changes what is sent to the server.
//!
//! ## Body (sending)
//!
//! To include a body, add `with_body("<body contents>")` before
//! `send()`.
//!
//! ```
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let response = bitreq::post("http://example.com")
//!     .with_body("Foobar")
//!     .send()?;
//! # Ok(()) }
//! # #[cfg(not(feature = "std"))]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
//! ```
//!
//! ## Headers (sending)
//!
//! To add a header, add `with_header("Key", "Value")` before
//! `send()`.
//!
//! ```
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let response = bitreq::get("http://example.com")
//!     .with_header("Accept", "text/html")
//!     .send()?;
//! # Ok(()) }
//! # #[cfg(not(feature = "std"))]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
//! ```
//!
//! ## Headers (receiving)
//!
//! Reading the headers sent by the servers is done via the
//! [`headers`](struct.Response.html#structfield.headers) field of the
//! [`Response`](struct.Response.html). Note: the header field names
//! (that is, the *keys* of the `HashMap`) are all lowercase: this is
//! because the names are case-insensitive according to the spec, and
//! this unifies the casings for easier `get()`ing.
//!
//! ```
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let response = bitreq::get("http://example.com").send()?;
//! assert!(response.headers.get("content-type").unwrap().starts_with("text/html"));
//! # Ok(()) }
//! # #[cfg(not(feature = "std"))]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
//! ```
//!
//! ## Timeouts
//!
//! To avoid timing out, or limit the request's response time, use
//! `with_timeout(n)` before `send()`. The given value is in seconds.
//!
//! NOTE: There is no timeout by default.
//!
//! ```no_run
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let response = bitreq::post("http://example.com")
//!     .with_timeout(10)
//!     .send()?;
//! # Ok(()) }
//! # #[cfg(not(feature = "std"))]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
//! ```
//!
//! ## Proxy
//!
//! To use a proxy server, simply create a `Proxy` instance and use
//! `.with_proxy()` on your request.
//!
//! Supported proxy formats are `host:port` and
//! `user:password@proxy:host`. Only HTTP CONNECT proxies are
//! supported at this time.
//!
//! ```no_run
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #[cfg(feature = "proxy")]
//! {
//!     let proxy = bitreq::Proxy::new_http("localhost:8080")?;
//!     let response = bitreq::post("http://example.com")
//!         .with_proxy(proxy)
//!         .send()?;
//!     println!("{}", response.as_str()?);
//! }
//! # Ok(()) }
//! # #[cfg(not(feature = "std"))]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
//! ```
//!
//! # Timeouts
//!
//! By default, a request has no timeout. You can change this in two
//! ways:
//!
//! - Use [`with_timeout`](struct.Request.html#method.with_timeout) on
//!   your request to set the timeout per-request like so:
//!   ```text,ignore
//!   bitreq::get("/").with_timeout(8).send();
//!   ```
//! - Set the environment variable `BITREQ_TIMEOUT` to the desired
//!   amount of seconds until timeout. Ie. if you have a program called
//!   `foo` that uses bitreq, and you want all the requests made by that
//!   program to timeout in 8 seconds, you launch the program like so:
//!   ```text,ignore
//!   $ BITREQ_TIMEOUT=8 ./foo
//!   ```
//!   Or add the following somewhere before the requests in the code.
//!   ```
//!   std::env::set_var("BITREQ_TIMEOUT", "8");
//!   ```
//! If the timeout is set with `with_timeout`, the environment
//! variable will be ignored.

#![deny(missing_docs)]
// std::io::Error::other was added in 1.74, so occurrences of this lint can't be
// fixed before our MSRV gets that high.
#![allow(clippy::io_other_error)]

extern crate alloc;

/// Acquires a mutex lock, recovering from poisoning.
///
/// A mutex becomes poisoned when a thread panics while holding the lock.
/// This macro ignores poisoning and returns the guard anyway, which is
/// usually the right behavior since:
/// - The data may still be in a valid state
/// - Propagating panics across threads is rarely useful
///
/// # Example
///
/// ```ignore
/// use std::sync::Mutex;
///
/// let mutex = Mutex::new(42);
/// let guard = lock!(mutex);
/// ```
#[cfg(feature = "async")]
macro_rules! lock {
    ($mutex:expr) => {
        $mutex.lock().unwrap_or_else(|e| e.into_inner())
    };
}

#[cfg(feature = "std")]
mod client;
#[cfg(feature = "std")]
mod connection;
mod error;
mod http_url;
#[cfg(feature = "proxy")]
mod proxy;
mod request;
mod response;

#[cfg(feature = "async")]
pub use client::{Client, RequestExt};
pub use error::*;
#[cfg(feature = "proxy")]
pub use proxy::*;
pub use request::*;
pub use response::Response;
#[cfg(feature = "std")]
pub use response::ResponseLazy;
