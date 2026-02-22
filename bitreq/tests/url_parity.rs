// Parity tests comparing bitreq::Url with url::Url (MaxUrl)
// Ensures our implementation matches the behavior of the reference url crate
// for all public API methods that exist on both types.
//
// Note: We use "special" schemes (http, https, ws, wss, ftp) for parity testing
// because the url crate treats non-special schemes differently (as opaque paths).

mod common;

use bitreq::Url as BitreqUrl;
use common::special_url_string_strategy;
use proptest::prelude::*;
use url::Url as MaxUrl;

proptest! {
    /// Test that scheme() returns the same value for both implementations.
    #[test]
    fn scheme_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        prop_assert_eq!(
            bitreq_url.scheme(),
            max_url.scheme(),
            "scheme() mismatch for URL: {}",
            url_string
        );
    }

    /// Test that port() returns expected values for both implementations.
    ///
    /// Note: The APIs differ:
    /// - `url::Url::port()` returns `Option<u16>`, where `None` means the default port for the scheme
    /// - `bitreq::Url::port()` returns `u16`, always returning the effective port (explicit or default)
    ///
    /// This test verifies that when `url::Url::port()` returns `Some(p)`, `bitreq::Url::port()`
    /// also returns `p`, and when `url::Url::port()` returns `None`, `bitreq::Url::port()`
    /// returns the expected default port for the scheme.
    #[test]
    fn port_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        // Both implementations should agree on the effective port
        prop_assert_eq!(
            bitreq_url.port(),
            max_url.port_or_known_default().expect("special schemes should have known default ports"),
            "port() mismatch for URL: {} (bitreq: {}, url port_or_known_default: {:?})",
            url_string,
            bitreq_url.port(),
            max_url.port_or_known_default()
        );

        // When url::Url::port() returns Some, it should match bitreq::Url::port()
        if let Some(explicit_port) = max_url.port() {
            prop_assert_eq!(
                bitreq_url.port(),
                explicit_port,
                "port() mismatch for URL with explicit port: {}",
                url_string
            );
        }
    }

    /// Test that username() returns the same value for both implementations.
    #[test]
    fn username_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        prop_assert_eq!(
            bitreq_url.username(),
            max_url.username(),
            "username() mismatch for URL: {}",
            url_string
        );
    }

    /// Test that password() returns consistent values for both implementations.
    /// Note: bitreq filters out empty passwords (returns None), while url crate
    /// returns Some(""). We compare the non-empty case for parity.
    #[test]
    fn password_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        let bitreq_password = bitreq_url.password();
        let max_password = max_url.password();

        // bitreq filters out empty passwords, url crate returns Some("")
        // Compare filtered versions for parity
        let max_password_filtered = max_password.filter(|s| !s.is_empty());

        prop_assert_eq!(
            bitreq_password,
            max_password_filtered,
            "password() mismatch for URL: {}",
            url_string
        );
    }

    /// Test that path() returns the same value for both implementations.
    #[test]
    fn path_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        let bitreq_path = bitreq_url.path();
        let max_path = max_url.path();

        prop_assert_eq!(
            bitreq_path,
            max_path,
            "path() mismatch for URL: {} (bitreq: '{}', url: '{}')",
            url_string,
            bitreq_path,
            max_path
        );
    }

    /// Test that query() returns the same value for both implementations.
    #[test]
    fn query_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        prop_assert_eq!(
            bitreq_url.query(),
            max_url.query(),
            "query() mismatch for URL: {}",
            url_string
        );
    }

    /// Test that fragment() returns the same value for both implementations.
    #[test]
    fn fragment_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        prop_assert_eq!(
            bitreq_url.fragment(),
            max_url.fragment(),
            "fragment() mismatch for URL: {}",
            url_string
        );
    }

    /// Test that host (base_url vs host_str) returns the same value.
    /// bitreq::Url::base_url() corresponds to url::Url::host_str().
    #[test]
    fn host_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        prop_assert_eq!(
            Some(bitreq_url.base_url()),
            max_url.host_str(),
            "host mismatch for URL: {} (bitreq base_url: '{}', url host_str: {:?})",
            url_string,
            bitreq_url.base_url(),
            max_url.host_str()
        );
    }

    /// Test that as_str() returns equivalent URLs.
    #[test]
    fn as_str_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        // Both should produce parseable URLs that round-trip correctly
        let bitreq_str = bitreq_url.as_str();
        let max_str = max_url.as_str();

        // Re-parse to ensure both produce valid URLs
        let bitreq_reparsed = BitreqUrl::parse(bitreq_str);
        let max_reparsed = MaxUrl::parse(max_str);

        prop_assert!(
            bitreq_reparsed.is_ok(),
            "bitreq as_str() produced unparseable URL: {}",
            bitreq_str
        );
        prop_assert!(
            max_reparsed.is_ok(),
            "url crate as_str() produced unparseable URL: {}",
            max_str
        );

        // The serialized forms should be semantically equivalent
        let bitreq_reparsed = bitreq_reparsed.unwrap();
        let max_reparsed = max_reparsed.unwrap();

        prop_assert_eq!(
            bitreq_reparsed.scheme(),
            max_reparsed.scheme(),
            "Reparsed scheme mismatch"
        );
        prop_assert_eq!(
            bitreq_reparsed.port(),
            max_reparsed.port_or_known_default().expect("special schemes should have known default ports"),
            "Reparsed port mismatch"
        );
        prop_assert_eq!(
            bitreq_reparsed.path(),
            max_reparsed.path(),
            "Reparsed path mismatch"
        );
        prop_assert_eq!(
            bitreq_reparsed.query(),
            max_reparsed.query(),
            "Reparsed query mismatch"
        );
        prop_assert_eq!(
            bitreq_reparsed.fragment(),
            max_reparsed.fragment(),
            "Reparsed fragment mismatch"
        );
    }

    /// Test that path_segments() returns consistent segments for both implementations.
    /// Note: url crate's path_segments() returns None for cannot-be-a-base URLs,
    /// but our generated URLs always have authority so this shouldn't happen.
    /// Note: bitreq filters out empty segments while the url crate includes them,
    /// so we compare the filtered versions.
    #[test]
    fn path_segments_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        let bitreq_segments: Vec<&str> = bitreq_url.path_segments().collect();
        let max_segments: Option<Vec<&str>> = max_url.path_segments().map(|s| s.collect());

        // url crate should always return Some for URLs with authority
        prop_assert!(
            max_segments.is_some(),
            "url crate returned None for path_segments on URL with authority: {}",
            url_string
        );

        let max_segments = max_segments.unwrap();

        // bitreq filters out empty segments, url crate includes them
        // Compare the filtered versions for parity
        let max_segments_filtered: Vec<&str> = max_segments.into_iter().filter(|s| !s.is_empty()).collect();

        prop_assert_eq!(
            bitreq_segments,
            max_segments_filtered,
            "path_segments() mismatch for URL: {}",
            url_string
        );
    }

    /// Test that Display output matches as_str() for both implementations.
    #[test]
    fn display_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        // Both should have Display == as_str()
        prop_assert_eq!(
            format!("{}", bitreq_url),
            bitreq_url.as_str(),
            "bitreq Display doesn't match as_str()"
        );
        prop_assert_eq!(
            format!("{}", max_url),
            max_url.as_str(),
            "url crate Display doesn't match as_str()"
        );
    }
}

