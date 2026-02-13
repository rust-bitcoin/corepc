use alloc::collections::BTreeMap;
use core::fmt;
#[cfg(feature = "std")]
use core::fmt::Write;
use core::time::Duration;
#[cfg(feature = "std")]
use std::env;
#[cfg(feature = "std")]
use std::time::Instant;

#[cfg(feature = "async")]
use crate::connection::AsyncConnection;
#[cfg(feature = "std")]
use crate::connection::Connection;
use crate::http_url::percent_encode_string;
#[cfg(feature = "std")]
use crate::http_url::{HttpUrl, Port};
#[cfg(feature = "proxy")]
use crate::proxy::Proxy;
#[cfg(feature = "std")]
use crate::{Error, Response, ResponseLazy};

/// A URL type for requests.
pub type URL = String;

/// An HTTP request method.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Method {
    /// The GET method
    Get,
    /// The HEAD method
    Head,
    /// The POST method
    Post,
    /// The PUT method
    Put,
    /// The DELETE method
    Delete,
    /// The CONNECT method
    Connect,
    /// The OPTIONS method
    Options,
    /// The TRACE method
    Trace,
    /// The PATCH method
    Patch,
    /// A custom method, use with care: the string will be embedded in
    /// your request as-is.
    Custom(String),
}

impl fmt::Display for Method {
    /// Formats the Method to the form in the HTTP request,
    /// ie. Method::Get -> "GET", Method::Post -> "POST", etc.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Method::Get => write!(f, "GET"),
            Method::Head => write!(f, "HEAD"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Connect => write!(f, "CONNECT"),
            Method::Options => write!(f, "OPTIONS"),
            Method::Trace => write!(f, "TRACE"),
            Method::Patch => write!(f, "PATCH"),
            Method::Custom(ref s) => write!(f, "{}", s),
        }
    }
}

/// An HTTP request.
///
/// Generally created by the [`bitreq::get`](fn.get.html)-style
/// functions, corresponding to the HTTP method we want to use.
///
/// # Example
///
/// ```
/// let request = bitreq::post("http://example.com");
/// ```
///
/// After creating the request, you would generally call
/// [`send`](struct.Request.html#method.send) or
/// [`send_lazy`](struct.Request.html#method.send_lazy) on it, as it
/// doesn't do much on its own.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Request {
    pub(crate) method: Method,
    url: URL,
    params: String,
    headers: BTreeMap<String, String>,
    body: Option<Vec<u8>>,
    timeout: Option<u64>,
    pub(crate) pipelining: bool,
    pub(crate) max_headers_size: Option<usize>,
    pub(crate) max_status_line_len: Option<usize>,
    pub(crate) max_body_size: Option<usize>,
    max_redirects: usize,
    #[cfg(feature = "proxy")]
    pub(crate) proxy: Option<Proxy>,
}

