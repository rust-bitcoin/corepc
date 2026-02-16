//! A minimal library for parsing and validating URLs.

use alloc::format;
use alloc::string::{String, ToString as _};
use alloc::vec::Vec;
use core::fmt;
use core::ops::Range;

/// Returns the default port for known schemes, or `None` for unknown schemes.
fn default_port_for_scheme(scheme: &str) -> Option<u16> {
    match scheme {
        "http" | "ws" => Some(80),
        "https" | "wss" => Some(443),
        "ftp" => Some(21),
        _ => None,
    }
}

/// Errors that can occur during URL parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The input string is empty.
    EmptyInput,
    /// The input contains invalid characters (control characters or non-ASCII).
    InvalidCharacter(char),
    /// The URL is missing a scheme.
    MissingScheme,
    /// The URL has an invalid scheme format.
    InvalidScheme,
    /// The URL has an empty host.
    EmptyHost,
    /// The port number is invalid.
    InvalidPort,
    /// The URL has an unknown scheme and no explicit port.
    MissingPort,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "empty input"),
            ParseError::InvalidCharacter(c) => write!(f, "invalid character: {:?}", c),
            ParseError::MissingScheme => write!(f, "missing scheme"),
            ParseError::InvalidScheme => write!(f, "invalid scheme"),
            ParseError::EmptyHost => write!(f, "empty host"),
            ParseError::InvalidPort => write!(f, "invalid port"),
            ParseError::MissingPort => write!(f, "missing port for unknown scheme"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

/// A parsed URL.
///
/// All accessor methods return slices into the original URL string,
/// avoiding any additional string allocations.
///
/// **Note:** This type currently only supports encoded URLs. IDNs or non-ASCII URLs must be
/// properly punycoded/%-encoded prior to parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    /// The full serialized URL string.
    serialization: String,
    /// Range of the scheme in `serialization`.
    scheme: Range<usize>,
    /// Range of the username in `serialization`. Empty range if no username.
    username: Range<usize>,
    /// Range of the password in `serialization`, if present.
    password: Option<Range<usize>>,
    /// Range of the host in `serialization`.
    host: Range<usize>,
    /// The port number, if specified.
    port: Option<u16>,
    /// Range of the path in `serialization`.
    path: Range<usize>,
    /// Range of the query string in `serialization` (excludes leading `?`).
    query: Option<Range<usize>>,
    /// Range of the fragment in `serialization` (excludes leading `#`).
    fragment: Option<Range<usize>>,
}

impl Url {
    /// Parses a URL string and returns a `Url` instance.
    ///
    /// Validates that the input contains only valid non-control ASCII characters.
    pub fn parse(url_str: &str) -> Result<Self, ParseError> {
        let url_str = url_str.trim();
        if url_str.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // Validate: only non-control ASCII characters allowed
        for c in url_str.chars() {
            if !c.is_ascii() || c.is_ascii_control() {
                return Err(ParseError::InvalidCharacter(c));
            }
        }

        // Find scheme and normalize to lowercase
        let scheme_end = url_str.find("://").ok_or(ParseError::MissingScheme)?;
        let mut serialization = url_str.to_string();
        serialization[..scheme_end].make_ascii_lowercase();

        Self::parse_inner(serialization)
    }