// Test that both implementations accept or reject the same URLs
proptest! {
    /// Test that valid URLs are accepted by both implementations.
    #[test]
    fn both_accept_valid_urls(url_string in special_url_string_strategy()) {
        let bitreq_result = BitreqUrl::parse(&url_string);
        let max_result = MaxUrl::parse(&url_string);

        prop_assert!(
            bitreq_result.is_ok(),
            "bitreq rejected valid URL: {} - {:?}",
            url_string,
            bitreq_result.err()
        );
        prop_assert!(
            max_result.is_ok(),
            "url crate rejected valid URL: {} - {:?}",
            url_string,
            max_result.err()
        );
    }
}

// Parity tests for empty and edge cases
#[cfg(test)]
mod empty_and_edge_cases {
    use super::*;

    #[test]
    fn path_empty_parity() {
        let bitreq = BitreqUrl::parse("http://example.com").unwrap();
        let max = MaxUrl::parse("http://example.com").unwrap();

        // url crate normalizes empty path to "/"
        // bitreq returns "" for empty path, which we normalize in parity tests
        let bitreq_path = bitreq.path();
        let bitreq_normalized = if bitreq_path.is_empty() { "/" } else { bitreq_path };
        assert_eq!(bitreq_normalized, max.path());
    }

    #[test]
    fn path_root_only_parity() {
        let bitreq = BitreqUrl::parse("http://example.com/").unwrap();
        let max = MaxUrl::parse("http://example.com/").unwrap();

        assert_eq!(bitreq.path(), max.path());
    }

