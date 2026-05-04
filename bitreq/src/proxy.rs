use base64::engine::general_purpose::STANDARD;
use base64::engine::Engine;

use crate::error::Error;

/// Kind of proxy connection (Basic, Digest, SOCKS5, etc)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum ProxyKind {
    Basic,
    Socks5,
}

/// Proxy configuration. Supports HTTP CONNECT proxies ([`Proxy::new_http`])
/// and SOCKS5 proxies ([`Proxy::new_socks5`]).
///
/// SOCKS5 uses domain-based addressing (RFC 1928 ATYP 0x03), so DNS
/// resolution is performed by the proxy. This enables routing through
/// Tor, including `.onion` addresses.
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
/// Pass `.await` for the async version, nothing for the sync version. The
/// caller's `use` imports decide which `Read`/`Write` trait set is in scope,
/// so the same body resolves to either `std::io` or `tokio::io` calls.
///
/// Macro hygiene means the call site must pass `self`, `stream`, and the
/// target host/port explicitly.
macro_rules! socks5_handshake_body {
    ($self:ident, $stream:ident, $target_host:ident, $target_port:ident; $($maybe_await:tt)*) => {{
        let (greeting, expected_method) = $self.socks5_greeting();
        $stream.write_all(&greeting) $($maybe_await)*.map_err(Error::IoError)?;
        $stream.flush() $($maybe_await)*.map_err(Error::IoError)?;

        let mut greeting_resp = [0u8; 2];
        $stream.read_exact(&mut greeting_resp) $($maybe_await)*.map_err(Error::IoError)?;
        Self::socks5_check_greeting(&greeting_resp, expected_method)?;

        if let Some(auth_req) = $self.socks5_auth_request() {
            $stream.write_all(&auth_req) $($maybe_await)*.map_err(Error::IoError)?;
            $stream.flush() $($maybe_await)*.map_err(Error::IoError)?;

            let mut auth_resp = [0u8; 2];
            $stream.read_exact(&mut auth_resp) $($maybe_await)*.map_err(Error::IoError)?;
            Self::socks5_check_auth(&auth_resp)?;
        }

        let req = Self::socks5_connect_request($target_host, $target_port)?;
        $stream.write_all(&req) $($maybe_await)*.map_err(Error::IoError)?;
        $stream.flush() $($maybe_await)*.map_err(Error::IoError)?;

        let mut connect_resp = [0u8; 4];
        $stream.read_exact(&mut connect_resp) $($maybe_await)*.map_err(Error::IoError)?;
        if connect_resp[0] != 0x05 || connect_resp[1] != 0x00 {
            return Err(Error::ProxyConnect);
        }

        match connect_resp[3] {
            0x01 => { // IPv4: 4 bytes + 2 port
                let mut buf = [0u8; 6];
                $stream.read_exact(&mut buf) $($maybe_await)*.map_err(Error::IoError)?;
            }
            0x03 => { // Domain: 1 len byte + domain + 2 port
                let mut len = [0u8; 1];
                $stream.read_exact(&mut len) $($maybe_await)*.map_err(Error::IoError)?;
                // Domain length is u8, so domain + 2 port bytes is at most 257.
                let mut buf = [0u8; 257];
                let total = len[0] as usize + 2;
                $stream.read_exact(&mut buf[..total]) $($maybe_await)*.map_err(Error::IoError)?;
            }
            0x04 => { // IPv6: 16 bytes + 2 port
                let mut buf = [0u8; 18];
                $stream.read_exact(&mut buf) $($maybe_await)*.map_err(Error::IoError)?;
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
    ($self:ident, $stream:ident, $target_host:ident, $target_port:ident; $($maybe_await:tt)*) => {{
        let request = $self.connect($target_host, $target_port);
        $stream.write_all(request.as_bytes()) $($maybe_await)*.map_err(Error::IoError)?;
        $stream.flush() $($maybe_await)*.map_err(Error::IoError)?;

        const MAX_PROXY_RESPONSE_SIZE: usize = 16 * 1024;
        let mut proxy_response = Vec::new();
        let mut buf = [0u8; 256];

        loop {
            let n = $stream.read(&mut buf) $($maybe_await)*.map_err(Error::IoError)?;
            if n == 0 {
                break;
            }
            proxy_response.extend_from_slice(&buf[..n]);
            if proxy_response.len() > MAX_PROXY_RESPONSE_SIZE {
                return Err(Error::ProxyConnect);
            }
            if n < buf.len() {
                // Partial read indicates end of response.
                break;
            }
        }

        Self::verify_response(&proxy_response)
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
    /// Both `socks5://` and `socks5h://` are accepted and behave identically:
    /// destination hostnames are forwarded to the proxy as RFC 1928 ATYP 0x03
    /// (DOMAIN), so DNS resolution always happens at the proxy. This matches
    /// the privacy expectation of the `socks5h` URL convention and is
    /// required for routing through Tor (including `.onion` addresses).
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
    ///     "127.0.0.1:9050", "session-1", "x"
    /// ).unwrap();
    /// ```
    ///
    pub fn new_socks5_with_credentials<S: AsRef<str>>(
        proxy: S,
        user: &str,
        password: &str,
    ) -> Result<Self, Error> {
        let mut p = Self::new_socks5(proxy)?;
        p.set_credentials(user, password)?;
        Ok(p)
    }

    /// Sets RFC 1929 username/password credentials on this proxy.
    ///
    /// For Tor SOCKS5 proxies, distinct credentials trigger separate circuits.
    /// Mutating credentials on a long-lived `Proxy` lets a caller rotate
    /// between isolated circuits without rebuilding the `Proxy` from a URL.
    ///
    /// Returns [`Error::BadProxy`] if `user` is empty or either field exceeds
    /// 255 bytes (RFC 1929 length limits).
    ///
    /// # Example
    ///
    /// ```
    /// let mut proxy = bitreq::Proxy::new_socks5("127.0.0.1:9050").unwrap();
    /// proxy.set_credentials("session-1", "x").unwrap();
    /// // ... later, for a fresh circuit:
    /// proxy.set_credentials("session-2", "x").unwrap();
    /// ```
    pub fn set_credentials(&mut self, user: &str, password: &str) -> Result<(), Error> {
        // RFC 1929: username and password are each 1-255 bytes
        if user.is_empty() || user.len() > 255 || password.len() > 255 {
            return Err(Error::BadProxy);
        }
        self.user = Some(user.to_string());
        self.password = Some(password.to_string());
        Ok(())
    }

    /// Build the SOCKS5 greeting bytes.
    /// Returns (greeting_bytes, expected_auth_method).
    fn socks5_greeting(&self) -> ([u8; 3], u8) {
        let method = if self.user.is_some() { 0x02 } else { 0x00 };
        ([0x05, 0x01, method], method)
    }

    /// Validate the SOCKS5 greeting response.
    fn socks5_check_greeting(resp: &[u8; 2], expected_method: u8) -> Result<(), Error> {
        if resp[0] != 0x05 || resp[1] != expected_method {
            return Err(Error::ProxyConnect);
        }
        Ok(())
    }

    /// Build the RFC 1929 username/password auth request.
    /// Returns None if no credentials are set.
    fn socks5_auth_request(&self) -> Option<Vec<u8>> {
        let user = self.user.as_ref()?;
        let pass = self.password.as_deref().unwrap_or("");
        let mut req = Vec::with_capacity(3 + user.len() + pass.len());
        req.push(0x01); // sub-negotiation version
        req.push(user.len() as u8);
        req.extend_from_slice(user.as_bytes());
        req.push(pass.len() as u8);
        req.extend_from_slice(pass.as_bytes());
        Some(req)
    }

    /// Validate the RFC 1929 auth response.
    fn socks5_check_auth(resp: &[u8; 2]) -> Result<(), Error> {
        if resp[1] != 0x00 {
            return Err(Error::InvalidProxyCreds);
        }
        Ok(())
    }

    /// Build the SOCKS5 CONNECT request for a domain target.
    fn socks5_connect_request(target_host: &str, target_port: u16) -> Result<Vec<u8>, Error> {
        let host_bytes = target_host.as_bytes();
        if host_bytes.len() > 255 {
            return Err(Error::ProxyConnect);
        }
        let mut req = Vec::with_capacity(7 + host_bytes.len());
        req.extend_from_slice(&[0x05, 0x01, 0x00, 0x03, host_bytes.len() as u8]);
        req.extend_from_slice(host_bytes);
        req.push((target_port >> 8) as u8);
        req.push((target_port & 0xff) as u8);
        Ok(req)
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

    pub(crate) fn verify_response(response: &[u8]) -> Result<(), Error> {
        let response_string = String::from_utf8_lossy(response);
        let top_line = response_string.lines().next().ok_or(Error::ProxyConnect)?;
        let status_code = top_line.split_whitespace().nth(1).ok_or(Error::BadProxy)?;

        match status_code {
            "200" => Ok(()),
            "401" | "407" => Err(Error::InvalidProxyCreds),
            _ => Err(Error::BadProxy),
        }
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
        socks5_handshake_body!(self, stream, target_host, target_port;)
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
        socks5_handshake_body!(self, stream, target_host, target_port; .await)
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
        http_connect_handshake_body!(self, stream, target_host, target_port;)
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
        http_connect_handshake_body!(self, stream, target_host, target_port; .await)
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
        let proxy = Proxy::new_socks5_with_credentials("127.0.0.1:9050", "user1", "pass1").unwrap();
        assert_eq!(proxy.server, "127.0.0.1");
        assert_eq!(proxy.port, 9050);
        assert!(matches!(proxy.kind, ProxyKind::Socks5));
        assert_eq!(proxy.user, Some("user1".to_string()));
        assert_eq!(proxy.password, Some("pass1".to_string()));
    }

    #[test]
    fn socks5_credentials_length_validation() {
        // Empty username rejected
        assert!(Proxy::new_socks5_with_credentials("localhost:9050", "", "pass").is_err());
        // Username >255 bytes rejected
        let long_user = "a".repeat(256);
        assert!(Proxy::new_socks5_with_credentials("localhost:9050", &long_user, "pass").is_err());
        // Password >255 bytes rejected
        let long_pass = "a".repeat(256);
        assert!(Proxy::new_socks5_with_credentials("localhost:9050", "user", &long_pass).is_err());
        // Max length (255) accepted
        let max_user = "a".repeat(255);
        assert!(Proxy::new_socks5_with_credentials("localhost:9050", &max_user, "x").is_ok());
    }

    #[test]
    fn socks5_set_credentials_rotation() {
        let mut proxy = Proxy::new_socks5("127.0.0.1:9050").unwrap();
        assert_eq!(proxy.user, None);
        assert_eq!(proxy.password, None);

        proxy.set_credentials("session-1", "x").unwrap();
        assert_eq!(proxy.user, Some("session-1".to_string()));
        assert_eq!(proxy.password, Some("x".to_string()));

        // Rotating to a new credential pair (Tor circuit isolation).
        proxy.set_credentials("session-2", "y").unwrap();
        assert_eq!(proxy.user, Some("session-2".to_string()));
        assert_eq!(proxy.password, Some("y".to_string()));
    }

    #[test]
    fn socks5_set_credentials_validation() {
        let mut proxy = Proxy::new_socks5("127.0.0.1:9050").unwrap();
        // Empty user rejected.
        assert!(proxy.set_credentials("", "pass").is_err());
        // Over-long fields rejected.
        let long = "a".repeat(256);
        assert!(proxy.set_credentials(&long, "pass").is_err());
        assert!(proxy.set_credentials("user", &long).is_err());
        // Failed calls don't mutate state.
        assert_eq!(proxy.user, None);
        assert_eq!(proxy.password, None);
    }

    // --- SOCKS5 handshake tests (sync, with mock server) ---

    #[cfg(feature = "std")]
    mod socks5_handshake {
        use super::*;
        use std::io::{Read, Write};
        use std::net::TcpListener;

        /// Mock SOCKS5 server that accepts one connection and performs the handshake.
        /// Returns the target host and port that the client requested.
        fn mock_socks5_server(
            listener: &TcpListener,
            reply_status: u8,
        ) -> (String, u16) {
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
            assert_eq!(header[3], 0x03, "domain address type");

            // Read domain
            let mut len = [0u8; 1];
            stream.read_exact(&mut len).unwrap();
            let mut domain = vec![0u8; len[0] as usize];
            stream.read_exact(&mut domain).unwrap();
            let host = String::from_utf8(domain).unwrap();

            // Read port
            let mut port_bytes = [0u8; 2];
            stream.read_exact(&mut port_bytes).unwrap();
            let port = ((port_bytes[0] as u16) << 8) | port_bytes[1] as u16;

            // 4. Send reply (IPv4 bound address 0.0.0.0:0)
            stream.write_all(&[
                0x05, reply_status, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x00,  // IPv4 0.0.0.0
                0x00, 0x00,              // port 0
            ]).unwrap();
            stream.flush().unwrap();

            (host, port)
        }

        #[test]
        fn handshake_success() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || {
                mock_socks5_server(&listener, 0x00)
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.socks5_handshake_sync(&mut stream, "example.com", 443);
            assert!(result.is_ok());

            let (host, port) = server.join().unwrap();
            assert_eq!(host, "example.com");
            assert_eq!(port, 443);
        }

        #[test]
        fn handshake_onion_domain() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5(format!("127.0.0.1:{}", addr.port())).unwrap();

            let server = std::thread::spawn(move || {
                mock_socks5_server(&listener, 0x00)
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let onion = "mempoolhqx4isw62xs7abwphsq7ldayuidyx2v2oethdhhj6mlo2r6ad.onion";
            let result = proxy.socks5_handshake_sync(&mut stream, onion, 9735);
            assert!(result.is_ok());

            let (host, port) = server.join().unwrap();
            assert_eq!(host, onion);
            assert_eq!(port, 9735);
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

            let server = std::thread::spawn(move || {
                mock_socks5_server(&listener, 0x00)
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            // Port 9735 = 0x2607 (tests both bytes matter)
            proxy.socks5_handshake_sync(&mut stream, "test.com", 9735).unwrap();

            let (_, port) = server.join().unwrap();
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
            stream.write_all(&[
                0x05, 0x00, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ]).unwrap();
            stream.flush().unwrap();

            (host, port, true)
        }

        #[test]
        fn handshake_with_credentials() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let proxy = Proxy::new_socks5_with_credentials(
                format!("127.0.0.1:{}", addr.port()), "session-42", "x"
            ).unwrap();

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
                format!("127.0.0.1:{}", addr.port()), "wrong-user", "wrong-pass"
            ).unwrap();

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

            let server = std::thread::spawn(move || {
                mock_socks5_server(&listener, 0x00)
            });

            let mut stream = std::net::TcpStream::connect(addr).unwrap();
            let result = proxy.socks5_handshake_sync(&mut stream, "test.com", 80);
            assert!(result.is_ok());

            server.join().unwrap();
        }
    }

    // --- HTTP CONNECT handshake tests (sync, with mock server) ---

    #[cfg(feature = "std")]
    mod http_connect_handshake {
        use super::*;
        use std::io::{Read, Write};
        use std::net::TcpListener;

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
    }
}
