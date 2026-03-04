//! This module implements the [`crate::client_sync::Transport`] trait using [`bitreq`]
//! as the underlying HTTP transport.
//!
//! [bitreq]: <https://github.com/rust-bitcoin/corepc/bitreq>

#[cfg(jsonrpc_fuzz)]
use std::io::{self, Read, Write};
#[cfg(jsonrpc_fuzz)]
use std::sync::Mutex;
use std::time::Duration;
use std::{error, fmt};

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

use crate::client_sync::Transport;
use crate::{Request, Response};

const DEFAULT_URL: &str = "http://localhost";
const DEFAULT_PORT: u16 = 8332; // the default RPC port for bitcoind.
#[cfg(not(jsonrpc_fuzz))]
const DEFAULT_TIMEOUT_SECONDS: u64 = 15;
#[cfg(jsonrpc_fuzz)]
const DEFAULT_TIMEOUT_SECONDS: u64 = 1;

/// An HTTP transport that uses [`bitreq`] and is useful for running a bitcoind RPC client.
#[derive(Clone, Debug)]
pub struct BitreqHttpTransport {
    /// URL of the RPC server.
    url: String,
    /// Timeout only supports second granularity.
    timeout: Duration,
    /// The value of the `Authorization` HTTP header, i.e., a base64 encoding of 'user:password'.
    basic_auth: Option<String>,
}

impl Default for BitreqHttpTransport {
    fn default() -> Self {
        BitreqHttpTransport {
            url: format!("{}:{}", DEFAULT_URL, DEFAULT_PORT),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECONDS),
            basic_auth: None,
        }
    }
}

impl BitreqHttpTransport {
    /// Constructs a new [`BitreqHttpTransport`] with default parameters.
    pub fn new() -> Self { BitreqHttpTransport::default() }

    /// Returns a builder for [`BitreqHttpTransport`].
    pub fn builder() -> Builder { Builder::new() }

    fn request<R>(&self, req: impl serde::Serialize) -> Result<R, Error>
    where
        R: for<'a> serde::de::Deserialize<'a>,
    {
        let req = match &self.basic_auth {
            Some(auth) => bitreq::Request::new(bitreq::Method::Post, &self.url)
                .with_timeout(self.timeout.as_secs())
                .with_header("Authorization", auth)
                .with_json(&req)?,
            None => bitreq::Request::new(bitreq::Method::Post, &self.url)
                .with_timeout(self.timeout.as_secs())
                .with_json(&req)?,
        };

        // Send the request and parse the response. If the response is an error that does not
        // contain valid JSON in its body (for instance if the bitcoind HTTP server work queue
        // depth is exceeded), return the raw HTTP error so users can match against it.
        let resp = req.send()?;
        match resp.json() {
            Ok(json) => Ok(json),
            Err(bitreq_err) =>
                if resp.status_code != 200 {
                    Err(Error::Http(HttpError {
                        status_code: resp.status_code,
                        body: resp.as_str().unwrap_or("").to_string(),
                    }))
                } else {
                    Err(Error::Bitreq(bitreq_err))
                },
        }
    }
}

impl Transport for BitreqHttpTransport {
    fn send_request(&self, req: Request) -> Result<Response, crate::Error> {
        Ok(self.request(req)?)
    }

    fn send_batch(&self, reqs: &[Request]) -> Result<Vec<Response>, crate::Error> {
        Ok(self.request(reqs)?)
    }

    fn fmt_target(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.url) }
}

/// Builder for simple bitcoind [`BitreqHttpTransport`].
#[derive(Clone, Debug)]
pub struct Builder {
    tp: BitreqHttpTransport,
}

impl Builder {
    /// Constructs a new [`Builder`] with default configuration and the URL to use.
    pub fn new() -> Builder { Builder { tp: BitreqHttpTransport::new() } }

