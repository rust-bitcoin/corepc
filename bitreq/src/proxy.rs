use base64::engine::general_purpose::STANDARD;
use base64::engine::Engine;

use crate::error::Error;
use crate::response::maybe_await;

/// Kind of proxy connection (Basic, Digest, SOCKS5, etc)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum ProxyKind {
    Basic,
    Socks5,
}

/// Proxy configuration. Supports HTTP CONNECT proxies ([`Proxy::new_http`])
/// and SOCKS5 proxies ([`Proxy::new_socks5`]).
///
/// SOCKS5 encodes targets per RFC 1928: IPv4 literals as ATYP 0x01, IPv6
/// literals as ATYP 0x04, and anything else as a domain name (ATYP 0x03)
/// with DNS resolution left to the proxy. The latter enables routing
/// through Tor, including `.onion` addresses.
///
/// For HTTP CONNECT proxies, when credentials are provided, the Basic
/// authentication type is used for Proxy-Authorization.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Proxy {
    pub(crate) server: String,
    pub(crate) port: u16,
    pub(crate) user: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) kind: ProxyKind,
}

/// SOCKS5 handshake body, parameterised over sync/async.
///
/// Pass `await` for the async version, nothing for the sync version. The
/// caller's `use` imports decide which `Read`/`Write` trait set is in scope,
/// so the same body resolves to either `std::io` or `tokio::io` calls.
///
/// Uses the `maybe_await!` helper from `response.rs` to thread `.await`
/// through the body, matching the convention used by `define_read_methods!`.
///
/// Macro hygiene means the call site must pass `self`, `stream`, and the
/// target host/port explicitly.
macro_rules! socks5_handshake_body {
    ($self:ident, $stream:ident, $target_host:ident, $target_port:ident $(, $await:tt)?) => {{
        let (greeting, expected_method) = $self.socks5_greeting();
        maybe_await!($stream.write_all(&greeting), $($await)?).map_err(Error::IoError)?;
        maybe_await!($stream.flush(), $($await)?).map_err(Error::IoError)?;

        let mut greeting_resp = [0u8; 2];
        maybe_await!($stream.read_exact(&mut greeting_resp), $($await)?).map_err(Error::IoError)?;
        if greeting_resp[0] != 0x05 || greeting_resp[1] != expected_method {
            return Err(Error::ProxyConnect);
        }

        if let Some((auth_req, auth_len)) = $self.socks5_auth_request() {
            maybe_await!($stream.write_all(&auth_req[..auth_len]), $($await)?).map_err(Error::IoError)?;
            maybe_await!($stream.flush(), $($await)?).map_err(Error::IoError)?;

            let mut auth_resp = [0u8; 2];
            maybe_await!($stream.read_exact(&mut auth_resp), $($await)?).map_err(Error::IoError)?;
            if auth_resp[1] != 0x00 {
                return Err(Error::InvalidProxyCreds);
            }
        }

        let (req, req_len) = Self::socks5_connect_request($target_host, $target_port)?;
        maybe_await!($stream.write_all(&req[..req_len]), $($await)?).map_err(Error::IoError)?;
        maybe_await!($stream.flush(), $($await)?).map_err(Error::IoError)?;

        let mut connect_resp = [0u8; 4];
        maybe_await!($stream.read_exact(&mut connect_resp), $($await)?).map_err(Error::IoError)?;
        if connect_resp[0] != 0x05 || connect_resp[1] != 0x00 {
            return Err(Error::ProxyConnect);
        }

        match connect_resp[3] {
            0x01 => { // IPv4: 4 bytes + 2 port
                let mut buf = [0u8; 6];
                maybe_await!($stream.read_exact(&mut buf), $($await)?).map_err(Error::IoError)?;
            }
            0x03 => { // Domain: 1 len byte + domain + 2 port
                let mut len = [0u8; 1];
                maybe_await!($stream.read_exact(&mut len), $($await)?).map_err(Error::IoError)?;
                // Domain length is u8, so domain + 2 port bytes is at most 257.
                let mut buf = [0u8; 257];
                let total = len[0] as usize + 2;
                maybe_await!($stream.read_exact(&mut buf[..total]), $($await)?).map_err(Error::IoError)?;
            }
            0x04 => { // IPv6: 16 bytes + 2 port
                let mut buf = [0u8; 18];
                maybe_await!($stream.read_exact(&mut buf), $($await)?).map_err(Error::IoError)?;
            }
            _ => return Err(Error::ProxyConnect),
        }

        Ok(())
    }};
}