    #[test]
    fn path_segments_empty_parity() {
        let bitreq = BitreqUrl::parse("http://example.com").unwrap();
        let max = MaxUrl::parse("http://example.com").unwrap();

        let bitreq_segments: Vec<&str> = bitreq.path_segments().collect();
        let max_segments: Vec<&str> =
            max.path_segments().unwrap().filter(|s| !s.is_empty()).collect();

        // Both should return empty after filtering empty segments
        assert_eq!(bitreq_segments, max_segments);
    }

    #[test]
    fn path_segments_root_only_parity() {
        let bitreq = BitreqUrl::parse("http://example.com/").unwrap();
        let max = MaxUrl::parse("http://example.com/").unwrap();

        let bitreq_segments: Vec<&str> = bitreq.path_segments().collect();
        let max_segments: Vec<&str> =
            max.path_segments().unwrap().filter(|s| !s.is_empty()).collect();

        assert_eq!(bitreq_segments, max_segments);
    }

    #[test]
    fn path_segments_consecutive_slashes_parity() {
        let bitreq = BitreqUrl::parse("http://example.com//foo//bar//").unwrap();
        let max = MaxUrl::parse("http://example.com//foo//bar//").unwrap();

        let bitreq_segments: Vec<&str> = bitreq.path_segments().collect();
        let max_segments: Vec<&str> =
            max.path_segments().unwrap().filter(|s| !s.is_empty()).collect();

        assert_eq!(bitreq_segments, max_segments);
        assert_eq!(bitreq_segments, vec!["foo", "bar"]);
    }

    #[test]
    fn query_empty_parity() {
        // No query at all
        let bitreq = BitreqUrl::parse("http://example.com").unwrap();
        let max = MaxUrl::parse("http://example.com").unwrap();

        assert_eq!(bitreq.query(), max.query());
        assert_eq!(bitreq.query(), None);
    }

    #[test]
    fn query_empty_string_parity() {
        // Query with just "?" - url crate returns Some("")
        let bitreq = BitreqUrl::parse("http://example.com?").unwrap();
        let max = MaxUrl::parse("http://example.com?").unwrap();

        assert_eq!(bitreq.query(), max.query());
        assert_eq!(bitreq.query(), Some(""));
    }

    #[test]
    fn query_pairs_empty_parity() {
        // No query
        let bitreq = BitreqUrl::parse("http://example.com").unwrap();
        let max = MaxUrl::parse("http://example.com").unwrap();

        let bitreq_pairs: Vec<_> = bitreq.query_pairs().collect();
        let max_pairs: Vec<_> = max.query_pairs().into_iter().collect();

        assert!(bitreq_pairs.is_empty());
        assert!(max_pairs.is_empty());
    }