impl Request {
    /// Creates a new HTTP `Request`.
    ///
    /// This is only the request's data, it is not sent yet. For
    /// sending the request, see [`send`](struct.Request.html#method.send).
    ///
    /// The resource part of the URL will be encoded. Any URL special characters (e.g. &, #, =) are
    /// not encoded as they are assumed to be meaningful parameters etc.
    pub fn new<T: Into<URL>>(method: Method, url: T) -> Request {
        Request {
            method,
            url: url.into(),
            params: String::new(),
            headers: BTreeMap::new(),
            body: None,
            timeout: None,
            pipelining: false,
            // Default matches chrome as of 2022-11:
            // https://groups.google.com/a/chromium.org/g/chromium-os-discuss/c/in-f59OKYAE/m/uVanwcXkAgAJ
            // https://source.chromium.org/chromium/chromium/src/+/refs/heads/main:net/http/http_stream_parser.h;l=164-168;drc=66941d1f0cfe9155b400aef887fe39a403c1f518
            max_headers_size: Some(256 * 1024),
            // Probably could be 128 bytes, but set conservatively for good measure.
            max_status_line_len: Some(64 * 1024),
            // Picked somewhat randomly
            max_body_size: Some(1024 * 1024 * 1024),
            max_redirects: 100,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Add headers to the request this is called on. Use this
    /// function to add headers to your requests.
    pub fn with_headers<T, K, V>(mut self, headers: T) -> Request
    where
        T: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let headers = headers.into_iter().map(|(k, v)| (k.into(), v.into()));
        self.headers.extend(headers);
        self
    }

    /// Adds a header to the request this is called on. Use this
    /// function to add headers to your requests.
    pub fn with_header<T: Into<String>, U: Into<String>>(mut self, key: T, value: U) -> Request {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Sets the request body.
    pub fn with_body<T: Into<Vec<u8>>>(mut self, body: T) -> Request {
        let body = body.into();
        let body_length = body.len();
        self.body = Some(body);
        self.with_header("Content-Length", format!("{}", body_length))
    }

    /// Adds given key and value as query parameter to request url
    /// (resource).
    ///
    /// The key and value are both encoded.
    pub fn with_param<T: Into<String>, U: Into<String>>(mut self, key: T, value: U) -> Request {
        let key = key.into();
        let key = percent_encode_string(&key);
        let value = value.into();
        let value = percent_encode_string(&value);

        if !self.params.is_empty() {
            self.params.push('&');
        }
        self.params.push_str(&key);
        self.params.push('=');
        self.params.push_str(&value);
        self
    }

    /// Converts given argument to JSON and sets it as body.
    ///
    /// # Errors
    ///
    /// Returns
    /// [`SerdeJsonError`](enum.Error.html#variant.SerdeJsonError) if
    /// Serde runs into a problem when converting `body` into a
    /// string.
    #[cfg(feature = "json-using-serde")]
    pub fn with_json<T: serde::ser::Serialize>(mut self, body: &T) -> Result<Request, Error> {
        self.headers
            .insert("Content-Type".to_string(), "application/json; charset=UTF-8".to_string());
        match serde_json::to_vec(&body) {
            Ok(json) => Ok(self.with_body(json)),
            Err(err) => Err(Error::SerdeJsonError(err)),
        }
    }

    /// Sets the request timeout in seconds.
    pub fn with_timeout(mut self, timeout: u64) -> Request {
        self.timeout = Some(timeout);
        self
    }

    /// Sets the max redirects we follow until giving up. 100 by
    /// default.
    ///
    /// Warning: setting this to a very high number, such as 1000, may
    /// cause a stack overflow if that many redirects are followed. If
    /// you have a use for so many redirects that the stack overflow
    /// becomes a problem, please open an issue.
    pub fn with_max_redirects(mut self, max_redirects: usize) -> Request {
        self.max_redirects = max_redirects;
        self
    }

    /// Sets the maximum size of all the headers this request will
    /// accept.
    ///
    /// If this limit is passed, the request will close the connection
    /// and return an [Error::HeadersOverflow] error.
    ///
    /// The maximum length is counted in bytes, including line-endings
    /// and other whitespace. Both normal and trailing headers count
    /// towards this cap.
    ///
    /// `None` disables the cap, and may cause the program to use any
    /// amount of memory if the server responds with a lot of headers
    /// (or an infinite amount). The default is 256KiB.
    pub fn with_max_headers_size<S: Into<Option<usize>>>(mut self, max_headers_size: S) -> Request {
        self.max_headers_size = max_headers_size.into();
        self
    }

    /// Sets the maximum length of the status line this request will
    /// accept.
    ///
    /// If this limit is passed, the request will close the connection
    /// and return an [Error::StatusLineOverflow] error.
    ///
    /// The maximum length is counted in bytes, including the
    /// line-ending `\r\n`.
    ///
    /// `None` disables the cap, and may cause the program to use any
    /// amount of memory if the server responds with a long (or
    /// infinite) status line. The default is 64 KiB.
    pub fn with_max_status_line_length<S: Into<Option<usize>>>(
        mut self,
        max_status_line_len: S,
    ) -> Request {
        self.max_status_line_len = max_status_line_len.into();
        self
    }

    /// Sets the maximum size of the response body this request will
    /// accept.
    ///
    /// If this limit is passed, the request will close the connection
    /// and return an [Error::BodyOverflow] error.
    ///
    /// The maximum size is counted in bytes.
    ///
    /// `None` disables the cap, and may cause the program to use any
    /// amount of memory if the server responds with a large (or
    /// infinite) body.
    ///
    /// The default is 1 GiB, which is likely to cause an
    /// out-of-memory condition in many cases so setting this
    /// manually is recommended when talking to untrusted servers.
    pub fn with_max_body_size<S: Into<Option<usize>>>(mut self, max_body_size: S) -> Request {
        self.max_body_size = max_body_size.into();
        self
    }

    /// Sets the proxy to use.
    #[cfg(feature = "proxy")]
    pub fn with_proxy(mut self, proxy: Proxy) -> Request {
        self.proxy = Some(proxy);
        self
    }

    /// Enables HTTP request pipelining for this request.
    ///
    /// Note that because pipelined requests may be replayed in case of failure, you should only
    /// set this on idempotent requests.
    ///
    /// This is only used if the request is sent using a [`Client`] and an existing connection to
    /// the same server with the same proxy exists.
    ///
    /// [`Client`]: crate::Client
    #[cfg(feature = "async")]
    pub fn with_pipelining(mut self) -> Request {
        self.pipelining = true;
        self
    }

    /// Sends this request to the host.
    ///
    /// # Errors
    ///
    /// Returns `Err` if we run into an error while sending the
    /// request, or receiving/parsing the response. The specific error
    /// is described in the `Err`, and it can be any
    /// [`bitreq::Error`](enum.Error.html) except
    /// [`InvalidUtf8InBody`](enum.Error.html#variant.InvalidUtf8InBody).
    #[cfg(feature = "std")]
    pub fn send(self) -> Result<Response, Error> {
        let parsed_request = ParsedRequest::new(self)?;
        let is_head = parsed_request.config.method == Method::Head;
        let max_body_size = parsed_request.config.max_body_size;
        let connection =
            Connection::new(parsed_request.connection_params(), parsed_request.timeout_at)?;
        let response = connection.send(parsed_request)?;
        Response::create(response, is_head, max_body_size)
    }

    /// Sends this request to the host, loaded lazily.
    ///
    /// # Errors
    ///
    /// See [`send`](struct.Request.html#method.send).
    #[cfg(feature = "std")]
    pub fn send_lazy(self) -> Result<ResponseLazy, Error> {
        let parsed_request = ParsedRequest::new(self)?;
        Connection::new(parsed_request.connection_params(), parsed_request.timeout_at)?
            .send(parsed_request)
    }

    /// Sends this request to the host asynchronously.
    ///
    /// # Errors
    ///
    /// Returns `Err` if we run into an error while sending the
    /// request, or receiving/parsing the response. The specific error
    /// is described in the `Err`, and it can be any
    /// [`bitreq::Error`](enum.Error.html) except
    /// [`InvalidUtf8InBody`](enum.Error.html#variant.InvalidUtf8InBody).
    #[cfg(feature = "async")]
    pub async fn send_async(self) -> Result<Response, Error> {
        let parsed_request = ParsedRequest::new(self)?;
        AsyncConnection::new(parsed_request.connection_params(), parsed_request.timeout_at, None)
            .await?
            .send(parsed_request)
            .await
    }

    /// Sends this request to the host asynchronously, "loaded lazily".
    ///
    /// Note that due to API limitations the response is not actually loaded lazily - it is loaded
    /// immediately and then can be re-read from the response. In a future version an
    /// `AsyncResponseLazy` will be added and lazy loading support will be added.
    ///
    /// Until then, you should use [`Self::send_async`].
    ///
    /// # Errors
    ///
    /// See [`send_async`](struct.Request.html#method.send_async).
    #[cfg(feature = "async")]
    pub async fn send_lazy_async(self) -> Result<ResponseLazy, Error> {
        let response = self.send_async().await?;
        Ok(ResponseLazy::dummy_from_response(response))
    }
}

#[cfg(feature = "std")]
pub(crate) struct ParsedRequest {
    pub(crate) url: HttpUrl,
    pub(crate) redirects: Vec<HttpUrl>,
    pub(crate) config: Request,
    pub(crate) timeout_at: Option<Instant>,
}

#[cfg(feature = "std")]
impl ParsedRequest {
    #[allow(unused_mut)]
    pub(crate) fn new(mut config: Request) -> Result<ParsedRequest, Error> {
        let mut url = HttpUrl::parse(&config.url, None)?;

        if !config.params.is_empty() {
            if url.path_and_query.contains('?') {
                url.path_and_query.push('&');
            } else {
                url.path_and_query.push('?');
            }
            url.path_and_query.push_str(&config.params);
        }

        #[cfg(all(feature = "proxy", feature = "std"))]
        // Set default proxy from environment variables
        //
        // Curl documentation: https://everything.curl.dev/usingcurl/proxies/env
        //
        // Accepted variables are `http_proxy`, `https_proxy`, `HTTPS_PROXY`, `ALL_PROXY`
        //
        // Note: https://everything.curl.dev/usingcurl/proxies/env#http_proxy-in-lower-case-only
        if config.proxy.is_none() {
            // Set HTTP proxies if request's protocol is HTTPS and they're given
            if url.https {
                if let Ok(proxy) =
                    std::env::var("https_proxy").map_err(|_| std::env::var("HTTPS_PROXY"))
                {
                    if let Ok(proxy) = Proxy::new_http(proxy) {
                        config.proxy = Some(proxy);
                    }
                }
            }
            // Set HTTP proxies if request's protocol is HTTP and they're given
            else if let Ok(proxy) = std::env::var("http_proxy") {
                if let Ok(proxy) = Proxy::new_http(proxy) {
                    config.proxy = Some(proxy);
                }
            }
            // Set any given proxies if neither of HTTP/HTTPS were given
            else if let Ok(proxy) =
                std::env::var("all_proxy").map_err(|_| std::env::var("ALL_PROXY"))
            {
                if let Ok(proxy) = Proxy::new_http(proxy) {
                    config.proxy = Some(proxy);
                }
            }
        }

        let timeout = config.timeout.or_else(|| match env::var("BITREQ_TIMEOUT") {
            Ok(t) => t.parse::<u64>().ok(),
            Err(_) => None,
        });
        let timeout_at = timeout.map(|t| Instant::now() + Duration::from_secs(t));

        Ok(ParsedRequest { url, redirects: Vec::new(), config, timeout_at })
    }

    fn get_http_head(&self) -> String {
        let mut http = String::with_capacity(32);

        // NOTE: As of 2.10.0, the fragment is intentionally left out of the request, based on:
        // - [RFC 3986 section 3.5](https://datatracker.ietf.org/doc/html/rfc3986#section-3.5):
        //   "...the fragment identifier is not used in the scheme-specific
        //   processing of a URI; instead, the fragment identifier is separated
        //   from the rest of the URI prior to a dereference..."
        // - [RFC 7231 section 9.5](https://datatracker.ietf.org/doc/html/rfc7231#section-9.5):
        //   "Although fragment identifiers used within URI references are not
        //   sent in requests..."

        // Add the request line and the "Host" header
        write!(
            http,
            "{} {} HTTP/1.1\r\nHost: {}",
            self.config.method, self.url.path_and_query, self.url.host
        )
        .unwrap();
        if let Port::Explicit(port) = self.url.port {
            write!(http, ":{}", port).unwrap();
        }
        http += "\r\n";

        // Add other headers
        for (k, v) in &self.config.headers {
            write!(http, "{}: {}\r\n", k, v).unwrap();
        }

        if self.config.method == Method::Post
            || self.config.method == Method::Put
            || self.config.method == Method::Patch
        {
            let not_length = |key: &String| {
                let key = key.to_lowercase();
                key != "content-length" && key != "transfer-encoding"
            };
            if self.config.headers.keys().all(not_length) {
                // A user agent SHOULD send a Content-Length in a request message when no Transfer-Encoding
                // is sent and the request method defines a meaning for an enclosed payload body.
                // refer: https://tools.ietf.org/html/rfc7230#section-3.3.2

                // A client MUST NOT send a message body in a TRACE request.
                // refer: https://tools.ietf.org/html/rfc7231#section-4.3.8
                // similar line found for GET, HEAD, CONNECT and DELETE.

                http += "Content-Length: 0\r\n";
            }
        }

        http += "\r\n";
        http
    }

    /// Returns the HTTP request as bytes, ready to be sent to
    /// the server.
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut head = self.get_http_head().into_bytes();
        if let Some(body) = &self.config.body {
            head.extend(body);
        }
        head
    }

    /// Returns the redirected version of this Request, unless an
    /// infinite redirection loop was detected, or the redirection
    /// limit was reached.
    pub(crate) fn redirect_to(&mut self, url: &str) -> Result<(), Error> {
        if url.contains("://") {
            let mut url = HttpUrl::parse(url, Some(&self.url)).map_err(|_| {
                // TODO: Uncomment this for 3.0
                // Error::InvalidProtocolInRedirect
                #[cfg(feature = "std")]
                {
                    Error::IoError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "was redirected to an absolute url with an invalid protocol",
                    ))
                }
                #[cfg(not(feature = "std"))]
                {
                    Error::Other("invalid protocol in redirect")
                }
            })?;
            std::mem::swap(&mut url, &mut self.url);
            self.redirects.push(url);
        } else {
            // The url does not have the protocol part, assuming it's
            // a relative resource.
            let mut absolute_url = String::new();
            self.url.write_base_url_to(&mut absolute_url).unwrap();
            absolute_url.push_str(url);
            let mut url = HttpUrl::parse(&absolute_url, Some(&self.url))?;
            std::mem::swap(&mut url, &mut self.url);
            self.redirects.push(url);
        }