/// HTTP CONNECT handshake body, parameterised over sync/async.
///
/// See `socks5_handshake_body!` for the dispatch convention.
macro_rules! http_connect_handshake_body {
    ($self:ident, $stream:ident, $target_host:ident, $target_port:ident $(, $await:tt)?) => {{
        let request = $self.connect($target_host, $target_port);
        maybe_await!($stream.write_all(request.as_bytes()), $($await)?).map_err(Error::IoError)?;
        maybe_await!($stream.flush(), $($await)?).map_err(Error::IoError)?;

        let mut buf = [0u8; 8192];
        let mut len = 0;
        let header_end = loop {
            let n = maybe_await!($stream.read(&mut buf[len..]), $($await)?)
                .map_err(Error::IoError)?;
            if n == 0 {
                return Err(Error::ProxyConnect);
            }
            len += n;
            if let Some(idx) = buf[..len].windows(4).position(|w| w == b"\r\n\r\n") {
                break idx;
            }
            if len == buf.len() {
                return Err(Error::ProxyConnect);
            }
        };

        let headers = core::str::from_utf8(&buf[..header_end]).map_err(|_| Error::ProxyConnect)?;
        let status_line = headers.lines().next().ok_or(Error::ProxyConnect)?;
        let (status_code, _) = crate::response::parse_status_line(status_line);
        match status_code {
            200 => Ok(()),
            401 | 407 => Err(Error::InvalidProxyCreds),
            _ => Err(Error::BadProxy),
        }
    }};
}

impl Proxy {
    fn parse_creds(creds: &str) -> (Option<String>, Option<String>) {
        if let Some((user, pass)) = split_once(creds, ":") {
            (Some(user.to_string()), Some(pass.to_string()))
        } else {
            (Some(creds.to_string()), None)
        }
    }

    fn parse_address(host: &str) -> Result<(String, Option<u16>), Error> {
        if let Some((host, port)) = split_once(host, ":") {
            let port = port.parse::<u16>().map_err(|_| Error::BadProxy)?;
            Ok((host.to_string(), Some(port)))
        } else {
            Ok((host.to_string(), None))
        }
    }

    /// Creates a new Proxy configuration for an HTTP proxy supporting the `CONNECT` command.
    ///
    /// Supported proxy format is:
    ///
    /// ```plaintext
    /// [http://][user[:password]@]host[:port]
    /// ```
    ///
    /// The default port is 8080.
    ///
    /// # Example
    ///
    /// ```
    /// let proxy = bitreq::Proxy::new_http("user:password@localhost:1080").unwrap();
    /// let request = bitreq::post("http://example.com").with_proxy(proxy);
    /// ```
    ///
    pub fn new_http<S: AsRef<str>>(proxy: S) -> Result<Self, Error> {
        let proxy = proxy.as_ref();
        let authority = if let Some((proto, auth)) = split_once(proxy, "://") {
            if proto != "http" {
                return Err(Error::BadProxy);
            }
            auth
        } else {
            proxy
        };

        let ((user, password), host) = if let Some((userinfo, host)) = rsplit_once(authority, "@") {
            (Proxy::parse_creds(userinfo), host)
        } else {
            ((None, None), authority)
        };

        let (host, port) = Proxy::parse_address(host)?;

        Ok(Self {
            server: host,
            user,
            password,
            port: port.unwrap_or(8080),
            kind: ProxyKind::Basic,
        })
    }

    /// Creates a new Proxy configuration for a SOCKS5 proxy.
    ///
    /// Supported proxy formats:
    ///
    /// ```plaintext
    /// [socks5://]host[:port]
    /// [socks5h://]host[:port]
    /// ```
    ///
    /// Both `socks5://` and `socks5h://` are accepted and behave identically.
    /// IPv4 and IPv6 literal destinations are encoded with their proper ATYP
    /// (0x01 / 0x04); anything else is sent as a domain name (ATYP 0x03),
    /// leaving DNS resolution to the proxy. The latter is the `socks5h`
    /// privacy expectation and is required for routing through Tor
    /// (including `.onion` addresses).
    ///
    /// The default port is 1080.
    ///
    /// # Example
    ///
    /// ```
    /// let proxy = bitreq::Proxy::new_socks5("127.0.0.1:9050").unwrap();
    /// let request = bitreq::post("http://example.com").with_proxy(proxy);
    /// ```
    ///
    pub fn new_socks5<S: AsRef<str>>(proxy: S) -> Result<Self, Error> {
        let proxy = proxy.as_ref();
        let authority = if let Some((proto, auth)) = split_once(proxy, "://") {
            if proto != "socks5" && proto != "socks5h" {
                return Err(Error::BadProxy);
            }
            auth
        } else {
            proxy
        };

        let (host, port) = Proxy::parse_address(authority)?;

        Ok(Self {
            server: host,
            user: None,
            password: None,
            port: port.unwrap_or(1080),
            kind: ProxyKind::Socks5,
        })
    }

    /// Creates a new SOCKS5 proxy with username/password credentials.
    ///
    /// Credentials trigger RFC 1929 username/password authentication during
    /// the SOCKS5 handshake. Tor uses credentials for circuit isolation:
    /// connections with different credentials are routed through separate
    /// circuits, preventing correlation.
    ///
    /// # Example
    ///
    /// ```
    /// let proxy = bitreq::Proxy::new_socks5_with_credentials(
    ///     "127.0.0.1:9050", "session-1".to_string(), "x".to_string()
    /// ).unwrap();
    /// ```
    ///
    pub fn new_socks5_with_credentials<S: AsRef<str>>(
        proxy: S,
        user: String,
        password: String,
    ) -> Result<Self, Error> {
        let mut p = Self::new_socks5(proxy)?;
        p.set_credentials(user, password)?;
        Ok(p)
    }

