// SPDX-License-Identifier: CC0-1.0

//! Errors returned by the async production client.
//!
//! The error model is intentionally richer than the sync client's: it preserves enough context
//! to drive operational concerns (retry, alerting, debugging) without forcing every caller to
//! string-match on Core's error messages. Every variant carries the JSON-RPC method name when
//! known so that logs and metrics can be cut by method without extra plumbing.

use std::{fmt, io};

/// Crate-specific `Result` shorthand for the async client.
pub type Result<T> = std::result::Result<T, Error>;

/// Classification of an [`Error`] for retry purposes.
///
/// Downstream callers can match on this to decide whether to retry, fail fast, or fall back to a
/// secondary endpoint. The classification is conservative: only errors known to be transient are
/// reported as [`Retryability::Retryable`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Retryability {
    /// Retrying the operation is likely safe and may succeed.
    Retryable,
    /// Retrying will not change the outcome (bad input, invariant violation, decode error, etc.).
    NonRetryable,
}

/// The error type produced by the async production client.
///
/// Unlike the sync testing client's error, this enum is `non_exhaustive`; new variants may be
/// added without a major version bump. Match with a `_ =>` arm or use the inspector methods
/// ([`Error::retryability`], [`Error::is_tx_not_found`], …) instead of exhaustive matches.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The builder was given invalid configuration (bad URL, bad cookie file, etc.).
    Config(ConfigError),
    /// The HTTP transport failed to deliver the request or the response.
    Transport(Box<dyn std::error::Error + Send + Sync>),
    /// Authentication was rejected by the remote (HTTP 401 or 403).
    Auth { method: Option<String> },
    /// The server returned a non-200 HTTP status that did not contain a JSON-RPC body.
    Http { method: Option<String>, status_code: i32, body: String },
    /// The server's response was not a valid JSON-RPC 2.0 message.
    MalformedResponse { method: Option<String>, source: serde_json::Error },
    /// The server returned a JSON-RPC application error (Core's RPC error object).
    Rpc { method: Option<String>, code: i32, message: String },
    /// The configured Core version does not support the requested method.
    ///
    /// Surfaced when a Core RPC error indicates the method is unknown (-32601) and we have a
    /// configured Core version, so callers can distinguish "wrong server" from "wrong call".
    VersionMismatch { method: String, configured_version: &'static str },
    /// Decoding the server's `result` field into the expected raw type failed.
    Decode { method: String, source: serde_json::Error },
}

/// Detail for [`Error::Config`].
#[derive(Debug)]
#[non_exhaustive]
pub enum ConfigError {
    /// The configured URL was missing, empty, or did not have an HTTP/HTTPS scheme.
    InvalidUrl(String),
    /// A cookie file was specified but could not be read.
    CookieFileIo { path: String, source: io::Error },
    /// A cookie file was read but did not contain a `user:pass` line.
    CookieFileMalformed { path: String },
    /// `Auth::None` was passed where credentials are required.
    MissingCredentials,
    /// The configured timeout was zero (would otherwise deadlock the request).
    ZeroTimeout,
}

impl Error {
    /// Returns the JSON-RPC method that produced this error, if known.
    pub fn method(&self) -> Option<&str> {
        match self {
            Error::Config(_) | Error::Transport(_) => None,
            Error::Auth { method }
            | Error::Http { method, .. }
            | Error::MalformedResponse { method, .. }
            | Error::Rpc { method, .. } => method.as_deref(),
            Error::VersionMismatch { method, .. } | Error::Decode { method, .. } =>
                Some(method.as_str()),
        }
    }