    #[test]
    fn query_pairs_empty_key_filtered_parity() {
        // Query with empty key "?=value" - bitreq filters these out
        let bitreq = BitreqUrl::parse("http://example.com?=value&foo=bar").unwrap();
        let max = MaxUrl::parse("http://example.com?=value&foo=bar").unwrap();

        let bitreq_pairs: Vec<(String, String)> = bitreq.query_pairs().collect();
        // url crate returns Cow<str> pairs, filter out empty keys for parity
        let max_pairs: Vec<(String, String)> = max
            .query_pairs()
            .filter(|(k, _)| !k.is_empty())
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        assert_eq!(bitreq_pairs, max_pairs);
        assert_eq!(bitreq_pairs, vec![("foo".to_string(), "bar".to_string())]);
    }

    #[test]
    fn query_pairs_normal_parity() {
        let bitreq = BitreqUrl::parse("http://example.com?foo=bar&baz=qux").unwrap();
        let max = MaxUrl::parse("http://example.com?foo=bar&baz=qux").unwrap();

        let bitreq_pairs: Vec<(String, String)> = bitreq.query_pairs().collect();
        let max_pairs: Vec<(String, String)> =
            max.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();

        assert_eq!(bitreq_pairs, max_pairs);
        assert_eq!(
            bitreq_pairs,
            vec![("foo".to_string(), "bar".to_string()), ("baz".to_string(), "qux".to_string())]
        );
    }

    #[test]
    fn username_parity() {
        // Normal username
        let bitreq = BitreqUrl::parse("http://user@example.com").unwrap();
        let max = MaxUrl::parse("http://user@example.com").unwrap();
        assert_eq!(bitreq.username(), max.username());

        // No username
        let bitreq = BitreqUrl::parse("http://example.com").unwrap();
        let max = MaxUrl::parse("http://example.com").unwrap();
        assert_eq!(bitreq.username(), max.username());
        assert_eq!(bitreq.username(), "");
    }

    #[test]
    fn password_empty_parity() {
        // Normal password
        let bitreq = BitreqUrl::parse("http://user:pass@example.com").unwrap();
        let max = MaxUrl::parse("http://user:pass@example.com").unwrap();
        assert_eq!(bitreq.password(), max.password());

        // No password
        let bitreq = BitreqUrl::parse("http://user@example.com").unwrap();
        let max = MaxUrl::parse("http://user@example.com").unwrap();
        assert_eq!(bitreq.password(), max.password());
        assert_eq!(bitreq.password(), None);

        // Empty password - both return None (url crate also filters empty password)
        let bitreq = BitreqUrl::parse("http://user:@example.com").unwrap();
        let max = MaxUrl::parse("http://user:@example.com").unwrap();
        assert_eq!(bitreq.password(), None);
        assert_eq!(bitreq.password(), max.password());
    }
}

// Parity tests for percent-encoded URLs
#[cfg(test)]
mod percent_encoding_parity {
    use super::*;

    #[test]
    fn username_with_percent_encoded_chars() {
        // %40 encodes '@'
        let bitreq = BitreqUrl::parse("http://user%40name@example.com").unwrap();
        let max = MaxUrl::parse("http://user%40name@example.com").unwrap();

        // Both should return raw percent-encoded username
        assert_eq!(bitreq.username(), max.username());
        assert_eq!(bitreq.username(), "user%40name");
    }

    #[test]
    fn password_with_percent_encoded_chars() {
        // %40 encodes '@', %3A encodes ':'
        let bitreq = BitreqUrl::parse("http://user:pass%40word@example.com").unwrap();
        let max = MaxUrl::parse("http://user:pass%40word@example.com").unwrap();

        // Both should return raw percent-encoded password
        assert_eq!(bitreq.password(), max.password());
        assert_eq!(bitreq.password(), Some("pass%40word"));
    }

    #[test]
    fn path_with_percent_encoded_chars() {
        // %2F encodes '/', %20 encodes space
        let bitreq = BitreqUrl::parse("http://example.com/path%2Fwith%20spaces").unwrap();
        let max = MaxUrl::parse("http://example.com/path%2Fwith%20spaces").unwrap();

        // Both should return raw percent-encoded path
        assert_eq!(bitreq.path(), max.path());
        assert_eq!(bitreq.path(), "/path%2Fwith%20spaces");
    }