    /// Sets username/password credentials on this proxy.
    ///
    /// For SOCKS5, credentials trigger RFC 1929 username/password
    /// authentication during the handshake; for Tor, distinct credentials
    /// also trigger separate circuits, so rotating them on a long-lived
    /// `Proxy` lets a caller cycle isolated circuits without rebuilding
    /// from a URL. For HTTP CONNECT proxies, credentials are sent as
    /// Basic auth in the `Proxy-Authorization` header (RFC 7617).
    ///
    /// Length limits depend on the proxy kind:
    /// - **SOCKS5:** `user` must be 1-255 bytes and `password` 0-255
    ///   bytes (RFC 1929 wire-format limits). Empty `user` is rejected.
    /// - **HTTP:** no protocol-level length limit; practical limits come
    ///   from server-side HTTP header size, which is server-specific.
    ///
    /// Returns [`Error::BadProxy`] when these limits are violated.
    ///
    /// # Example
    ///
    /// ```
    /// let mut proxy = bitreq::Proxy::new_socks5("127.0.0.1:9050").unwrap();
    /// proxy.set_credentials("session-1".to_string(), "x".to_string()).unwrap();
    /// // ... later, for a fresh circuit:
    /// proxy.set_credentials("session-2".to_string(), "x".to_string()).unwrap();
    /// ```
    pub fn set_credentials(&mut self, user: String, password: String) -> Result<(), Error> {
        match self.kind {
            ProxyKind::Socks5 => {
                // RFC 1929 ULEN: 1-255; PLEN: 0-255.
                if user.is_empty() || user.len() > 255 || password.len() > 255 {
                    return Err(Error::BadProxy);
                }
            }
            ProxyKind::Basic => {
                // HTTP Basic (RFC 7617) imposes no protocol length limit.
            }
        }
        self.user = Some(user);
        self.password = Some(password);
        Ok(())
    }

    /// Build the SOCKS5 greeting bytes.
    /// Returns (greeting_bytes, expected_auth_method).
    fn socks5_greeting(&self) -> ([u8; 3], u8) {
        let method = if self.user.is_some() { 0x02 } else { 0x00 };
        ([0x05, 0x01, method], method)
    }

    /// Build the RFC 1929 username/password auth request.
    /// Returns the filled buffer and its length. None if no credentials are set.
    /// Max length is 3 + 255 + 255 = 513 bytes.
    fn socks5_auth_request(&self) -> Option<([u8; 513], usize)> {
        let user = self.user.as_ref()?;
        let pass = self.password.as_deref().unwrap_or("");
        let mut buf = [0u8; 513];
        buf[0] = 0x01; // sub-negotiation version
        buf[1] = user.len() as u8;
        let mut pos = 2;
        buf[pos..pos + user.len()].copy_from_slice(user.as_bytes());
        pos += user.len();
        buf[pos] = pass.len() as u8;
        pos += 1;
        buf[pos..pos + pass.len()].copy_from_slice(pass.as_bytes());
        pos += pass.len();
        Some((buf, pos))
    }

    /// Build the SOCKS5 CONNECT request.
    ///
    /// `target_host` is parsed as an [`IpAddr`] so IPv4 and IPv6 literals
    /// are encoded with their proper ATYP (0x01 / 0x04). Anything else is
    /// treated as a domain name (ATYP 0x03), with DNS resolution left to
    /// the proxy. This mirrors curl's `inet_pton`-based dispatch in
    /// `lib/socks.c` and matches the response parser above, which reads
    /// all three ATYPs.
    ///
    /// Returns the filled buffer and its length. Max length is the domain
    /// case: 4 + 1 + 255 + 2 = 262 bytes.
    fn socks5_connect_request(
        target_host: &str,
        target_port: u16,
    ) -> Result<([u8; 262], usize), Error> {
        use std::net::IpAddr;

        let mut buf = [0u8; 262];
        buf[0] = 0x05; // VER
        buf[1] = 0x01; // CMD = CONNECT
        buf[2] = 0x00; // RSV

        let body_end = match target_host.parse::<IpAddr>() {
            Ok(IpAddr::V4(v4)) => {
                buf[3] = 0x01;
                buf[4..8].copy_from_slice(&v4.octets());
                8
            }
            Ok(IpAddr::V6(v6)) => {
                buf[3] = 0x04;
                buf[4..20].copy_from_slice(&v6.octets());
                20
            }
            Err(_) => {
                let host_bytes = target_host.as_bytes();
                if host_bytes.len() > 255 {
                    return Err(Error::ProxyConnect);
                }
                buf[3] = 0x03;
                buf[4] = host_bytes.len() as u8;
                buf[5..5 + host_bytes.len()].copy_from_slice(host_bytes);
                5 + host_bytes.len()
            }
        };

        buf[body_end] = (target_port >> 8) as u8;
        buf[body_end + 1] = (target_port & 0xff) as u8;
        Ok((buf, body_end + 2))
    }

