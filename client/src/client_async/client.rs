// SPDX-License-Identifier: CC0-1.0

//! The async production [`Client`] and its [`Builder`].
//!
//! The client is intentionally minimal: it owns a [`jsonrpc::client_async::Client`] and exposes
//! [`Client::call_raw`] for arbitrary JSON-RPC calls. All higher-level methods (the generated
//! return-typed wrappers) live in version-specific submodules (`super::v30` / `super::v31`).

use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use jsonrpc::bitreq_http_async::BitreqHttpTransport;
use jsonrpc::client_async::{Client as JsonRpcAsyncClient, Transport};
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

use crate::client_async::auth::Auth;
use crate::client_async::error::{from_jsonrpc, ConfigError, Error, Result};

/// The Bitcoin Core version this client was compiled to talk to.
///
/// Set via the version Cargo features at compile time (the highest enabled version wins). Used in
/// error messages and the version-mismatch heuristic.
pub const CONFIGURED_VERSION: &str = configured_version();

const fn configured_version() -> &'static str {
    #[cfg(feature = "31_0")]
    {
        "31_0"
    }
    #[cfg(all(feature = "30_0", not(feature = "31_0")))]
    {
        "30_0"
    }
    #[cfg(not(any(feature = "30_0", feature = "31_0")))]
    {
        compile_error!(
            "the `client-async` feature requires a version feature (`30_0` or `31_0`) to be enabled"
        )
    }
}

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// An async JSON-RPC client to a Bitcoin Core daemon.
///
/// The client is `Clone` and cheap to clone (the underlying transport is shared via [`Arc`]),
/// which makes it convenient to share across spawned tasks. All RPC calls go through
/// [`jsonrpc::client_async::Client`] and so respect its nonce / version checking.
///
/// # Quick start
///
/// ```no_run
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// use corepc_client::client_async::{Auth, Client};
///
/// let client = Client::builder()
///     .url("http://127.0.0.1:8332")?
///     .auth(Auth::CookieFile("/var/lib/bitcoind/.cookie".into()))
///     .build()?;
///
/// let count = client.get_block_count().await?;
/// println!("tip height: {}", *count);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Client {
    inner: Arc<JsonRpcAsyncClient>,
}

impl Client {
    /// Creates a [`Builder`] for configuring a new [`Client`].
    pub fn builder() -> Builder { Builder::new() }

    /// Constructs a [`Client`] from a pre-built [`jsonrpc::client_async::Transport`].
    ///
    /// Use this escape hatch when you need to configure the transport beyond what the [`Builder`]
    /// exposes (for example, plugging in a custom transport with built-in retry, tracing, or a
    /// proxy). When using the bundled `bitreq` transport, prefer [`Client::builder`].
    pub fn with_transport<T: Transport>(transport: T) -> Self {
        Self { inner: Arc::new(JsonRpcAsyncClient::with_transport(transport)) }
    }

    /// Sends a JSON-RPC request for `method` with `params` and decodes the response as `R`.
    ///
    /// This is the universal escape hatch: it lets callers invoke any RPC method, including ones
    /// the client does not yet wrap natively, and decode the response into any
    /// [`DeserializeOwned`] type. Higher-level wrappers (e.g. [`Client::get_block_count`]) are
    /// thin layers on top of this.
    pub async fn call_raw<R, P>(&self, method: &str, params: &P) -> Result<R>
    where
        R: DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        log::debug!(target: "corepc::async", "request: {}", method);

        let raw = serde_json::value::to_raw_value(params)
            .map_err(|source| Error::Decode { method: method.to_owned(), source })?;
        let raw_ref: Option<&RawValue> = match raw.get() {
            // jsonrpc treats `null` as "no params"; passing `null` explicitly upsets some
            // bitcoind builds (they expect either an array or an object).
            "null" => None,
            _ => Some(&raw),
        };
        let request = self.inner.build_request(method, raw_ref);
        let id = request.id.clone();

        let response =
            self.inner.send_request(request).await.map_err(|e| from_jsonrpc(method, e))?;

        if response.id != id {
            return Err(Error::Transport(Box::new(NonceMismatch)));
        }

        // `Response::result` already extracts the RPC error if any; reuse it then map.
        match response.result::<R>() {
            Ok(v) => Ok(v),
            Err(e) => Err(from_jsonrpc(method, e)),
        }
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "corepc_client::client_async::Client({:?})", self.inner)
    }
}

#[derive(Debug)]
struct NonceMismatch;

impl fmt::Display for NonceMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JSON-RPC response id did not match request id")
    }
}

impl std::error::Error for NonceMismatch {}

/// Builder for [`Client`].
///
/// All validation happens at [`Builder::build`]. Construction itself is infallible; URL parse
/// errors are returned by [`Builder::url`] so the caller can localise them next to the bad
/// argument.
#[derive(Clone, Debug)]
pub struct Builder {
    url: String,
    auth: Auth,
    timeout: Duration,
}

impl Builder {
    /// Returns a new builder with default settings (`http://127.0.0.1:8332`, no auth, 30 s timeout).
    pub fn new() -> Self {
        Self { url: "http://127.0.0.1:8332".to_owned(), auth: Auth::None, timeout: DEFAULT_TIMEOUT }
    }

    /// Sets the URL of the JSON-RPC endpoint.
    ///
    /// Validates that the URL parses and uses an HTTP scheme. The cost of failing fast here is
    /// trivial and surfaces typos before they manifest as cryptic transport errors.
    pub fn url(mut self, url: &str) -> Result<Self> {
        if !is_valid_http_url(url) {
            return Err(ConfigError::InvalidUrl(url.to_owned()).into());
        }
        self.url = url.to_owned();
        Ok(self)
    }