    /// Parses the URL structure from an already-validated serialization string.
    ///
    /// This method assumes the input has already been validated for invalid characters
    /// and has a normalized (lowercase) scheme.
    fn parse_inner(serialization: String) -> Result<Self, ParseError> {
        // Find the scheme (everything before "://")
        let scheme_end = serialization.find("://").ok_or(ParseError::MissingScheme)?;

        if scheme_end == 0 {
            return Err(ParseError::InvalidScheme);
        }

        let scheme = &serialization[..scheme_end];

        // Validate scheme: must start with a letter and contain only
        // letters, digits, '+', '-', or '.'
        let mut scheme_chars = scheme.chars();
        let first_char = scheme_chars.next().ok_or(ParseError::InvalidScheme)?;
        if !first_char.is_ascii_alphabetic() {
            return Err(ParseError::InvalidScheme);
        }

        for c in scheme_chars {
            if !c.is_ascii_alphanumeric() && c != '+' && c != '-' && c != '.' {
                return Err(ParseError::InvalidScheme);
            }
        }

        // Parse the rest after "://"
        let after_scheme_pos = scheme_end + 3;
        let after_scheme = &serialization[after_scheme_pos..];

        // Extract the authority (host:port) - everything before '/', '?', or '#'
        let authority_end = after_scheme.find(['/', '?', '#']).unwrap_or(after_scheme.len());

        let authority = &after_scheme[..authority_end];
        let after_authority = &after_scheme[authority_end..];

        // Extract userinfo (username:password@) from authority if present and calculate
        // `host_start` position
        let (userinfo, host_start, host_and_port) = if let Some(at_pos) = authority.rfind('@') {
            (Some(&authority[..at_pos]), after_scheme_pos + at_pos + 1, &authority[at_pos + 1..])
        } else {
            (None, after_scheme_pos, authority)
        };

        // Calculate username and password ranges
        let (username, password) = if let Some(info) = userinfo {
            if let Some(colon_pos) = info.find(':') {
                let username = after_scheme_pos..(after_scheme_pos + colon_pos);
                let password = Some((after_scheme_pos + colon_pos + 1)..(host_start - 1));
                (username, password)
            } else {
                let username = after_scheme_pos..(after_scheme_pos + info.len());
                (username, None)
            }
        } else {
            (after_scheme_pos..after_scheme_pos, None) // Empty range for no username
        };

        // Parse host and optional port from host_and_port
        // Handle IPv6 addresses specially: [ipv6]:port
        let (host_len, port) = if host_and_port.starts_with('[') {
            // IPv6 address - find the closing bracket
            if let Some(bracket_pos) = host_and_port.find(']') {
                let after_bracket = &host_and_port[bracket_pos + 1..];
                if after_bracket.starts_with(':') && after_bracket.len() > 1 {
                    // Has a port after the bracket
                    let potential_port = &after_bracket[1..];
                    if potential_port.chars().all(|c| c.is_ascii_digit()) {
                        let port_num: u16 =
                            potential_port.parse().map_err(|_| ParseError::InvalidPort)?;
                        (bracket_pos + 1, Some(port_num))
                    } else {
                        (host_and_port.len(), None)
                    }
                } else if after_bracket.is_empty() {
                    // Just [ipv6] with no port
                    (host_and_port.len(), None)
                } else {
                    // Invalid: something after ] that isn't :port
                    (host_and_port.len(), None)
                }
            } else {
                // No closing bracket - malformed, but don't fail, just use as-is
                (host_and_port.len(), None)
            }
        } else if let Some(colon_pos) = host_and_port.rfind(':') {
            let potential_port = &host_and_port[colon_pos + 1..];
            // Check if this is actually a port (all digits)
            if !potential_port.is_empty() && potential_port.chars().all(|c| c.is_ascii_digit()) {
                let port_num: u16 = potential_port.parse().map_err(|_| ParseError::InvalidPort)?;
                (colon_pos, Some(port_num))
            } else {
                (host_and_port.len(), None)
            }
        } else {
            (host_and_port.len(), None)
        };

        let host_end = host_start + host_len;

        // Validate that host is not empty
        if host_len == 0 {
            return Err(ParseError::EmptyHost);
        }

        // Calculate path start position (after authority)
        let path_start = after_scheme_pos + authority_end;
        let url_len = serialization.len();

        // Calculate path, query, and fragment ranges
        let (path, query, fragment) = {
            let mut query = None;
            let mut fragment = None;
            let mut path_end = url_len;

            if after_authority.starts_with('/') {
                // Find where path ends (at '?' or '#')
                if let Some(q_pos) = after_authority.find('?') {
                    let query_start = path_start + q_pos;
                    path_end = query_start;
                    // Fragment comes after query
                    if let Some(f_pos) = after_authority[q_pos..].find('#') {
                        let fragment_start = query_start + f_pos;
                        query = Some((query_start + 1)..fragment_start);
                        fragment = Some((fragment_start + 1)..url_len);
                    } else {
                        query = Some((query_start + 1)..url_len);
                    }
                } else if let Some(f_pos) = after_authority.find('#') {
                    let fragment_start = path_start + f_pos;
                    path_end = fragment_start;
                    fragment = Some((fragment_start + 1)..url_len);
                }
            } else {
                // No path, check for query/fragment directly
                if after_authority.starts_with('?') {
                    let query_start = path_start;
                    path_end = query_start;
                    if let Some(f_pos) = after_authority.find('#') {
                        let fragment_start = path_start + f_pos;
                        query = Some((query_start + 1)..fragment_start);
                        fragment = Some((fragment_start + 1)..url_len);
                    } else {
                        query = Some((query_start + 1)..url_len);
                    }
                } else if after_authority.starts_with('#') {
                    let fragment_start = path_start;
                    path_end = fragment_start;
                    fragment = Some((fragment_start + 1)..url_len);
                }
            }

            (path_start..path_end, query, fragment)
        };

        // Validate: unknown schemes require an explicit port
        let scheme_str = &serialization[..scheme_end];
        if port.is_none() && default_port_for_scheme(scheme_str).is_none() {
            return Err(ParseError::MissingPort);
        }

        Ok(Url {
            serialization,
            scheme: 0..scheme_end,
            username,
            password,
            host: host_start..host_end,
            port,
            path,
            query,
            fragment,
        })
    }

    /// Returns the scheme of the URL (e.g., "http", "https").
    pub fn scheme(&self) -> &str { &self.serialization[self.scheme.clone()] }

    /// Returns the username from the URL, if present.
    ///
    /// Returns an empty string if no username was specified.
    /// The returned value is trimmed of leading and trailing whitespace.
    pub fn username(&self) -> &str { self.serialization[self.username.clone()].trim() }

    /// Returns the password from the URL, if present.
    ///
    /// Returns `None` if no password was specified or if the password is empty
    /// after trimming whitespace.
    pub fn password(&self) -> Option<&str> {
        self.password
            .as_ref()
            .map(|r| self.serialization[r.clone()].trim())
            .filter(|s| !s.is_empty())
    }

    /// Returns the base URL (host portion).
    pub fn base_url(&self) -> &str { &self.serialization[self.host.clone()] }

    /// Returns the port number for the URL.
    ///
    /// If a port was explicitly specified in the URL, that port is returned.
    /// Otherwise, the default port for the URL's scheme is returned
    /// (e.g., 80 for `http`, 443 for `https`).
    pub fn port(&self) -> u16 {
        self.port
            .or_else(|| default_port_for_scheme(self.scheme()))
            .expect("Url with unknown scheme must have explicit port")
    }