    pub(crate) fn connect(&self, host: &str, port: u16) -> String {
        let authorization = if let Some(user) = &self.user {
            match self.kind {
                ProxyKind::Basic => {
                    let creds = if let Some(password) = &self.password {
                        STANDARD.encode(format!("{}:{}", user, password))
                    } else {
                        STANDARD.encode(user)
                    };
                    format!("Proxy-Authorization: Basic {}\r\n", creds)
                }
                ProxyKind::Socks5 => unreachable!("SOCKS5 uses binary handshake, not HTTP CONNECT"),
            }
        } else {
            String::new()
        };
        format!("CONNECT {host}:{port} HTTP/1.1\r\n{authorization}\r\n")
    }

    /// Perform a SOCKS5 handshake on a connected TCP stream (sync).
    #[cfg(feature = "std")]
    pub(crate) fn socks5_handshake_sync(
        &self,
        stream: &mut std::net::TcpStream,
        target_host: &str,
        target_port: u16,
    ) -> Result<(), Error> {
        use std::io::{Read, Write};
        socks5_handshake_body!(self, stream, target_host, target_port)
    }

    /// Perform a SOCKS5 handshake on a connected async TCP stream.
    #[cfg(feature = "async")]
    pub(crate) async fn socks5_handshake_async(
        &self,
        stream: &mut tokio::net::TcpStream,
        target_host: &str,
        target_port: u16,
    ) -> Result<(), Error> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        socks5_handshake_body!(self, stream, target_host, target_port, await)
    }

    /// Perform the proxy-specific handshake on a connected sync TCP stream.
    ///
    /// Dispatches to the SOCKS5 binary handshake or the HTTP CONNECT request
    /// based on this proxy's `kind`. Callers don't need to know which.
    #[cfg(feature = "std")]
    pub(crate) fn handshake_sync(
        &self,
        stream: &mut std::net::TcpStream,
        target_host: &str,
        target_port: u16,
    ) -> Result<(), Error> {
        match self.kind {
            ProxyKind::Socks5 => self.socks5_handshake_sync(stream, target_host, target_port),
            ProxyKind::Basic => self.http_connect_handshake_sync(stream, target_host, target_port),
        }
    }

    /// Perform the proxy-specific handshake on a connected async TCP stream.
    ///
    /// Dispatches to the SOCKS5 binary handshake or the HTTP CONNECT request
    /// based on this proxy's `kind`. Callers don't need to know which.
    #[cfg(feature = "async")]
    pub(crate) async fn handshake_async(
        &self,
        stream: &mut tokio::net::TcpStream,
        target_host: &str,
        target_port: u16,
    ) -> Result<(), Error> {
        match self.kind {
            ProxyKind::Socks5 =>
                self.socks5_handshake_async(stream, target_host, target_port).await,
            ProxyKind::Basic =>
                self.http_connect_handshake_async(stream, target_host, target_port).await,
        }
    }

    /// Issue the HTTP CONNECT request and read the proxy response (sync).
    #[cfg(feature = "std")]
    fn http_connect_handshake_sync(
        &self,
        stream: &mut std::net::TcpStream,
        target_host: &str,
        target_port: u16,
    ) -> Result<(), Error> {
        use std::io::{Read, Write};
        http_connect_handshake_body!(self, stream, target_host, target_port)
    }

    /// Issue the HTTP CONNECT request and read the proxy response (async).
    #[cfg(feature = "async")]
    async fn http_connect_handshake_async(
        &self,
        stream: &mut tokio::net::TcpStream,
        target_host: &str,
        target_port: u16,
    ) -> Result<(), Error> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        http_connect_handshake_body!(self, stream, target_host, target_port, await)
    }
}

#[allow(clippy::manual_split_once)]
/// Replacement for str::split_once until MSRV is at least 1.52.0.
fn split_once<'a>(string: &'a str, pattern: &str) -> Option<(&'a str, &'a str)> {
    let mut parts = string.splitn(2, pattern);
    let first = parts.next()?;
    let second = parts.next()?;
    Some((first, second))
}

#[allow(clippy::manual_split_once)]
/// Replacement for str::rsplit_once until MSRV is at least 1.52.0.
fn rsplit_once<'a>(string: &'a str, pattern: &str) -> Option<(&'a str, &'a str)> {
    let mut parts = string.rsplitn(2, pattern);
    let second = parts.next()?;
    let first = parts.next()?;
    Some((first, second))
}

#[cfg(test)]
mod tests {
    use super::{Proxy, ProxyKind};

    #[test]
    fn parse_proxy() {
        let proxy = Proxy::new_http("user:p@ssw0rd@localhost:9999").unwrap();
        assert_eq!(proxy.user, Some(String::from("user")));
        assert_eq!(proxy.password, Some(String::from("p@ssw0rd")));
        assert_eq!(proxy.server, String::from("localhost"));
        assert_eq!(proxy.port, 9999);
    }