    /// Sets the authentication strategy.
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    /// Convenience wrapper for `auth(Auth::CookieFile(path))`.
    pub fn cookie_file<P: Into<PathBuf>>(self, path: P) -> Self {
        self.auth(Auth::CookieFile(path.into()))
    }

    /// Convenience wrapper for `auth(Auth::UserPass(user, pass))`.
    pub fn user_pass<U: Into<String>, P: Into<String>>(self, user: U, pass: P) -> Self {
        self.auth(Auth::UserPass(user.into(), pass.into()))
    }

    /// Sets the request timeout.
    ///
    /// `bitreq` only supports second-granularity timeouts. Sub-second timeouts are accepted at
    /// build time but rounded up to one second by the transport layer; passing exactly zero is
    /// rejected because it would mean "wait forever, but error immediately" depending on the
    /// transport implementation.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Validates configuration and constructs a [`Client`] using the bundled async `bitreq`
    /// transport.
    ///
    /// Use [`Client::with_transport`] if you need a custom transport.
    pub fn build(self) -> Result<Client> {
        if self.timeout.is_zero() {
            return Err(ConfigError::ZeroTimeout.into());
        }
        // Re-validate the URL: callers may have mutated `self.url` via [`Builder::default`]
        // patterns or future API additions.
        if !is_valid_http_url(&self.url) {
            return Err(ConfigError::InvalidUrl(self.url).into());
        }
        let resolved = self.auth.resolve()?;

        let mut tp_builder = BitreqHttpTransport::builder()
            .timeout(self.timeout)
            .url(&self.url)
            .map_err(|_| -> Error {
                // `Builder::url` is defined to be infallible by the bitreq transport (it
                // returns `Result` for forward compat). Treat any future error as invalid URL.
                ConfigError::InvalidUrl(self.url.clone()).into()
            })?;
        if let Some(auth) = resolved {
            tp_builder = tp_builder.basic_auth(auth.user, auth.pass);
        }
        Ok(Client::with_transport(tp_builder.build()))
    }
}

impl Default for Builder {
    fn default() -> Self { Self::new() }
}

fn is_valid_http_url(s: &str) -> bool {
    // Minimal validator: scheme present, scheme is http or https, and there is something after.
    // We deliberately do not pull in a URL parser here; bitreq itself will reject malformed URLs
    // at request time, and tighter validation belongs in a downstream `url` crate adapter.
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    let rest = if let Some(rest) = lower.strip_prefix("http://") {
        rest
    } else if let Some(rest) = lower.strip_prefix("https://") {
        rest
    } else {
        return false;
    };
    !rest.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_validator_accepts_well_formed_http_urls() {
        for url in [
            "http://127.0.0.1:8332",
            "http://localhost",
            "https://node.example.com:443",
            "HTTP://node",
        ] {
            assert!(is_valid_http_url(url), "expected `{}` to be valid", url);
        }
    }

    #[test]
    fn url_validator_rejects_obvious_garbage() {
        for url in ["", " ", "ftp://node", "127.0.0.1:8332", "http://", "https://"] {
            assert!(!is_valid_http_url(url), "expected `{}` to be invalid", url);
        }
    }

    #[test]
    fn builder_url_rejects_invalid() {
        let err = Builder::new().url("not-a-url").unwrap_err();
        assert!(matches!(err, Error::Config(ConfigError::InvalidUrl(_))), "{:?}", err);
    }

    #[test]
    fn builder_zero_timeout_rejected_at_build() {
        let err = Builder::new()
            .url("http://127.0.0.1:8332")
            .unwrap()
            .timeout(Duration::from_secs(0))
            .build()
            .unwrap_err();
        assert!(matches!(err, Error::Config(ConfigError::ZeroTimeout)), "{:?}", err);
    }

    #[test]
    fn builder_missing_cookie_file_returns_config_error() {
        let err = Builder::new()
            .url("http://127.0.0.1:8332")
            .unwrap()
            .cookie_file("/definitely/does/not/exist.cookie")
            .build()
            .unwrap_err();
        assert!(matches!(err, Error::Config(ConfigError::CookieFileIo { .. })), "{:?}", err);
    }

    #[test]
    fn builder_user_pass_builds_a_client() {
        let client = Builder::new()
            .url("http://127.0.0.1:8332")
            .unwrap()
            .user_pass("alice", "secret")
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        // Round-trip through Debug to confirm we got a real client; transport is opaque.
        let dbg = format!("{:?}", client);
        assert!(dbg.contains("Client"));
    }

    #[test]
    fn with_transport_accepts_a_custom_transport() {
        use jsonrpc::client_async::{BoxFuture, Transport};
        use jsonrpc::Error as J;

        struct StubTransport;
        impl Transport for StubTransport {
            fn send_request<'a>(
                &'a self,
                _: jsonrpc::Request<'a>,
            ) -> BoxFuture<'a, std::result::Result<jsonrpc::Response, J>> {
                Box::pin(async { Err(J::NonceMismatch) })
            }
            fn send_batch<'a>(
                &'a self,
                _: &'a [jsonrpc::Request<'a>],
            ) -> BoxFuture<'a, std::result::Result<Vec<jsonrpc::Response>, J>> {
                Box::pin(async { Ok(vec![]) })
            }
            fn fmt_target(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "stub") }
        }

        // Type-level assertion: with_transport accepts our custom Transport. End-to-end behaviour
        // is covered by `tests/client_async_stub.rs`.
        let _client: Client = Client::with_transport(StubTransport);
    }
}