        if self.redirects.len() > self.config.max_redirects {
            Err(Error::TooManyRedirections)
        } else if self.redirects.iter().any(|redirect_url| redirect_url == &self.url) {
            Err(Error::InfiniteRedirectionLoop)
        } else {
            Ok(())
        }
    }

    pub(crate) fn connection_params(&self) -> ConnectionParams<'_> {
        ConnectionParams::from_request(self)
    }
}

/// A key which determines whether an existing connection can be reused
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[cfg(feature = "std")]
pub(crate) struct ConnectionParams<'a> {
    pub(crate) https: bool,
    pub(crate) host: &'a str,
    pub(crate) port: Port,
    #[cfg(feature = "proxy")]
    pub(crate) proxy: Option<&'a Proxy>,
}

#[cfg(feature = "std")]
impl<'a> ConnectionParams<'a> {
    fn from_request(request: &'a ParsedRequest) -> Self {
        Self {
            https: request.url.https,
            host: &request.url.host,
            port: request.url.port,
            #[cfg(feature = "proxy")]
            proxy: request.config.proxy.as_ref(),
        }
    }
}

/// A [`ConnectionParams`] without references.
#[cfg(feature = "std")]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct OwnedConnectionParams {
    pub(crate) https: bool,
    pub(crate) host: String,
    pub(crate) port: Port,
    #[cfg(feature = "proxy")]
    pub(crate) proxy: Option<Proxy>,
}