    #[test]
    fn parse_regular_proxy_with_protocol() {
        let proxy = Proxy::new_http("http://localhost:1080").unwrap();
        assert_eq!(proxy.user, None);
        assert_eq!(proxy.password, None);
        assert_eq!(proxy.server, String::from("localhost"));
        assert_eq!(proxy.port, 1080);
    }

    // --- SOCKS5 parsing tests ---

    #[test]
    fn parse_socks5_host_port() {
        let proxy = Proxy::new_socks5("127.0.0.1:9050").unwrap();
        assert_eq!(proxy.server, "127.0.0.1");
        assert_eq!(proxy.port, 9050);
        assert!(matches!(proxy.kind, ProxyKind::Socks5));
        assert_eq!(proxy.user, None);
    }

    #[test]
    fn parse_socks5_with_protocol() {
        let proxy = Proxy::new_socks5("socks5://localhost:1080").unwrap();
        assert_eq!(proxy.server, "localhost");
        assert_eq!(proxy.port, 1080);
        assert!(matches!(proxy.kind, ProxyKind::Socks5));
    }

    #[test]
    fn parse_socks5h_alias() {
        // socks5h:// is accepted as an alias for socks5:// since both schemes
        // result in remote-DNS behaviour in this implementation.
        let proxy = Proxy::new_socks5("socks5h://localhost:1080").unwrap();
        assert_eq!(proxy.server, "localhost");
        assert_eq!(proxy.port, 1080);
        assert!(matches!(proxy.kind, ProxyKind::Socks5));
    }

    #[test]
    fn parse_socks5_default_port() {
        let proxy = Proxy::new_socks5("localhost").unwrap();
        assert_eq!(proxy.server, "localhost");
        assert_eq!(proxy.port, 1080); // default SOCKS5 port
    }

    #[test]
    fn parse_socks5_wrong_protocol() {
        assert!(Proxy::new_socks5("http://localhost:1080").is_err());
    }

    #[test]
    fn proxy_kind_distinguishes_http_and_socks5() {
        let http = Proxy::new_http("localhost:8080").unwrap();
        let socks = Proxy::new_socks5("localhost:1080").unwrap();
        assert!(matches!(http.kind, ProxyKind::Basic));
        assert!(matches!(socks.kind, ProxyKind::Socks5));
    }

    #[test]
    fn parse_socks5_with_credentials() {
        let proxy = Proxy::new_socks5_with_credentials(
            "127.0.0.1:9050",
            "user1".to_string(),
            "pass1".to_string(),
        )
        .unwrap();
        assert_eq!(proxy.server, "127.0.0.1");
        assert_eq!(proxy.port, 9050);
        assert!(matches!(proxy.kind, ProxyKind::Socks5));
        assert_eq!(proxy.user, Some("user1".to_string()));
        assert_eq!(proxy.password, Some("pass1".to_string()));
    }

    #[test]
    fn socks5_credentials_length_validation() {
        // Empty username rejected
        assert!(Proxy::new_socks5_with_credentials(
            "localhost:9050",
            String::new(),
            "pass".to_string()
        )
        .is_err());
        // Empty user with non-empty password is rejected (RFC 1929 ULEN >= 1).
        assert!(Proxy::new_socks5_with_credentials(
            "localhost:9050",
            String::new(),
            "some-password".to_string()
        )
        .is_err());
        // Username >255 bytes rejected
        let long_user = "a".repeat(256);
        assert!(Proxy::new_socks5_with_credentials(
            "localhost:9050",
            long_user,
            "pass".to_string()
        )
        .is_err());
        // Password >255 bytes rejected
        let long_pass = "a".repeat(256);
        assert!(Proxy::new_socks5_with_credentials(
            "localhost:9050",
            "user".to_string(),
            long_pass
        )
        .is_err());
        // Max length (255) accepted
        let max_user = "a".repeat(255);
        assert!(
            Proxy::new_socks5_with_credentials("localhost:9050", max_user, "x".to_string()).is_ok()
        );
    }

    #[test]
    fn socks5_set_credentials_rotation() {
        let mut proxy = Proxy::new_socks5("127.0.0.1:9050").unwrap();
        assert_eq!(proxy.user, None);
        assert_eq!(proxy.password, None);

        proxy.set_credentials("session-1".to_string(), "x".to_string()).unwrap();
        assert_eq!(proxy.user, Some("session-1".to_string()));
        assert_eq!(proxy.password, Some("x".to_string()));

        // Rotating to a new credential pair (Tor circuit isolation).
        proxy.set_credentials("session-2".to_string(), "y".to_string()).unwrap();
        assert_eq!(proxy.user, Some("session-2".to_string()));
        assert_eq!(proxy.password, Some("y".to_string()));
    }