    /// Returns the path of the URL.
    ///
    /// The path includes the leading `/` if present. Returns an empty string
    /// if no path was specified.
    pub fn path(&self) -> &str {
        let res = &self.serialization[self.path.clone()];
        if res.is_empty() {
            "/"
        } else {
            res
        }
    }

    /// Returns an iterator over the path segments.
    ///
    /// Path segments are the portions between `/` characters. Empty segments
    /// (from leading, trailing, or consecutive slashes) are filtered out.
    pub fn path_segments(&self) -> impl Iterator<Item = &str> {
        let path = self.path();
        let path = if let Some(stripped) = path.strip_prefix('/') { stripped } else { path };
        path.split('/').map(|s| s.trim()).filter(|s| !s.is_empty())
    }

    /// Returns the query string of the URL, if present.
    ///
    /// The returned string does not include the leading `?`.
    pub fn query(&self) -> Option<&str> {
        self.query.as_ref().map(|r| &self.serialization[r.clone()])
    }

    /// Returns an iterator over the query string's key-value pairs.
    ///
    /// Pairs are separated by `&` and keys are separated from values by `=`.
    /// If a pair has no `=`, the value will be an empty string.
    /// Pairs with empty keys (after decoding and trimming) are filtered out.
    ///
    /// Keys and values are percent-decoded according to form-urlencoded rules:
    /// - `%XX` sequences are decoded to the corresponding byte
    /// - `+` is decoded as a space
    pub fn query_pairs(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.query().into_iter().flat_map(|q| {
            q.split('&')
                .map(|pair| {
                    let pair = pair.trim();
                    if let Some(eq_pos) = pair.find('=') {
                        let key = percent_decode_string(pair[..eq_pos].trim());
                        let value = percent_decode_string(pair[eq_pos + 1..].trim());
                        (key, value)
                    } else {
                        (percent_decode_string(pair.trim()), String::new())
                    }
                })
                .filter(|(k, _)| !k.is_empty())
        })
    }

    /// Returns the fragment identifier of the URL, if present.
    ///
    /// The returned string does not include the leading `#`.
    pub fn fragment(&self) -> Option<&str> {
        self.fragment.as_ref().map(|r| &self.serialization[r.clone()])
    }

    /// Returns the serialized URL as a string slice.
    pub fn as_str(&self) -> &str { &self.serialization }

    /// Returns `true` if the URL scheme is "https" or "wss".
    pub(crate) fn is_https(&self) -> bool { matches!(self.scheme(), "https" | "wss") }

    /// Returns `true` if a non-default port was explicitly specified in the URL.
    ///
    /// This is useful for serialization purposes: ports that are the default for
    /// their scheme (e.g., 80 for `http`) are typically omitted from the URL string.
    #[cfg(feature = "std")]
    pub(crate) fn has_explicit_non_default_port(&self) -> bool {
        match self.port {
            Some(port) => Some(port) != default_port_for_scheme(self.scheme()),
            None => false,
        }
    }

    /// Returns the combined path and query string.
    ///
    /// The returned string includes the leading `/` (if present) and the `?`
    /// separator (if there's a query string). Returns "/" if the path is empty.
    pub(crate) fn path_and_query(&self) -> String {
        let path = self.path();
        let path = if path.is_empty() { "/" } else { path };

        match self.query() {
            Some(query) => format!("{}?{}", path, query),
            None => path.to_string(),
        }
    }

    /// Appends query parameters from an iterator to the URL.
    ///
    /// Keys and values are percent-encoded before being appended.
    /// If the URL already has a query string, parameters are appended with `&`.
    /// Otherwise, the first parameter is appended with `?`.
    ///
    /// Only calls `parse_inner` once after all parameters have been appended.
    pub fn append_query_params<'a>(
        &mut self,
        params: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) {
        let mut params = params.into_iter().peekable();
        if params.peek().is_none() {
            return;
        }

        // Extract fragment string and compute base (everything before '#')
        let (mut new_serialization, fragment) = if let Some(ref frag_range) = self.fragment {
            let frag = self.serialization[frag_range.clone()].to_string();
            let base = self.serialization[..frag_range.start - 1].to_string(); // -1 for '#'
            (base, Some(frag))
        } else {
            (self.serialization.clone(), None)
        };

        let mut has_query = self.query.is_some();

        for (key, value) in params {
            let encoded_key = percent_encode_string(key);
            let encoded_value = percent_encode_string(value);
            if has_query {
                new_serialization.push('&');
            } else {
                new_serialization.push('?');
                has_query = true;
            }
            new_serialization.push_str(&encoded_key);
            new_serialization.push('=');
            new_serialization.push_str(&encoded_value);
        }

        if let Some(frag) = fragment {
            new_serialization.push('#');
            new_serialization.push_str(&frag);
        }

        // Reparse to update all fields
        *self =
            Self::parse_inner(new_serialization).expect("append_query_params produced invalid URL");
    }

    /// If this URL has no fragment but `other` does, copies the fragment from `other`.
    ///
    /// This implements RFC 7231 section 7.1.2 behavior for preserving fragments
    /// across redirects.
    pub fn preserve_fragment_from(&mut self, other: &Url) {
        if self.fragment.is_some() {
            return;
        }

        if let Some(other_frag) = other.fragment() {
            let new_serialization = format!("{}#{}", &self.serialization, other_frag);
            *self = Self::parse_inner(new_serialization)
                .expect("preserve_fragment_from produced invalid URL");
        }
    }

    /// Writes the `scheme "://" host [ ":" port ]` part to the destination.
    #[cfg(feature = "std")]
    pub(crate) fn write_base_url_to<W: std::fmt::Write>(&self, dst: &mut W) -> std::fmt::Result {
        write!(dst, "{}://{}", self.scheme(), self.base_url())?;
        if self.has_explicit_non_default_port() {
            write!(dst, ":{}", self.port())?;
        }
        Ok(())
    }

    /// Writes the `path [ "?" query ] [ "#" fragment ]` part to the destination.
    #[cfg(feature = "std")]
    pub(crate) fn write_resource_to<W: std::fmt::Write>(&self, dst: &mut W) -> std::fmt::Result {
        let path = self.path();
        let path = if path.is_empty() { "/" } else { path };
        write!(dst, "{}", path)?;

        if let Some(query) = self.query() {
            write!(dst, "?{}", query)?;
        }

        if let Some(fragment) = self.fragment() {
            write!(dst, "#{}", fragment)?;
        }

        Ok(())
    }
}