#[cfg(feature = "std")]
impl PartialEq<ConnectionParams<'_>> for OwnedConnectionParams {
    fn eq(&self, other: &ConnectionParams<'_>) -> bool {
        if self.https != other.https || self.host != other.host || self.port != other.port {
            return false;
        }
        #[cfg(feature = "proxy")]
        {
            self.proxy.as_ref() == other.proxy
        }
        #[cfg(not(feature = "proxy"))]
        {
            true
        }
    }
}

#[cfg(feature = "std")]
impl From<ConnectionParams<'_>> for OwnedConnectionParams {
    fn from(other: ConnectionParams<'_>) -> Self {
        Self {
            https: other.https,
            host: other.host.to_owned(),
            port: other.port,
            #[cfg(feature = "proxy")]
            proxy: other.proxy.cloned(),
        }
    }
}

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Get](enum.Method.html).
pub fn get<T: Into<URL>>(url: T) -> Request { Request::new(Method::Get, url) }

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Head](enum.Method.html).
pub fn head<T: Into<URL>>(url: T) -> Request { Request::new(Method::Head, url) }

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Post](enum.Method.html).
pub fn post<T: Into<URL>>(url: T) -> Request { Request::new(Method::Post, url) }

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Put](enum.Method.html).
pub fn put<T: Into<URL>>(url: T) -> Request { Request::new(Method::Put, url) }

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Delete](enum.Method.html).
pub fn delete<T: Into<URL>>(url: T) -> Request { Request::new(Method::Delete, url) }

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Connect](enum.Method.html).
pub fn connect<T: Into<URL>>(url: T) -> Request { Request::new(Method::Connect, url) }

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Options](enum.Method.html).
pub fn options<T: Into<URL>>(url: T) -> Request { Request::new(Method::Options, url) }

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Trace](enum.Method.html).
pub fn trace<T: Into<URL>>(url: T) -> Request { Request::new(Method::Trace, url) }