    #[test]
    fn socks5_set_credentials_validation() {
        let mut proxy = Proxy::new_socks5("127.0.0.1:9050").unwrap();
        // Empty user rejected.
        assert!(proxy.set_credentials(String::new(), "pass".to_string()).is_err());
        // Empty user with non-empty password is also rejected.
        assert!(proxy.set_credentials(String::new(), "some-password".to_string()).is_err());
        // Over-long fields rejected.
        let long = "a".repeat(256);
        assert!(proxy.set_credentials(long.clone(), "pass".to_string()).is_err());
        assert!(proxy.set_credentials("user".to_string(), long).is_err());
        // Failed calls don't mutate state.
        assert_eq!(proxy.user, None);
        assert_eq!(proxy.password, None);
    }

    #[test]
    fn http_set_credentials_no_length_limit() {
        // HTTP Basic auth doesn't have a protocol-level length limit, so
        // strings exceeding the SOCKS5 RFC 1929 255-byte cap must succeed
        // on an HTTP proxy.
        let mut proxy = Proxy::new_http("127.0.0.1:8080").unwrap();
        let long_user = "u".repeat(1024);
        let long_pass = "p".repeat(1024);
        assert!(proxy.set_credentials(long_user.clone(), long_pass.clone()).is_ok());
        assert_eq!(proxy.user.as_deref(), Some(long_user.as_str()));
        assert_eq!(proxy.password.as_deref(), Some(long_pass.as_str()));
        // Empty user is also valid for HTTP Basic (RFC 7617 allows it).
        assert!(proxy.set_credentials(String::new(), "p".to_string()).is_ok());
    }

    // --- SOCKS5 handshake tests (sync, with mock server) ---

    #[cfg(feature = "std")]
    mod socks5_handshake {
        use std::io::{Read, Write};
        use std::net::TcpListener;

        use super::*;

        /// Mock SOCKS5 server that accepts one connection and performs the handshake.
        /// Returns `(host, port, atyp)` so callers can assert which ATYP the
        /// client used: 0x01 for IPv4 literals (`host` is e.g. `"127.0.0.1"`),
        /// 0x04 for IPv6 literals (`host` is e.g. `"::1"`), 0x03 for domain
        /// names (`host` is the raw domain).
        fn mock_socks5_server(listener: &TcpListener, reply_status: u8) -> (String, u16, u8) {
            let (mut stream, _) = listener.accept().unwrap();

            // 1. Read greeting
            let mut greeting = [0u8; 3];
            stream.read_exact(&mut greeting).unwrap();
            assert_eq!(greeting[0], 0x05, "SOCKS version must be 5");
            assert_eq!(greeting[1], 0x01, "1 auth method");
            assert_eq!(greeting[2], 0x00, "no-auth method");

            // 2. Reply: accept no-auth
            stream.write_all(&[0x05, 0x00]).unwrap();
            stream.flush().unwrap();

            // 3. Read connect request header
            let mut header = [0u8; 4];
            stream.read_exact(&mut header).unwrap();
            assert_eq!(header[0], 0x05, "SOCKS version");
            assert_eq!(header[1], 0x01, "CONNECT command");
            assert_eq!(header[2], 0x00, "reserved");
            let atyp = header[3];

            let host = match atyp {
                0x01 => {
                    let mut octets = [0u8; 4];
                    stream.read_exact(&mut octets).unwrap();
                    std::net::Ipv4Addr::from(octets).to_string()
                }
                0x03 => {
                    let mut len = [0u8; 1];
                    stream.read_exact(&mut len).unwrap();
                    let mut domain = vec![0u8; len[0] as usize];
                    stream.read_exact(&mut domain).unwrap();
                    String::from_utf8(domain).unwrap()
                }
                0x04 => {
                    let mut octets = [0u8; 16];
                    stream.read_exact(&mut octets).unwrap();
                    std::net::Ipv6Addr::from(octets).to_string()
                }
                other => panic!("unexpected ATYP {:#x}", other),
            };

            // Read port
            let mut port_bytes = [0u8; 2];
            stream.read_exact(&mut port_bytes).unwrap();
            let port = ((port_bytes[0] as u16) << 8) | port_bytes[1] as u16;

            // 4. Send reply (IPv4 bound address 0.0.0.0:0)
            stream
                .write_all(&[
                    0x05,
                    reply_status,
                    0x00,
                    0x01,
                    0x00,
                    0x00,
                    0x00,
                    0x00, // IPv4 0.0.0.0
                    0x00,
                    0x00, // port 0
                ])
                .unwrap();
            stream.flush().unwrap();

            (host, port, atyp)
        }

        #[test]
        fn handshake_success() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || mock_socks5_server(&listener, 0x00));

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.socks5_handshake_sync(&mut stream, "example.com", 443);
            assert!(result.is_ok());