impl core::fmt::Display for Url {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Returns the `%HH` triplet representing `byte` for percent encoding.
fn percent_encoded_triplet(byte: u8) -> [char; 3] {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    ['%', HEX[(byte >> 4) as usize] as char, HEX[(byte & 0x0F) as usize] as char]
}

/// Percent-encodes a char and appends it to `result`.
/// Unreserved characters (0-9, A-Z, a-z, -, ., _, ~) are not encoded.
fn percent_encode_char(c: char, result: &mut String) {
    match c {
        // All URL-'safe' characters are not encoded
        '0'..='9' | 'A'..='Z' | 'a'..='z' | '-' | '.' | '_' | '~' => {
            result.push(c);
        }
        _ => {
            // Any UTF-8 character can fit in 4 bytes
            let mut utf8_buf = [0u8; 4];
            c.encode_utf8(&mut utf8_buf).as_bytes().iter().for_each(|byte| {
                for ch in percent_encoded_triplet(*byte) {
                    result.push(ch);
                }
            });
        }
    }
}

/// Percent-encodes the entire input string and returns the encoded version.
fn percent_encode_string(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    for ch in input.chars() {
        percent_encode_char(ch, &mut encoded);
    }
    encoded
}

/// Decodes a percent-encoded string according to form-urlencoded rules.
///
/// - `%XX` sequences are decoded to the corresponding byte
/// - `+` is decoded as a space (form-urlencoded convention)
/// - Invalid percent sequences (e.g., `%ZZ`, `%4`) are preserved as-is
/// - UTF-8 sequences are properly decoded
fn percent_decode_string(input: &str) -> String {
    let mut result = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                // In form-urlencoded, '+' represents space
                result.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                // Try to decode %XX
                let hex1 = bytes[i + 1];
                let hex2 = bytes[i + 2];
                if let (Some(d1), Some(d2)) = (hex_digit_value(hex1), hex_digit_value(hex2)) {
                    result.push((d1 << 4) | d2);
                    i += 3;
                } else {
                    // Invalid hex digits, preserve '%' as-is
                    result.push(b'%');
                    i += 1;
                }
            }
            b'%' => {
                // Incomplete sequence at end of string, preserve as-is
                result.push(b'%');
                i += 1;
            }
            byte => {
                result.push(byte);
                i += 1;
            }
        }
    }

    // Convert to String, replacing invalid UTF-8 with replacement character
    String::from_utf8_lossy(&result).into_owned()
}