/// Alias for [Request::new](struct.Request.html#method.new) with `method` set to
/// [Method::Patch](enum.Method.html).
pub fn patch<T: Into<URL>>(url: T) -> Request { Request::new(Method::Patch, url) }

#[cfg(test)]
#[cfg(feature = "std")]
mod parsing_tests {

    use alloc::collections::BTreeMap;

    use super::{get, ParsedRequest};

    #[test]
    fn test_headers() {
        let mut headers = BTreeMap::new();
        headers.insert("foo".to_string(), "bar".to_string());
        headers.insert("foo".to_string(), "baz".to_string());

        let req = get("http://www.example.org/test/res").with_headers(headers.clone());

        assert_eq!(req.headers, headers);
    }

    #[test]
    fn test_multiple_params() {
        let req = get("http://www.example.org/test/res")
            .with_param("foo", "bar")
            .with_param("asd", "qwe");
        let req = ParsedRequest::new(req).unwrap();
        assert_eq!(&req.url.path_and_query, "/test/res?foo=bar&asd=qwe");
    }

    #[test]
    fn test_domain() {
        let req = get("http://www.example.org/test/res").with_param("foo", "bar");
        let req = ParsedRequest::new(req).unwrap();
        assert_eq!(&req.url.host, "www.example.org");
    }