            let (host, port, atyp) = server.join().unwrap();
            assert_eq!(host, "example.com");
            assert_eq!(port, 443);
            assert_eq!(atyp, 0x03, "domain destination should use ATYP 0x03");
        }

        #[test]
        fn handshake_onion_domain() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || mock_socks5_server(&listener, 0x00));

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let onion = "mempoolhqx4isw62xs7abwphsq7ldayuidyx2v2oethdhhj6mlo2r6ad.onion";
            let result = proxy.socks5_handshake_sync(&mut stream, onion, 9735);
            assert!(result.is_ok());

            let (host, port, atyp) = server.join().unwrap();
            assert_eq!(host, onion);
            assert_eq!(port, 9735);
            assert_eq!(atyp, 0x03, ".onion destinations must remain ATYP 0x03");
        }

        #[test]
        fn handshake_server_rejects() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || {
                mock_socks5_server(&listener, 0x05) // connection refused
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.socks5_handshake_sync(&mut stream, "blocked.com", 80);
            assert!(result.is_err());

            server.join().unwrap();
        }

        #[test]
        fn handshake_port_encoding() {
            // Test that port bytes are correctly big-endian
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || mock_socks5_server(&listener, 0x00));

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            // Port 9735 = 0x2607 (tests both bytes matter)
            proxy.socks5_handshake_sync(&mut stream, "test.com", 9735).unwrap();

            let (_, port, _) = server.join().unwrap();
            assert_eq!(port, 9735);
        }

        #[test]
        fn handshake_domain_too_long() {
            // Domain >255 bytes should fail during the connect request phase.
            // We need a mock server to handle the initial greeting handshake.
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                // Read greeting
                let mut greeting = [0u8; 3];
                stream.read_exact(&mut greeting).unwrap();
                // Reply: accept no-auth
                stream.write_all(&[0x05, 0x00]).unwrap();
                stream.flush().unwrap();
                // Client should disconnect after domain length check fails
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let long_domain = "a".repeat(256);
            let result = proxy.socks5_handshake_sync(&mut stream, &long_domain, 80);
            assert!(result.is_err());

            let _ = server.join();
        }

        /// Mock SOCKS5 server that expects username/password auth (RFC 1929).
        fn mock_socks5_server_with_auth(
            listener: &TcpListener,
            expected_user: &str,
            expected_pass: &str,
        ) -> (String, u16, bool) {
            let (mut stream, _) = listener.accept().unwrap();

            // 1. Read greeting (should request method 0x02)
            let mut greeting = [0u8; 3];
            stream.read_exact(&mut greeting).unwrap();
            assert_eq!(greeting[0], 0x05);
            assert_eq!(greeting[1], 0x01);
            assert_eq!(greeting[2], 0x02, "should request username/password auth");

            // Accept method 0x02
            stream.write_all(&[0x05, 0x02]).unwrap();
            stream.flush().unwrap();

            // 2. Read RFC 1929 auth request
            let mut ver = [0u8; 1];
            stream.read_exact(&mut ver).unwrap();
            assert_eq!(ver[0], 0x01, "sub-negotiation version");

            let mut ulen = [0u8; 1];
            stream.read_exact(&mut ulen).unwrap();
            let mut user = vec![0u8; ulen[0] as usize];
            stream.read_exact(&mut user).unwrap();

            let mut plen = [0u8; 1];
            stream.read_exact(&mut plen).unwrap();
            let mut pass = vec![0u8; plen[0] as usize];
            stream.read_exact(&mut pass).unwrap();

            let user_str = String::from_utf8(user).unwrap();
            let pass_str = String::from_utf8(pass).unwrap();
            let auth_ok = user_str == expected_user && pass_str == expected_pass;

            // Reply: 0x00 = success, 0x01 = failure
            stream.write_all(&[0x01, if auth_ok { 0x00 } else { 0x01 }]).unwrap();
            stream.flush().unwrap();

            if !auth_ok {
                return (user_str, 0, false);
            }

            // 3. Read connect request
            let mut header = [0u8; 4];
            stream.read_exact(&mut header).unwrap();
            let mut len = [0u8; 1];
            stream.read_exact(&mut len).unwrap();
            let mut domain = vec![0u8; len[0] as usize];
            stream.read_exact(&mut domain).unwrap();
            let host = String::from_utf8(domain).unwrap();
            let mut port_bytes = [0u8; 2];
            stream.read_exact(&mut port_bytes).unwrap();
            let port = ((port_bytes[0] as u16) << 8) | port_bytes[1] as u16;

            // 4. Reply success
            stream
                .write_all(&[0x05, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
                .unwrap();
            stream.flush().unwrap();

            (host, port, true)
        }

        #[test]
        fn handshake_with_credentials() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5_with_credentials(
                format!("127.0.0.1:{}", addr.port()),
                "session-42".to_string(),
                "x".to_string(),
            )
            .unwrap();

            let server = std::thread::spawn(move || {
                mock_socks5_server_with_auth(&listener, "session-42", "x")
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.socks5_handshake_sync(&mut stream, "example.com", 443);
            assert!(result.is_ok());

            let (host, port, auth_ok) = server.join().unwrap();
            assert!(auth_ok);
            assert_eq!(host, "example.com");
            assert_eq!(port, 443);
        }

        #[test]
        fn handshake_credentials_rejected() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5_with_credentials(
                format!("127.0.0.1:{}", addr.port()),
                "wrong-user".to_string(),
                "wrong-pass".to_string(),
            )
            .unwrap();

            let server = std::thread::spawn(move || {
                mock_socks5_server_with_auth(&listener, "right-user", "right-pass")
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.socks5_handshake_sync(&mut stream, "example.com", 443);
            assert!(result.is_err());

            server.join().unwrap();
        }

        #[test]
        fn handshake_no_auth_skips_credentials() {
            // Proxy without credentials should use method 0x00 (no auth)
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || mock_socks5_server(&listener, 0x00));

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.socks5_handshake_sync(&mut stream, "test.com", 80);
            assert!(result.is_ok());

            server.join().unwrap();
        }

        #[test]
        fn handshake_ipv4_literal_uses_atyp_one() {
            // An IPv4 destination must be encoded with ATYP 0x01 and four
            // raw octets, not as ATYP 0x03 "127.0.0.1" domain bytes.
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || mock_socks5_server(&listener, 0x00));

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            proxy.socks5_handshake_sync(&mut stream, "192.0.2.1", 443).unwrap();

            let (host, port, atyp) = server.join().unwrap();
            assert_eq!(atyp, 0x01);
            assert_eq!(host, "192.0.2.1");
            assert_eq!(port, 443);
        }

        #[test]
        fn handshake_ipv6_literal_uses_atyp_four() {
            // An IPv6 destination must be encoded with ATYP 0x04 and 16
            // raw octets.
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || mock_socks5_server(&listener, 0x00));

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            proxy.socks5_handshake_sync(&mut stream, "2001:db8::1", 443).unwrap();

            let (host, port, atyp) = server.join().unwrap();
            assert_eq!(atyp, 0x04);
            assert_eq!(host, "2001:db8::1");
            assert_eq!(port, 443);
        }
    }

    // --- HTTP CONNECT handshake tests (sync, with mock server) ---

    #[cfg(feature = "std")]
    mod http_connect_handshake {
        use std::io::{Read, Write};
        use std::net::TcpListener;

        use super::*;

        /// Mock HTTP CONNECT proxy that accepts one connection, reads the
        /// CONNECT request, and replies with the supplied status line.
        /// Returns the host:port the client requested.
        fn mock_http_connect_server(listener: &TcpListener, reply_status_line: &str) -> String {
            let (mut stream, _) = listener.accept().unwrap();

            // Read request bytes until we see end-of-headers (\r\n\r\n).
            let mut buf = [0u8; 1024];
            let mut received = Vec::new();
            loop {
                let n = stream.read(&mut buf).unwrap();
                if n == 0 {
                    break;
                }
                received.extend_from_slice(&buf[..n]);
                if received.windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
            }

            let request = String::from_utf8_lossy(&received).into_owned();
            let first_line = request.lines().next().unwrap();
            // "CONNECT host:port HTTP/1.1"
            let target = first_line.split_whitespace().nth(1).unwrap().to_string();

            // Reply
            let response = format!("{reply_status_line}\r\n\r\n");
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();

            target
        }

        #[test]
        fn handshake_success_dispatches_via_handshake_sync() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_http(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || {
                mock_http_connect_server(&listener, "HTTP/1.1 200 Connection Established")
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.handshake_sync(&mut stream, "example.com", 443);
            assert!(result.is_ok());

            let target = server.join().unwrap();
            assert_eq!(target, "example.com:443");
        }

        #[test]
        fn handshake_non_200_status_returns_error() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_http(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || {
                mock_http_connect_server(&listener, "HTTP/1.1 502 Bad Gateway")
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.handshake_sync(&mut stream, "example.com", 443);
            assert!(result.is_err());

            server.join().unwrap();
        }

        #[test]
        fn handshake_handles_split_response() {
            // Writes the proxy response in two TCP segments with a small delay,
            // so the client's first `read()` returns before the end of headers.
            // The previous implementation treated `n < buf.len()` as EOF and
            // would parse the truncated first chunk; this test guards against
            // that regression.
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_http(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                let mut buf = [0u8; 1024];
                let mut received = Vec::new();
                loop {
                    let n = stream.read(&mut buf).unwrap();
                    if n == 0 {
                        break;
                    }
                    received.extend_from_slice(&buf[..n]);
                    if received.windows(4).any(|w| w == b"\r\n\r\n") {
                        break;
                    }
                }
                // Truncate the status line so the first chunk on the wire is
                // unusable on its own.
                stream.write_all(b"HTTP/1.1 ").unwrap();
                stream.flush().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(50));
                stream.write_all(b"200 Connection Established\r\n\r\n").unwrap();
                stream.flush().unwrap();
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.handshake_sync(&mut stream, "example.com", 443);
            assert!(result.is_ok(), "handshake failed on split response: {:?}", result);

            server.join().unwrap();
        }
    }
}
