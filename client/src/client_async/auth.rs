// SPDX-License-Identifier: CC0-1.0

//! Authentication for the async production client.
//!
//! Two flavours are supported: a `user:pass` pair and Bitcoin Core's `.cookie` file. Cookie
//! resolution happens at builder time (see [`super::Builder::build`]) so the auth header is
//! cached for the lifetime of the [`super::Client`] rather than re-read on every request.

use std::fs;
use std::path::PathBuf;

use crate::client_async::error::{ConfigError, Error};

/// How the client authenticates against `bitcoind`.
///
/// Use [`Auth::None`] only against a node that has been deliberately configured to allow
/// unauthenticated access (i.e. a privately-networked test setup); all default Bitcoin Core
/// configurations require authentication.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Auth {
    /// No authentication. Only safe for unauthenticated test daemons.
    None,
    /// Static `(user, password)` credentials.
    UserPass(String, String),
    /// Path to Bitcoin Core's `.cookie` file. The contents are read once at builder time.
    CookieFile(PathBuf),
}

/// Resolved credentials, ready to be applied to a transport builder.
#[derive(Clone, Debug)]
pub(crate) struct ResolvedAuth {
    pub user: String,
    pub pass: Option<String>,
}

impl Auth {
    /// Resolves this `Auth` into a `(user, pass)` pair, reading the cookie file if necessary.
    ///
    /// Errors are returned as [`ConfigError`] so the builder can surface them as
    /// [`Error::Config`].
    pub(crate) fn resolve(self) -> Result<Option<ResolvedAuth>, Error> {
        match self {
            Auth::None => Ok(None),
            Auth::UserPass(user, pass) => Ok(Some(ResolvedAuth { user, pass: Some(pass) })),
            Auth::CookieFile(path) => {
                let display = path.display().to_string();
                let raw = fs::read_to_string(&path).map_err(|source| {
                    ConfigError::CookieFileIo { path: display.clone(), source }
                })?;
                let line = raw
                    .lines()
                    .next()
                    .ok_or_else(|| ConfigError::CookieFileMalformed { path: display.clone() })?;
                let colon = line
                    .find(':')
                    .ok_or_else(|| ConfigError::CookieFileMalformed { path: display.clone() })?;
                Ok(Some(ResolvedAuth {
                    user: line[..colon].to_owned(),
                    pass: Some(line[colon + 1..].to_owned()),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_temp_file(contents: &str) -> tempfile_lite::Path { tempfile_lite::write(contents) }

    /// Helper to extract the underlying [`PathBuf`] without consuming the [`Drop`]-guarded
    /// wrapper (so the temp file is still cleaned up when the scope ends).
    fn pathbuf(path: &tempfile_lite::Path) -> PathBuf { path.as_path().to_owned() }

    #[test]
    fn user_pass_resolves_directly() {
        let auth = Auth::UserPass("alice".into(), "hunter2".into());
        let resolved = auth.resolve().unwrap().unwrap();
        assert_eq!(resolved.user, "alice");
        assert_eq!(resolved.pass.as_deref(), Some("hunter2"));
    }

    #[test]
    fn none_resolves_to_no_credentials() {
        assert!(Auth::None.resolve().unwrap().is_none());
    }

    #[test]
    fn cookie_file_is_split_on_colon() {
        let path = write_temp_file("__cookie__:secretvalue\n");
        let resolved = Auth::CookieFile(pathbuf(&path)).resolve().unwrap().unwrap();
        assert_eq!(resolved.user, "__cookie__");
        assert_eq!(resolved.pass.as_deref(), Some("secretvalue"));
    }

    #[test]
    fn cookie_file_malformed_returns_config_error() {
        let path = write_temp_file("no-colon-here\n");
        let err = Auth::CookieFile(pathbuf(&path)).resolve().unwrap_err();
        assert!(matches!(err, Error::Config(ConfigError::CookieFileMalformed { .. })), "{:?}", err);
    }

    #[test]
    fn cookie_file_missing_returns_io_error() {
        let path = PathBuf::from("/nonexistent/path/should/not/exist.cookie");
        let err = Auth::CookieFile(path).resolve().unwrap_err();
        assert!(matches!(err, Error::Config(ConfigError::CookieFileIo { .. })), "{:?}", err);
    }

    /// Tiny stdlib-only helper that writes a temporary file. Avoids pulling in `tempfile`.
    mod tempfile_lite {
        use std::io::Write;
        use std::path::PathBuf;
        use std::sync::atomic::{AtomicU64, Ordering};

        static COUNTER: AtomicU64 = AtomicU64::new(0);

        pub struct Path(PathBuf);

        impl Path {
            pub fn as_path(&self) -> &std::path::Path { &self.0 }
        }

        impl Drop for Path {
            fn drop(&mut self) { let _ = std::fs::remove_file(&self.0); }
        }

        pub fn write(contents: &str) -> Path {
            let pid = std::process::id();
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = std::env::temp_dir();
            path.push(format!("corepc-async-test-{}-{}.cookie", pid, n));
            let mut f = std::fs::File::create(&path).expect("create temp cookie file");
            f.write_all(contents.as_bytes()).expect("write temp cookie file");
            Path(path)
        }
    }
}