    /// Classifies this error for retry decisions.
    ///
    /// The classification deliberately treats only obviously-transient failures as
    /// [`Retryability::Retryable`]. Application-level errors from Core (`Error::Rpc`) are
    /// classified per the well-known Core error code: the
    /// [`-28` "loading wallet" / "verifying blocks"](https://github.com/bitcoin/bitcoin/blob/master/src/rpc/protocol.h)
    /// family is retryable; everything else is non-retryable.
    pub fn retryability(&self) -> Retryability {
        match self {
            // Transport failures are usually network blips. Retry.
            Error::Transport(_) => Retryability::Retryable,
            // 5xx and 429 are retryable; 4xx generally is not.
            Error::Http { status_code, .. } => {
                if *status_code == 429 || (*status_code >= 500 && *status_code < 600) {
                    Retryability::Retryable
                } else {
                    Retryability::NonRetryable
                }
            }
            // RPC_IN_WARMUP (-28): node is still starting up.
            Error::Rpc { code, .. } if *code == -28 => Retryability::Retryable,
            Error::Rpc { .. } => Retryability::NonRetryable,
            // Everything below is a programmer or operator error; retrying changes nothing.
            Error::Config(_)
            | Error::Auth { .. }
            | Error::MalformedResponse { .. }
            | Error::VersionMismatch { .. }
            | Error::Decode { .. } => Retryability::NonRetryable,
        }
    }

    /// Returns `true` if this error indicates a missing transaction.
    ///
    /// Maps Core's `-5` (`RPC_INVALID_ADDRESS_OR_KEY`) raised by `getrawtransaction` /
    /// `gettransaction`. Recognises Core's canonical messages:
    /// * "No such mempool or blockchain transaction" (`getrawtransaction` against an unknown txid)
    /// * "Invalid or non-wallet transaction id" (`gettransaction` against a non-wallet txid)
    pub fn is_tx_not_found(&self) -> bool {
        matches!(self, Error::Rpc { code: -5, message, .. } if {
            let lower = message.to_lowercase();
            lower.contains("transaction") || lower.contains("txid")
        })
    }

    /// Returns `true` if this error indicates a missing block.
    ///
    /// Maps Core's `-5` (`RPC_INVALID_ADDRESS_OR_KEY`) for `getblock` / `getblockheader`.
    /// Recognises Core's canonical "Block not found" message; we deliberately use the exact
    /// phrase rather than `contains("block")` to avoid colliding with messages like
    /// "No such mempool or **block**chain transaction".
    pub fn is_block_not_found(&self) -> bool {
        matches!(self, Error::Rpc { code: -5, message, .. } if {
            let lower = message.to_lowercase();
            lower.contains("block not found") || lower.contains("block height out of range")
        })
    }