/// Returns the numeric value of a hex digit (0-15), or None if not a valid hex digit.
fn hex_digit_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_url() {
        let url = Url::parse("http://example.com").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.port(), 80);
    }

    #[test]
    fn parse_url_with_port() {
        let url = Url::parse("https://example.com:8080").unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.port(), 8080);
    }

    #[test]
    fn parse_url_with_path() {
        let url = Url::parse("http://example.com/path/to/resource").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.port(), 80);
    }

    #[test]
    fn empty_input_returns_error() {
        assert_eq!(Url::parse(""), Err(ParseError::EmptyInput));
    }

    #[test]
    fn missing_scheme_returns_error() {
        assert_eq!(Url::parse("example.com"), Err(ParseError::MissingScheme));
    }

    #[test]
    fn invalid_character_returns_error() {
        // Control character
        assert!(matches!(
            Url::parse("http://example\x00.com"),
            Err(ParseError::InvalidCharacter('\x00'))
        ));

        // Non-ASCII character
        assert!(matches!(Url::parse("http://exämple.com"), Err(ParseError::InvalidCharacter('ä'))));
    }

    #[test]
    fn scheme_is_lowercased() {
        let url = Url::parse("HTTP://EXAMPLE.COM").unwrap();
        assert_eq!(url.scheme(), "http");
    }

    #[test]
    fn path_returns_full_path() {
        let url = Url::parse("http://example.com/path/to/resource").unwrap();
        assert_eq!(url.path(), "/path/to/resource");
    }

    #[test]
    fn path_is_empty_when_not_specified() {
        let url = Url::parse("http://example.com").unwrap();
        assert_eq!(url.path(), "/");
    }

    #[test]
    fn path_segments_splits_correctly() {
        let url = Url::parse("http://example.com/path/to/resource").unwrap();
        let segments: Vec<&str> = url.path_segments().collect();
        assert_eq!(segments, vec!["path", "to", "resource"]);
    }

    #[test]
    fn path_segments_handles_empty_path() {
        let url = Url::parse("http://example.com").unwrap();
        let segments: Vec<&str> = url.path_segments().collect();
        assert!(segments.is_empty());
    }

    #[test]
    fn path_stops_at_query_string() {
        let url = Url::parse("http://example.com/path?query=value").unwrap();
        assert_eq!(url.path(), "/path");
    }

    #[test]
    fn path_stops_at_fragment() {
        let url = Url::parse("http://example.com/path#section").unwrap();
        assert_eq!(url.path(), "/path");
    }

    #[test]
    fn query_returns_query_string() {
        let url = Url::parse("http://example.com/path?foo=bar&baz=qux").unwrap();
        assert_eq!(url.query(), Some("foo=bar&baz=qux"));
    }

    #[test]
    fn query_is_none_when_not_present() {
        let url = Url::parse("http://example.com/path").unwrap();
        assert_eq!(url.query(), None);
    }

    #[test]
    fn query_stops_at_fragment() {
        let url = Url::parse("http://example.com/path?query=value#section").unwrap();
        assert_eq!(url.query(), Some("query=value"));
    }

    #[test]
    fn query_pairs_parses_key_value_pairs() {
        let url = Url::parse("http://example.com?foo=bar&baz=qux").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert_eq!(
            pairs,
            vec![("foo".to_string(), "bar".to_string()), ("baz".to_string(), "qux".to_string())]
        );
    }

    #[test]
    fn query_pairs_handles_missing_value() {
        let url = Url::parse("http://example.com?foo&bar=baz").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert_eq!(
            pairs,
            vec![("foo".to_string(), "".to_string()), ("bar".to_string(), "baz".to_string())]
        );
    }

    #[test]
    fn query_pairs_is_empty_when_no_query() {
        let url = Url::parse("http://example.com").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert!(pairs.is_empty());
    }

    #[test]
    fn fragment_returns_fragment() {
        let url = Url::parse("http://example.com/path#section").unwrap();
        assert_eq!(url.fragment(), Some("section"));
    }

    #[test]
    fn fragment_is_none_when_not_present() {
        let url = Url::parse("http://example.com/path").unwrap();
        assert_eq!(url.fragment(), None);
    }

    #[test]
    fn fragment_with_query() {
        let url = Url::parse("http://example.com/path?query=value#section").unwrap();
        assert_eq!(url.query(), Some("query=value"));
        assert_eq!(url.fragment(), Some("section"));
    }

    #[test]
    fn fragment_without_path_or_query() {
        let url = Url::parse("http://example.com#section").unwrap();
        assert_eq!(url.path(), "/");
        assert_eq!(url.query(), None);
        assert_eq!(url.fragment(), Some("section"));
    }

    #[test]
    fn as_str_returns_full_url() {
        let url = Url::parse("http://example.com/path?query=value#section").unwrap();
        assert_eq!(url.as_str(), "http://example.com/path?query=value#section");
    }

    #[test]
    fn as_str_with_port() {
        let url = Url::parse("https://example.com:8080/path").unwrap();
        assert_eq!(url.as_str(), "https://example.com:8080/path");
    }

    #[test]
    fn as_str_normalizes_scheme_to_lowercase() {
        let url = Url::parse("HTTP://EXAMPLE.COM/path").unwrap();
        assert_eq!(url.as_str(), "http://EXAMPLE.COM/path");
    }

    #[test]
    fn as_str_minimal_url() {
        let url = Url::parse("http://example.com").unwrap();
        assert_eq!(url.as_str(), "http://example.com");
    }

    #[test]
    fn display_matches_as_str() {
        let url = Url::parse("http://example.com/path?query=value#section").unwrap();
        assert_eq!(format!("{}", url), url.as_str());
    }

    #[test]
    fn display_can_be_used_in_format_string() {
        let url = Url::parse("http://example.com").unwrap();
        let formatted = format!("URL: {}", url);
        assert_eq!(formatted, "URL: http://example.com");
    }

    #[test]
    fn ipv6_without_port() {
        let url = Url::parse("http://[::1]/path").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.base_url(), "[::1]");
        assert_eq!(url.port(), 80);
        assert_eq!(url.path(), "/path");
    }

    #[test]
    fn ipv6_with_port() {
        let url = Url::parse("http://[::1]:8080/path").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.base_url(), "[::1]");
        assert_eq!(url.port(), 8080);
        assert_eq!(url.path(), "/path");
    }

    #[test]
    fn ipv6_full_address_with_port() {
        let url = Url::parse("http://[2001:db8::1]:443/").unwrap();
        assert_eq!(url.base_url(), "[2001:db8::1]");
        assert_eq!(url.port(), 443);
    }

    #[test]
    fn ipv6_as_str_roundtrip() {
        let url = Url::parse("http://[::1]:8080/path").unwrap();
        assert_eq!(url.as_str(), "http://[::1]:8080/path");
    }

    #[test]
    fn userinfo_with_username_only() {
        let url = Url::parse("http://user@example.com/path").unwrap();
        assert_eq!(url.username(), "user");
        assert_eq!(url.password(), None);
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.path(), "/path");
    }

    #[test]
    fn userinfo_with_username_and_password() {
        let url = Url::parse("http://user:pass@example.com/path").unwrap();
        assert_eq!(url.username(), "user");
        assert_eq!(url.password(), Some("pass"));
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.path(), "/path");
    }

    #[test]
    fn userinfo_with_port() {
        let url = Url::parse("http://user:pass@example.com:8080/path").unwrap();
        assert_eq!(url.username(), "user");
        assert_eq!(url.password(), Some("pass"));
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.port(), 8080);
    }

    #[test]
    fn userinfo_empty_when_not_present() {
        let url = Url::parse("http://example.com/path").unwrap();
        assert_eq!(url.username(), "");
        assert_eq!(url.password(), None);
    }

    #[test]
    fn userinfo_as_str_roundtrip() {
        let url = Url::parse("http://user:pass@example.com:8080/path").unwrap();
        assert_eq!(url.as_str(), "http://user:pass@example.com:8080/path");
    }

    #[test]
    fn userinfo_with_empty_password() {
        // Empty password after trimming returns None
        let url = Url::parse("http://user:@example.com").unwrap();
        assert_eq!(url.username(), "user");
        assert_eq!(url.password(), None);
        assert_eq!(url.base_url(), "example.com");
    }

    #[test]
    fn parse_error_display() {
        assert_eq!(ParseError::EmptyInput.to_string(), "empty input");
        assert_eq!(ParseError::InvalidCharacter('\x00').to_string(), "invalid character: '\\0'");
        assert_eq!(ParseError::MissingScheme.to_string(), "missing scheme");
        assert_eq!(ParseError::InvalidScheme.to_string(), "invalid scheme");
        assert_eq!(ParseError::EmptyHost.to_string(), "empty host");
        assert_eq!(ParseError::InvalidPort.to_string(), "invalid port");
    }

    #[test]
    fn empty_host_returns_error() {
        assert_eq!(Url::parse("http:///path"), Err(ParseError::EmptyHost));
        assert_eq!(Url::parse("http://:8080/path"), Err(ParseError::EmptyHost));
        assert_eq!(Url::parse("http://user@/path"), Err(ParseError::EmptyHost));
    }

    #[test]
    fn unknown_scheme_without_port_returns_error() {
        // Unknown schemes require an explicit port
        assert_eq!(Url::parse("foo://example.com"), Err(ParseError::MissingPort));
        assert_eq!(Url::parse("custom://host/path"), Err(ParseError::MissingPort));
        // But with explicit port, it works
        assert!(Url::parse("foo://example.com:8080").is_ok());
        assert!(Url::parse("custom://host:1234/path").is_ok());
    }

    #[test]
    #[cfg(feature = "std")]
    fn parse_error_is_std_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}
        assert_error(&ParseError::EmptyInput);
    }

    #[test]
    fn percent_encode_unreserved_chars_unchanged() {
        // RFC 3986 unreserved characters should not be encoded
        assert_eq!(percent_encode_string("abc"), "abc");
        assert_eq!(percent_encode_string("ABC"), "ABC");
        assert_eq!(percent_encode_string("0123456789"), "0123456789");
        assert_eq!(percent_encode_string("-._~"), "-._~");
    }

    #[test]
    fn percent_encode_reserved_chars() {
        // Reserved characters should be encoded
        assert_eq!(percent_encode_string(" "), "%20");
        assert_eq!(percent_encode_string("!"), "%21");
        assert_eq!(percent_encode_string("#"), "%23");
        assert_eq!(percent_encode_string("$"), "%24");
        assert_eq!(percent_encode_string("&"), "%26");
        assert_eq!(percent_encode_string("'"), "%27");
        assert_eq!(percent_encode_string("("), "%28");
        assert_eq!(percent_encode_string(")"), "%29");
        assert_eq!(percent_encode_string("*"), "%2A");
        assert_eq!(percent_encode_string("+"), "%2B");
        assert_eq!(percent_encode_string(","), "%2C");
        assert_eq!(percent_encode_string("/"), "%2F");
        assert_eq!(percent_encode_string(":"), "%3A");
        assert_eq!(percent_encode_string(";"), "%3B");
        assert_eq!(percent_encode_string("="), "%3D");
        assert_eq!(percent_encode_string("?"), "%3F");
        assert_eq!(percent_encode_string("@"), "%40");
        assert_eq!(percent_encode_string("["), "%5B");
        assert_eq!(percent_encode_string("]"), "%5D");
    }

    #[test]
    fn percent_encode_unicode() {
        // Unicode characters should be encoded as UTF-8 bytes
        assert_eq!(percent_encode_string("ó"), "%C3%B3");
        assert_eq!(percent_encode_string("ò"), "%C3%B2");
        assert_eq!(percent_encode_string("👀"), "%F0%9F%91%80");
        assert_eq!(percent_encode_string("日本語"), "%E6%97%A5%E6%9C%AC%E8%AA%9E");
    }

    #[test]
    fn percent_encode_mixed_string() {
        assert_eq!(percent_encode_string("hello world"), "hello%20world");
        assert_eq!(percent_encode_string("foo=bar"), "foo%3Dbar");
        assert_eq!(percent_encode_string("what's this? 👀"), "what%27s%20this%3F%20%F0%9F%91%80");
    }

    #[test]
    fn percent_encode_percent_sign() {
        // The percent sign itself must be encoded
        assert_eq!(percent_encode_string("%"), "%25");
        assert_eq!(percent_encode_string("%7B"), "%257B");
    }

    #[test]
    fn append_query_params_to_url_without_query() {
        let mut url = Url::parse("http://example.com/path").unwrap();
        url.append_query_params([("foo", "bar")]);
        assert_eq!(url.query(), Some("foo=bar"));
        assert_eq!(url.as_str(), "http://example.com/path?foo=bar");
    }

    #[test]
    fn append_query_params_to_url_with_existing_query() {
        let mut url = Url::parse("http://example.com/path?existing=value").unwrap();
        url.append_query_params([("foo", "bar")]);
        assert_eq!(url.query(), Some("existing=value&foo=bar"));
        assert_eq!(url.as_str(), "http://example.com/path?existing=value&foo=bar");
    }

    #[test]
    fn append_query_params_encodes_special_chars() {
        let mut url = Url::parse("http://example.com").unwrap();
        url.append_query_params([("key with spaces", "value&special=chars")]);
        assert_eq!(url.query(), Some("key%20with%20spaces=value%26special%3Dchars"));
    }

    #[test]
    fn append_query_params_encodes_unicode() {
        let mut url = Url::parse("http://example.com").unwrap();
        url.append_query_params([("ówò", "what's this? 👀")]);
        assert_eq!(url.query(), Some("%C3%B3w%C3%B2=what%27s%20this%3F%20%F0%9F%91%80"));
    }

    #[test]
    fn append_query_params_preserves_fragment() {
        let mut url = Url::parse("http://example.com/path#section").unwrap();
        url.append_query_params([("foo", "bar")]);
        assert_eq!(url.query(), Some("foo=bar"));
        assert_eq!(url.fragment(), Some("section"));
        assert_eq!(url.as_str(), "http://example.com/path?foo=bar#section");
    }

    #[test]
    fn append_query_params_to_url_with_query_and_fragment() {
        let mut url = Url::parse("http://example.com/path?existing=value#section").unwrap();
        url.append_query_params([("foo", "bar")]);
        assert_eq!(url.query(), Some("existing=value&foo=bar"));
        assert_eq!(url.fragment(), Some("section"));
        assert_eq!(url.as_str(), "http://example.com/path?existing=value&foo=bar#section");
    }

    #[test]
    fn append_query_params_multiple_params() {
        let mut url = Url::parse("http://example.com").unwrap();
        url.append_query_params([("a", "1"), ("b", "2"), ("c", "3")]);
        assert_eq!(url.query(), Some("a=1&b=2&c=3"));
    }

    #[test]
    fn append_query_params_empty_iterator() {
        let mut url = Url::parse("http://example.com/path").unwrap();
        url.append_query_params(std::iter::empty::<(&str, &str)>());
        assert_eq!(url.query(), None);
        assert_eq!(url.as_str(), "http://example.com/path");
    }

    #[test]
    fn no_double_encoding_existing_query_params() {
        // When a URL already has percent-encoded query params,
        // they should NOT be re-encoded when new params are appended.
        // This is the fix for issue #468.
        let mut url = Url::parse("http://example.com/test?query=%7B%22id%22%7D").unwrap();

        // Verify the existing encoded query is preserved as-is
        assert_eq!(url.query(), Some("query=%7B%22id%22%7D"));

        // Add a new param
        url.append_query_params([("foo", "bar")]);

        // The existing encoded query should still be preserved, not double-encoded
        // i.e., %7B should NOT become %257B
        assert_eq!(url.query(), Some("query=%7B%22id%22%7D&foo=bar"));
        assert_eq!(url.as_str(), "http://example.com/test?query=%7B%22id%22%7D&foo=bar");
    }

    #[test]
    fn no_double_encoding_complex_encoded_url() {
        // Test with a more complex encoded URL
        let mut url =
            Url::parse("http://example.com/api?filter=%7B%22name%22%3A%22test%22%7D").unwrap();

        // Original query should be preserved
        assert_eq!(url.query(), Some("filter=%7B%22name%22%3A%22test%22%7D"));

        // Add multiple new params in one call
        url.append_query_params([("page", "1"), ("sort", "name")]);

        // Verify no double encoding occurred
        assert_eq!(url.query(), Some("filter=%7B%22name%22%3A%22test%22%7D&page=1&sort=name"));
    }

    #[test]
    fn parse_trims_leading_and_trailing_whitespace() {
        // Leading whitespace
        let url = Url::parse("  http://example.com").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.as_str(), "http://example.com");

        // Trailing whitespace
        let url = Url::parse("http://example.com  ").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.as_str(), "http://example.com");

        // Both leading and trailing whitespace
        let url = Url::parse("  http://example.com/path?query=value  ").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.base_url(), "example.com");
        assert_eq!(url.path(), "/path");
        assert_eq!(url.query(), Some("query=value"));
        assert_eq!(url.as_str(), "http://example.com/path?query=value");

        // Tabs and newlines
        let url = Url::parse("\t\nhttp://example.com\n\t").unwrap();
        assert_eq!(url.as_str(), "http://example.com");

        // Only whitespace should return EmptyInput error
        assert_eq!(Url::parse("   "), Err(ParseError::EmptyInput));
        assert_eq!(Url::parse("\t\n"), Err(ParseError::EmptyInput));
    }

    #[test]
    fn path_segments_filters_empty_segments() {
        // Consecutive slashes should not produce empty segments
        let url = Url::parse("http://example.com//foo//bar//").unwrap();
        let segments: Vec<&str> = url.path_segments().collect();
        assert_eq!(segments, vec!["foo", "bar"]);

        // Root path only
        let url = Url::parse("http://example.com/").unwrap();
        let segments: Vec<&str> = url.path_segments().collect();
        assert!(segments.is_empty());

        // Multiple slashes only
        let url = Url::parse("http://example.com///").unwrap();
        let segments: Vec<&str> = url.path_segments().collect();
        assert!(segments.is_empty());
    }

    #[test]
    fn query_pairs_filters_empty_keys() {
        // Empty key should be filtered
        let url = Url::parse("http://example.com?=value&foo=bar").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert_eq!(pairs, vec![("foo".to_string(), "bar".to_string())]);

        // Multiple empty keys
        let url = Url::parse("http://example.com?=&=&foo=bar&=").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert_eq!(pairs, vec![("foo".to_string(), "bar".to_string())]);

        // All empty keys results in empty iterator
        let url = Url::parse("http://example.com?=value&=").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert!(pairs.is_empty());
    }

    #[test]
    fn query_pairs_trims_keys_and_values() {
        // Note: In practice, whitespace in URLs should be percent-encoded,
        // but this tests the trimming behavior if somehow present
        let url = Url::parse("http://example.com?foo=bar&baz=qux").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert_eq!(
            pairs,
            vec![("foo".to_string(), "bar".to_string()), ("baz".to_string(), "qux".to_string())]
        );
    }

    #[test]
    fn username_is_trimmed() {
        let url = Url::parse("http://user@example.com").unwrap();
        assert_eq!(url.username(), "user");

        // Empty username returns empty string
        let url = Url::parse("http://example.com").unwrap();
        assert_eq!(url.username(), "");
    }

    #[test]
    fn password_is_trimmed_and_empty_returns_none() {
        let url = Url::parse("http://user:pass@example.com").unwrap();
        assert_eq!(url.password(), Some("pass"));

        // No password returns None
        let url = Url::parse("http://user@example.com").unwrap();
        assert_eq!(url.password(), None);

        // Empty password returns None (filtered out)
        let url = Url::parse("http://user:@example.com").unwrap();
        assert_eq!(url.password(), None);
    }

    #[test]
    fn percent_decode_basic() {
        // Basic percent decoding
        assert_eq!(percent_decode_string("%40"), "@");
        assert_eq!(percent_decode_string("%3D"), "=");
        assert_eq!(percent_decode_string("%26"), "&");
        assert_eq!(percent_decode_string("%20"), " ");
    }

    #[test]
    fn percent_decode_utf8() {
        // UTF-8 sequences
        assert_eq!(percent_decode_string("%C3%B3"), "ó");
        assert_eq!(percent_decode_string("%C3%B2"), "ò");
        assert_eq!(percent_decode_string("%E6%97%A5%E6%9C%AC%E8%AA%9E"), "日本語");
    }

    #[test]
    fn percent_decode_invalid_hex() {
        // Invalid hex digits are preserved as-is
        assert_eq!(percent_decode_string("%ZZ"), "%ZZ");
        assert_eq!(percent_decode_string("%GH"), "%GH");
        assert_eq!(percent_decode_string("foo%ZZbar"), "foo%ZZbar");
    }

    #[test]
    fn percent_decode_incomplete() {
        // Incomplete sequences at end of string are preserved
        assert_eq!(percent_decode_string("%4"), "%4");
        assert_eq!(percent_decode_string("foo%"), "foo%");
        assert_eq!(percent_decode_string("foo%2"), "foo%2");
    }

    #[test]
    fn percent_decode_plus_as_space() {
        // In form-urlencoded, '+' represents space
        assert_eq!(percent_decode_string("hello+world"), "hello world");
        assert_eq!(percent_decode_string("foo+bar+baz"), "foo bar baz");
    }

    #[test]
    fn percent_decode_mixed() {
        // Mix of encoded and plain chars
        assert_eq!(percent_decode_string("hello%20world"), "hello world");
        assert_eq!(percent_decode_string("key%3Dvalue"), "key=value");
        assert_eq!(percent_decode_string("foo%26bar%3Dbaz"), "foo&bar=baz");
    }

    #[test]
    fn percent_decode_lowercase_hex() {
        // Lowercase hex digits should work
        assert_eq!(percent_decode_string("%c3%b3"), "ó");
        assert_eq!(percent_decode_string("%2f"), "/");
    }

    #[test]
    fn percent_decode_empty_string() {
        assert_eq!(percent_decode_string(""), "");
    }

    #[test]
    fn percent_decode_no_encoding() {
        // Plain string without encoding
        assert_eq!(percent_decode_string("hello"), "hello");
        assert_eq!(percent_decode_string("foo123bar"), "foo123bar");
    }

    #[test]
    fn query_pairs_decodes_percent_encoded() {
        // Test that query_pairs decodes percent-encoded values
        let url = Url::parse("http://example.com?key%3D1=value%26more").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert_eq!(pairs, vec![("key=1".to_string(), "value&more".to_string())]);
    }

    #[test]
    fn query_pairs_decodes_plus_as_space() {
        // Test that query_pairs decodes '+' as space
        let url = Url::parse("http://example.com?hello+world=foo+bar").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert_eq!(pairs, vec![("hello world".to_string(), "foo bar".to_string())]);
    }

    #[test]
    fn query_pairs_decodes_utf8() {
        // Test UTF-8 decoding in query pairs
        let url = Url::parse("http://example.com?name=%C3%B3").unwrap();
        let pairs: Vec<(String, String)> = url.query_pairs().collect();
        assert_eq!(pairs, vec![("name".to_string(), "ó".to_string())]);
    }
}
