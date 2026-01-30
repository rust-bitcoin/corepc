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

    /// Test that password() returns the same value for both implementations.
    #[test]
    fn password_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        prop_assert_eq!(
            bitreq_url.password(),
            max_url.password(),
            "password() mismatch for URL: {}",
            url_string
        );
    }

    /// Test that path() returns the same value for both implementations.
    /// Note: url crate normalizes empty paths to "/" for URLs with authority,
    /// while bitreq returns "". We test that they match for non-empty paths.
    #[test]
    fn path_parity(url_string in special_url_string_strategy()) {
        let bitreq_url = BitreqUrl::parse(&url_string).expect("bitreq should parse");
        let max_url = MaxUrl::parse(&url_string).expect("url crate should parse");

        let bitreq_path = bitreq_url.path();
        let max_path = max_url.path();

        // url crate adds "/" for empty paths, bitreq returns ""
        // Both are valid interpretations, so we normalize for comparison
        let bitreq_normalized = if bitreq_path.is_empty() { "/" } else { bitreq_path };

        prop_assert_eq!(
            bitreq_normalized,
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
    /// Note: The url crate normalizes empty paths to "/" so we account for that.
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
        // (they may differ in empty path normalization)
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

    /// Test that path_segments() returns the same segments for both implementations.
    /// Note: url crate's path_segments() returns None for cannot-be-a-base URLs,
    /// but our generated URLs always have authority so this shouldn't happen.
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

        // Handle the empty path case: bitreq returns [""], url crate returns [""]
        // for "/" path (which url crate normalizes empty paths to)
        prop_assert_eq!(
            bitreq_segments,
            max_segments,
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
