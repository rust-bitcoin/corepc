// Common test utilities and strategies for property-based testing
// Based on WHATWG URL Standard: https://url.spec.whatwg.org/

#![allow(dead_code)]

use proptest::prelude::*;

/// Strategy for generating valid URL schemes.
/// Per WHATWG spec: scheme starts with ASCII alpha, followed by ASCII alphanumeric, '+', '-', or '.'
/// We generate lowercase schemes to ensure round-trip serialization works.
pub fn scheme_strategy() -> impl Strategy<Value = String> {
    // First char: lowercase letter
    let first_char = prop::sample::select(('a'..='z').collect::<Vec<_>>());
    // Rest: lowercase alphanumeric, '+', '-', '.'
    let rest_chars = prop::collection::vec(
        prop::sample::select(
            ('a'..='z').chain('0'..='9').chain(['+', '-', '.']).collect::<Vec<_>>(),
        ),
        0..8,
    );

    (first_char, rest_chars).prop_map(|(first, rest)| {
        let mut s = String::with_capacity(1 + rest.len());
        s.push(first);
        for c in rest {
            s.push(c);
        }
        s
    })
}

/// Strategy for generating "special" URL schemes as defined by WHATWG.
/// Special schemes (http, https, ws, wss, ftp) have different parsing behavior
/// (e.g., they always have an authority component after ://).
/// Use this for parity testing with the `url` crate.
pub fn special_scheme_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "http".to_string(),
        "https".to_string(),
        "ws".to_string(),
        "wss".to_string(),
        "ftp".to_string(),
    ])
}

/// Strategy for generating valid host/domain names.
/// Simplified: alphanumeric labels separated by dots, no leading/trailing hyphens in labels.
/// Labels must start with a letter to avoid being treated as IP addresses.
/// Avoids "xn--" prefix which triggers IDNA/punycode validation in the url crate.
pub fn host_strategy() -> impl Strategy<Value = String> {
    // A label starts with a letter and contains alphanumeric chars, optionally with hyphens
    // Minimum 2 chars per label to avoid edge cases with single-char hosts
    // Avoid patterns that look like punycode (xn--)
    let label = "[a-wyz][a-z0-9]{1,10}";
    // 1-3 labels separated by dots, generating realistic domain names like "example.com"
    prop::string::string_regex(&format!("{}(\\.{})?(\\.{})?", label, label, label))
        .expect("valid regex")
}

/// Strategy for generating valid ports (u16).
pub fn port_strategy() -> impl Strategy<Value = Option<u16>> { prop::option::of(any::<u16>()) }

/// Strategy for generating valid URL path characters.
/// Path can contain unreserved chars, percent-encoded, sub-delims, ':', '@', '/'
/// Simplified: alphanumeric, '-', '_', '~', '/'
/// Avoids '.' in paths to prevent path normalization differences with url crate.
pub fn path_strategy() -> impl Strategy<Value = String> {
    prop::option::of(prop::string::string_regex("/[a-zA-Z0-9_~/-]{0,50}").expect("valid regex"))
        .prop_map(|opt| opt.unwrap_or_default())
}

/// Strategy for generating valid query strings.
/// Query can contain unreserved, percent-encoded, sub-delims, ':', '@', '/', '?'
/// Simplified: alphanumeric, '-', '.', '_', '~', '=', '&'
pub fn query_strategy() -> impl Strategy<Value = Option<String>> {
    prop::option::of(prop::string::string_regex("[a-zA-Z0-9_.~=&-]{1,30}").expect("valid regex"))
}

/// Strategy for generating valid fragment identifiers.
/// Similar character set to query.
pub fn fragment_strategy() -> impl Strategy<Value = Option<String>> {
    prop::option::of(prop::string::string_regex("[a-zA-Z0-9_.~-]{1,20}").expect("valid regex"))
}

/// Strategy for generating valid userinfo (username and optional password).
/// Returns (username, password) where username may be empty and password is optional.
pub fn userinfo_strategy() -> impl Strategy<Value = (String, Option<String>)> {
    let username =
        prop::option::of(prop::string::string_regex("[a-zA-Z0-9_-]{1,10}").expect("valid regex"))
            .prop_map(|opt| opt.unwrap_or_default());
    let password =
        prop::option::of(prop::string::string_regex("[a-zA-Z0-9_-]{1,10}").expect("valid regex"));
    (username, password)
}

/// A generated valid URL with all its expected components.
#[derive(Debug, Clone)]
pub struct ValidUrl {
    pub url_string: String,
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

/// Returns true if the scheme has a known default port.
fn is_known_scheme(scheme: &str) -> bool {
    matches!(scheme, "http" | "https" | "ws" | "wss" | "ftp")
}

/// Combined strategy that generates a complete valid URL and its expected components.
/// Unknown schemes always have an explicit port (required by Url::parse).
pub fn valid_url_strategy() -> impl Strategy<Value = ValidUrl> {
    (
        scheme_strategy(),
        host_strategy(),
        any::<u16>(),    // Always generate a port value
        port_strategy(), // Whether to include it (for known schemes)
        path_strategy(),
        query_strategy(),
        fragment_strategy(),
    )
        .prop_map(|(scheme, host, port_value, optional_port, path, query, fragment)| {
            // For unknown schemes, always use explicit port; for known schemes, use optional
            let port = if is_known_scheme(&scheme) { optional_port } else { Some(port_value) };

            let mut url_string = format!("{}://{}", scheme, host);

            if let Some(p) = port {
                url_string.push_str(&format!(":{}", p));
            }

            url_string.push_str(&path);

            if let Some(ref q) = query {
                url_string.push('?');
                url_string.push_str(q);
            }

            if let Some(ref f) = fragment {
                url_string.push('#');
                url_string.push_str(f);
            }

            ValidUrl { url_string, scheme, host, port, path, query, fragment }
        })
}

/// Strategy for generating URL strings directly (without component breakdown).
/// Useful for parity tests where we just need the URL string.
pub fn url_string_strategy() -> impl Strategy<Value = String> {
    valid_url_strategy().prop_map(|v| v.url_string)
}

/// Strategy for generating URL strings with "special" schemes (http, https, etc.).
/// These schemes have consistent parsing behavior between bitreq::Url and the url crate.
/// Use this for parity testing.
pub fn special_url_string_strategy() -> impl Strategy<Value = String> {
    (
        special_scheme_strategy(),
        userinfo_strategy(),
        host_strategy(),
        port_strategy(),
        path_strategy(),
        query_strategy(),
        fragment_strategy(),
    )
        .prop_map(|(scheme, (username, password), host, port, path, query, fragment)| {
            let mut url_string = format!("{}://", scheme);

            if !username.is_empty() {
                url_string.push_str(&username);
                if let Some(ref pw) = password {
                    url_string.push(':');
                    url_string.push_str(pw);
                }
                url_string.push('@');
            }

            url_string.push_str(&host);

            if let Some(p) = port {
                url_string.push_str(&format!(":{}", p));
            }

            url_string.push_str(&path);

            if let Some(ref q) = query {
                url_string.push('?');
                url_string.push_str(q);
            }

            if let Some(ref f) = fragment {
                url_string.push('#');
                url_string.push_str(f);
            }

            url_string
        })
}