    /// Sets the timeout after which requests will abort if they aren't finished.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.tp.timeout = timeout;
        self
    }

    /// Sets the URL of the server to the transport.
    #[allow(clippy::assigning_clones)] // clone_into is only available in Rust 1.63
    pub fn url(mut self, url: &str) -> Result<Self, Error> {
        self.tp.url = url.to_owned();
        Ok(self)
    }

    /// Adds authentication information to the transport.
    pub fn basic_auth(mut self, user: String, pass: Option<String>) -> Self {
        let mut s = user;
        s.push(':');
        if let Some(ref pass) = pass {
            s.push_str(pass.as_ref());
        }
        self.tp.basic_auth = Some(format!("Basic {}", &BASE64.encode(s.as_bytes())));
        self
    }

    /// Adds authentication information to the transport using a cookie string ('user:pass').
    ///
    /// Does no checking on the format of the cookie string, just base64 encodes whatever is passed in.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use jsonrpc::bitreq_http_sync::BitreqHttpTransport;
    /// # use std::fs::{self, File};
    /// # use std::path::Path;
    /// # let cookie_file = Path::new("~/.bitcoind/.cookie");
    /// let mut file = File::open(cookie_file).expect("couldn't open cookie file");
    /// let mut cookie = String::new();
    /// fs::read_to_string(&mut cookie).expect("couldn't read cookie file");
    /// let client = BitreqHttpTransport::builder().cookie_auth(cookie);
    /// ```
    pub fn cookie_auth<S: AsRef<str>>(mut self, cookie: S) -> Self {
        self.tp.basic_auth = Some(format!("Basic {}", &BASE64.encode(cookie.as_ref().as_bytes())));
        self
    }

    /// Builds the final [`BitreqHttpTransport`].
    pub fn build(self) -> BitreqHttpTransport { self.tp }
}

impl Default for Builder {
    fn default() -> Self { Builder::new() }
}

/// An HTTP error.
#[derive(Debug)]
pub struct HttpError {
    /// Status code of the error response.
    pub status_code: i32,
    /// Raw body of the error response.
    pub body: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "status: {}, body: {}", self.status_code, self.body)
    }
}

impl error::Error for HttpError {}

/// Error that can happen when sending requests.
///
/// In case of error, a JSON error is returned if the body of the response could be parsed as such.
/// Otherwise, an HTTP error is returned containing the status code and the raw body.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// JSON parsing error.
    Json(serde_json::Error),
    /// Bitreq error.
    Bitreq(bitreq::Error),
    /// HTTP error that does not contain valid JSON as body.
    Http(HttpError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::Json(ref e) => write!(f, "parsing JSON failed: {}", e),
            Error::Bitreq(ref e) => write!(f, "bitreq: {}", e),
            Error::Http(ref e) => write!(f, "http ({})", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use self::Error::*;

        match *self {
            Json(ref e) => Some(e),
            Bitreq(ref e) => Some(e),
            Http(ref e) => Some(e),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self { Error::Json(e) }
}

impl From<bitreq::Error> for Error {
    fn from(e: bitreq::Error) -> Self { Error::Bitreq(e) }
}

impl From<Error> for crate::Error {
    fn from(e: Error) -> crate::Error {
        match e {
            Error::Json(e) => crate::Error::Json(e),
            e => crate::Error::Transport(Box::new(e)),
        }
    }
}

/// Global mutex used by the fuzzing harness to inject data into the read end of the TCP stream.
#[cfg(jsonrpc_fuzz)]
pub static FUZZ_TCP_SOCK: Mutex<Option<io::Cursor<Vec<u8>>>> = Mutex::new(None);

#[cfg(jsonrpc_fuzz)]
#[derive(Clone, Debug)]
struct TcpStream;

#[cfg(jsonrpc_fuzz)]
mod impls {
    use super::*;

    impl Read for TcpStream {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match *FUZZ_TCP_SOCK.lock().unwrap() {
                Some(ref mut cursor) => io::Read::read(cursor, buf),
                None => Ok(0),
            }
        }
    }
    impl Write for TcpStream {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { io::sink().write(buf) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_sync::Client;

    #[test]
    fn construct() {
        let tp = Builder::new()
            .timeout(Duration::from_millis(100))
            .url("http://localhost:22")
            .unwrap()
            .basic_auth("user".to_string(), None)
            .build();
        let _ = Client::with_transport(tp);
    }
}