    #[test]
    fn test_protocol() {
        let req =
            ParsedRequest::new(get("http://www.example.org/").with_param("foo", "bar")).unwrap();
        assert!(!req.url.https);
        let req =
            ParsedRequest::new(get("https://www.example.org/").with_param("foo", "bar")).unwrap();
        assert!(req.url.https);
    }
}

#[cfg(all(test, feature = "std"))]
mod encoding_tests {
    use super::{get, ParsedRequest};

    #[test]
    fn test_with_param() {
        let req = get("http://www.example.org").with_param("foo", "bar");
        let req = ParsedRequest::new(req).unwrap();
        assert_eq!(&req.url.path_and_query, "/?foo=bar");

        let req = get("http://www.example.org").with_param("ówò", "what's this? 👀");
        let req = ParsedRequest::new(req).unwrap();
        assert_eq!(&req.url.path_and_query, "/?%C3%B3w%C3%B2=what%27s%20this%3F%20%F0%9F%91%80");
    }

    #[test]
    fn test_on_creation() {
        let req = ParsedRequest::new(get("http://www.example.org/?foo=bar#baz")).unwrap();
        assert_eq!(&req.url.path_and_query, "/?foo=bar");

        let req = ParsedRequest::new(get("http://www.example.org/?ówò=what's this? 👀")).unwrap();
        assert_eq!(&req.url.path_and_query, "/?%C3%B3w%C3%B2=what%27s%20this?%20%F0%9F%91%80");
    }
}