    /// Returns `true` if this error indicates a missing or invalid input (e.g. a malformed txid
    /// passed to a wallet RPC, or an output that has been spent).
    ///
    /// Covers Core's `-8` (`RPC_INVALID_PARAMETER`), `-22` (`RPC_DESERIALIZATION_ERROR`), and
    /// `-25` (`RPC_VERIFY_ERROR`).
    pub fn is_missing_or_invalid_input(&self) -> bool {
        matches!(self, Error::Rpc { code: -8 | -22 | -25, .. })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Config(e) => write!(f, "configuration error: {}", e),
            Error::Transport(e) => write!(f, "transport error: {}", e),
            Error::Auth { method } => match method {
                Some(m) => write!(f, "authentication failed for method `{}`", m),
                None => write!(f, "authentication failed"),
            },
            Error::Http { method, status_code, body } => match method {
                Some(m) =>
                    write!(f, "HTTP {} for method `{}`: {}", status_code, m, truncate(body, 256)),
                None => write!(f, "HTTP {}: {}", status_code, truncate(body, 256)),
            },
            Error::MalformedResponse { method, source } => match method {
                Some(m) => write!(f, "malformed JSON-RPC response for `{}`: {}", m, source),
                None => write!(f, "malformed JSON-RPC response: {}", source),
            },
            Error::Rpc { method, code, message } => match method {
                Some(m) => write!(f, "RPC error {} for `{}`: {}", code, m, message),
                None => write!(f, "RPC error {}: {}", code, message),
            },
            Error::VersionMismatch { method, configured_version } => write!(
                f,
                "method `{}` is not supported by configured Core version {}",
                method, configured_version
            ),
            Error::Decode { method, source } =>
                write!(f, "failed to decode response for `{}`: {}", method, source),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Config(e) => Some(e),
            Error::Transport(e) => Some(&**e),
            Error::MalformedResponse { source, .. } | Error::Decode { source, .. } => Some(source),
            Error::Auth { .. }
            | Error::Http { .. }
            | Error::Rpc { .. }
            | Error::VersionMismatch { .. } => None,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::InvalidUrl(s) => write!(f, "invalid URL: `{}`", s),
            ConfigError::CookieFileIo { path, source } =>
                write!(f, "could not read cookie file `{}`: {}", path, source),
            ConfigError::CookieFileMalformed { path } =>
                write!(f, "cookie file `{}` is malformed (expected `user:pass`)", path),
            ConfigError::MissingCredentials =>
                write!(f, "no credentials provided (use cookie or user/pass)"),
            ConfigError::ZeroTimeout => write!(f, "timeout must be greater than zero"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::CookieFileIo { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<ConfigError> for Error {
    fn from(e: ConfigError) -> Self { Error::Config(e) }
}

/// Maps a `jsonrpc::Error` into the production [`Error`] enum.
///
/// The method name is supplied externally so the caller can decorate the error after the call
/// site (jsonrpc-level errors don't carry the method on their own).
pub(crate) fn from_jsonrpc(method: &str, err: jsonrpc::Error) -> Error {
    use jsonrpc::Error as J;

    match err {
        J::Json(source) => Error::MalformedResponse { method: Some(method.to_owned()), source },
        // RPC_METHOD_NOT_FOUND: the bindings are generated for a specific Core version, so a
        // method the server does not know means the server is not that version (or build).
        J::Rpc(rpc) if rpc.code == -32601 => Error::VersionMismatch {
            method: method.to_owned(),
            configured_version: super::client::CONFIGURED_VERSION,
        },
        J::Rpc(rpc) =>
            Error::Rpc { method: Some(method.to_owned()), code: rpc.code, message: rpc.message },
        J::Transport(boxed) => map_transport_error(method, boxed),
        J::NonceMismatch | J::VersionMismatch => Error::Transport(Box::new(JsonRpcProtocolError(
            "JSON-RPC nonce or version mismatch in response",
        ))),
        // Batches are not (yet) exposed by the production client; surface as a transport error.
        e => Error::Transport(Box::new(e)),
    }
}

/// Wrapper used to convey JSON-RPC level protocol mismatches up the stack.
#[derive(Debug)]
struct JsonRpcProtocolError(&'static str);

impl fmt::Display for JsonRpcProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

impl std::error::Error for JsonRpcProtocolError {}

/// Inspect a transport error from the bitreq HTTP transport and lift HTTP-level failures up.
fn map_transport_error(method: &str, err: Box<dyn std::error::Error + Send + Sync>) -> Error {
    if let Some(http_err) = err.downcast_ref::<jsonrpc::bitreq_http_async::Error>() {
        match http_err {
            jsonrpc::bitreq_http_async::Error::Http(h) =>
                if h.status_code == 401 || h.status_code == 403 {
                    Error::Auth { method: Some(method.to_owned()) }
                } else {
                    Error::Http {
                        method: Some(method.to_owned()),
                        status_code: h.status_code,
                        body: h.body.clone(),
                    }
                },
            // Json/Bitreq are bona fide transport-layer issues from the client's POV.
            _ => Error::Transport(err),
        }
    } else {
        Error::Transport(err)
    }
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    // Back up to a char boundary so slicing can never panic on multi-byte content.
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retryability_transport_is_retryable() {
        let err = Error::Transport(Box::new(io::Error::other("boom")));
        assert_eq!(err.retryability(), Retryability::Retryable);
    }

    #[test]
    fn retryability_5xx_and_429_are_retryable() {
        for status in [500, 502, 503, 504, 429] {
            let err = Error::Http { method: None, status_code: status, body: String::new() };
            assert_eq!(
                err.retryability(),
                Retryability::Retryable,
                "expected status {} to be retryable",
                status
            );
        }
    }

    #[test]
    fn retryability_4xx_other_than_429_is_non_retryable() {
        let err = Error::Http { method: None, status_code: 400, body: String::new() };
        assert_eq!(err.retryability(), Retryability::NonRetryable);
    }

    #[test]
    fn retryability_rpc_warmup_is_retryable() {
        let err = Error::Rpc {
            method: Some("getblockcount".into()),
            code: -28,
            message: "Loading block index...".into(),
        };
        assert_eq!(err.retryability(), Retryability::Retryable);
    }

    #[test]
    fn retryability_rpc_other_is_non_retryable() {
        let err = Error::Rpc {
            method: Some("getblock".into()),
            code: -5,
            message: "Block not found".into(),
        };
        assert_eq!(err.retryability(), Retryability::NonRetryable);
    }

    #[test]
    fn retryability_config_decode_conversion_are_non_retryable() {
        let cfg = Error::Config(ConfigError::ZeroTimeout);
        assert_eq!(cfg.retryability(), Retryability::NonRetryable);

        let decode = Error::Decode {
            method: "x".into(),
            source: serde_json::from_str::<u32>("not-json").unwrap_err(),
        };
        assert_eq!(decode.retryability(), Retryability::NonRetryable);
    }

    #[test]
    fn is_block_not_found_matches_block_message() {
        let err = Error::Rpc {
            method: Some("getblock".into()),
            code: -5,
            message: "Block not found".into(),
        };
        assert!(err.is_block_not_found());
        assert!(!err.is_tx_not_found());
    }

    #[test]
    fn is_tx_not_found_matches_transaction_message() {
        let err = Error::Rpc {
            method: Some("getrawtransaction".into()),
            code: -5,
            message: "No such mempool or blockchain transaction".into(),
        };
        assert!(err.is_tx_not_found());
        assert!(!err.is_block_not_found());
    }

    #[test]
    fn is_missing_or_invalid_input_covers_known_codes() {
        for code in [-8, -22, -25] {
            let err = Error::Rpc { method: None, code, message: "x".into() };
            assert!(err.is_missing_or_invalid_input(), "code {} should be invalid input", code);
        }
        let err = Error::Rpc { method: None, code: -1, message: "x".into() };
        assert!(!err.is_missing_or_invalid_input());
    }

    #[test]
    fn method_accessor_returns_method_when_known() {
        let err =
            Error::Rpc { method: Some("getblockcount".into()), code: -1, message: "x".into() };
        assert_eq!(err.method(), Some("getblockcount"));

        let err = Error::Transport(Box::new(io::Error::other("x")));
        assert_eq!(err.method(), None);
    }

    #[test]
    fn config_error_zero_timeout_displays_clearly() {
        let err = Error::Config(ConfigError::ZeroTimeout);
        let msg = err.to_string();
        assert!(msg.contains("timeout"), "got: {}", msg);
    }

    #[test]
    fn rpc_method_not_found_becomes_version_mismatch() {
        let err = from_jsonrpc(
            "getsomethingnew",
            jsonrpc::Error::Rpc(jsonrpc::error::RpcError {
                code: -32601,
                message: "Method not found".into(),
                data: None,
            }),
        );
        match &err {
            Error::VersionMismatch { method, .. } => assert_eq!(method, "getsomethingnew"),
            other => panic!("expected VersionMismatch, got {:?}", other),
        }
        assert_eq!(err.retryability(), Retryability::NonRetryable);
    }

    #[test]
    fn truncate_respects_char_boundaries() {
        // A body of multi-byte characters whose 256-byte mark falls inside a character must not
        // panic when displayed.
        let body = "\u{00e9}".repeat(200); // 400 bytes of two-byte chars.
        let err = Error::Http { method: None, status_code: 500, body };
        let _ = err.to_string();
    }
}