    #[test]
    fn path_segments_with_percent_encoded_chars() {
        // Segments should be raw (percent-encoded)
        let bitreq = BitreqUrl::parse("http://example.com/seg%2F1/seg%202").unwrap();
        let max = MaxUrl::parse("http://example.com/seg%2F1/seg%202").unwrap();

        let bitreq_segments: Vec<&str> = bitreq.path_segments().collect();
        let max_segments: Vec<&str> =
            max.path_segments().unwrap().filter(|s| !s.is_empty()).collect();

        // Both return raw segments
        assert_eq!(bitreq_segments, max_segments);
        assert_eq!(bitreq_segments, vec!["seg%2F1", "seg%202"]);
    }

    #[test]
    fn query_with_percent_encoded_chars() {
        // Query string should be raw
        let bitreq = BitreqUrl::parse("http://example.com?key%3D1=value%26more").unwrap();
        let max = MaxUrl::parse("http://example.com?key%3D1=value%26more").unwrap();

        // Both should return raw query string
        assert_eq!(bitreq.query(), max.query());
        assert_eq!(bitreq.query(), Some("key%3D1=value%26more"));
    }

    #[test]
    fn query_pairs_with_percent_encoded_chars() {
        // url::Url::query_pairs() decodes percent-encoded chars
        // bitreq::Url::query_pairs() should match this behavior
        let bitreq = BitreqUrl::parse("http://example.com?key%3D1=value%26more").unwrap();
        let max = MaxUrl::parse("http://example.com?key%3D1=value%26more").unwrap();

        let bitreq_pairs: Vec<(String, String)> = bitreq.query_pairs().collect();
        let max_pairs: Vec<(String, String)> =
            max.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();

        // Both should decode percent-encoded chars
        assert_eq!(bitreq_pairs, max_pairs);
        // key%3D1 -> "key=1", value%26more -> "value&more"
        assert_eq!(bitreq_pairs, vec![("key=1".to_string(), "value&more".to_string())]);
    }

    #[test]
    fn query_pairs_with_plus_as_space() {
        // In form-urlencoded (query strings), '+' represents space
        let bitreq = BitreqUrl::parse("http://example.com?hello+world=foo+bar").unwrap();
        let max = MaxUrl::parse("http://example.com?hello+world=foo+bar").unwrap();

        let bitreq_pairs: Vec<(String, String)> = bitreq.query_pairs().collect();
        let max_pairs: Vec<(String, String)> =
            max.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();

        // Both should decode '+' as space
        assert_eq!(bitreq_pairs, max_pairs);
        assert_eq!(bitreq_pairs, vec![("hello world".to_string(), "foo bar".to_string())]);
    }

    #[test]
    fn query_pairs_with_utf8_percent_encoded() {
        // UTF-8 encoded character: ó = %C3%B3
        let bitreq = BitreqUrl::parse("http://example.com?name=%C3%B3").unwrap();
        let max = MaxUrl::parse("http://example.com?name=%C3%B3").unwrap();

        let bitreq_pairs: Vec<(String, String)> = bitreq.query_pairs().collect();
        let max_pairs: Vec<(String, String)> =
            max.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();

        // Both should decode UTF-8 sequences
        assert_eq!(bitreq_pairs, max_pairs);
        assert_eq!(bitreq_pairs, vec![("name".to_string(), "ó".to_string())]);
    }

    #[test]
    fn query_pairs_mixed_encoding() {
        // Mix of encoded and unencoded characters
        let bitreq = BitreqUrl::parse("http://example.com?a=1&b=%32&c=three").unwrap();
        let max = MaxUrl::parse("http://example.com?a=1&b=%32&c=three").unwrap();

        let bitreq_pairs: Vec<(String, String)> = bitreq.query_pairs().collect();
        let max_pairs: Vec<(String, String)> =
            max.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();

        assert_eq!(bitreq_pairs, max_pairs);
        // %32 decodes to '2'
        assert_eq!(
            bitreq_pairs,
            vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "2".to_string()),
                ("c".to_string(), "three".to_string()),
            ]
        );
    }
}
