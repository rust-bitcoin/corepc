// Property-based tests for URL parsing based on WHATWG URL Standard
// https://url.spec.whatwg.org/

mod common;

use bitreq::Url;
use common::valid_url_strategy;
use proptest::prelude::*;

proptest! {
    /// Test that all generated valid URLs can be parsed successfully.
    #[test]
    fn valid_urls_parse_successfully(valid_url in valid_url_strategy()) {
        let result = Url::parse(&valid_url.url_string);
        prop_assert!(
            result.is_ok(),
            "Failed to parse valid URL: {} - Error: {:?}",
            valid_url.url_string,
            result.err()
        );
    }

    /// Test round-trip: as_str() returns the same string that was parsed.
    #[test]
    fn parse_as_str_roundtrip(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        prop_assert_eq!(
            parsed.as_str(),
            &valid_url.url_string,
            "Round-trip failed for URL"
        );
    }

    /// Test that scheme() returns the expected scheme.
    #[test]
    fn scheme_returns_expected_value(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        prop_assert_eq!(
            parsed.scheme(),
            &valid_url.scheme,
            "Scheme mismatch"
        );
    }

    /// Test that base_url() returns the expected host.
    #[test]
    fn base_url_returns_expected_value(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        prop_assert_eq!(
            parsed.base_url(),
            &valid_url.host,
            "Host mismatch"
        );
    }

    /// Test that port() returns the expected effective port.
    ///
    /// Note: port() returns u16 (the effective port), while valid_url.port is Option<u16>
    /// (the explicitly specified port). For known schemes without explicit port, the default
    /// port is returned. For unknown schemes without explicit port, this test is skipped.
    #[test]
    fn port_returns_expected_value(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");

        // Compute expected effective port based on explicit port or scheme default
        let expected_port = match valid_url.port {
            Some(p) => p,
            None => match valid_url.scheme.as_str() {
                "http" | "ws" => 80,
                "https" | "wss" => 443,
                "ftp" => 21,
                // Unknown scheme without explicit port - skip this test case
                _ => return Ok(()),
            }
        };

        prop_assert_eq!(
            parsed.port(),
            expected_port,
            "Port mismatch"
        );
    }

    /// Test that path() returns the expected path.
    #[test]
    fn path_returns_expected_value(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        let expected_path = if valid_url.path.is_empty() { "/" } else { &valid_url.path};
        prop_assert_eq!(
            parsed.path(),
            expected_path,
            "Path mismatch"
        );
    }

    /// Test that query() returns the expected query string.
    #[test]
    fn query_returns_expected_value(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        prop_assert_eq!(
            parsed.query(),
            valid_url.query.as_deref(),
            "Query mismatch"
        );
    }

    /// Test that fragment() returns the expected fragment.
    #[test]
    fn fragment_returns_expected_value(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        prop_assert_eq!(
            parsed.fragment(),
            valid_url.fragment.as_deref(),
            "Fragment mismatch"
        );
    }

    /// Test that path_segments() correctly splits the path, filtering empty segments.
    #[test]
    fn path_segments_splits_correctly(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");

        let path = &valid_url.path;
        let expected_segments: Vec<&str> = if path.is_empty() {
            vec![]
        } else if let Some(stripped) = path.strip_prefix('/') {
            stripped.split('/').filter(|s| !s.is_empty()).collect()
        } else {
            path.split('/').filter(|s| !s.is_empty()).collect()
        };

        let actual_segments: Vec<&str> = parsed.path_segments().collect();
        prop_assert_eq!(
            actual_segments,
            expected_segments,
            "Path segments mismatch"
        );
    }

    /// Test Display implementation matches as_str().
    #[test]
    fn display_matches_as_str(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        prop_assert_eq!(
            format!("{}", parsed),
            parsed.as_str(),
            "Display doesn't match as_str()"
        );
    }

    /// Test that parsing is deterministic - same input always produces same output.
    #[test]
    fn parsing_is_deterministic(valid_url in valid_url_strategy()) {
        let parsed1 = Url::parse(&valid_url.url_string).expect("should parse");
        let parsed2 = Url::parse(&valid_url.url_string).expect("should parse");
        prop_assert_eq!(parsed1, parsed2, "Parsing is not deterministic");
    }

    /// Test query_pairs() correctly parses query parameters, filtering empty keys.
    /// Note: query_pairs() now decodes percent-encoded values, but our test URLs
    /// don't contain percent-encoded characters, so decoded output matches input.
    #[test]
    fn query_pairs_parses_correctly(
        valid_url in valid_url_strategy()
            .prop_filter("has query", |u| u.query.is_some())
    ) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        let query = valid_url.query.as_ref().unwrap();

        let expected_pairs: Vec<(String, String)> = query
            .split('&')
            .map(|pair| {
                let pair = pair.trim();
                if let Some(eq_pos) = pair.find('=') {
                    (pair[..eq_pos].trim().to_string(), pair[eq_pos + 1..].trim().to_string())
                } else {
                    (pair.trim().to_string(), String::new())
                }
            })
            .filter(|(k, _)| !k.is_empty())
            .collect();

        let actual_pairs: Vec<(String, String)> = parsed.query_pairs().collect();
        prop_assert_eq!(
            actual_pairs,
            expected_pairs,
            "Query pairs mismatch"
        );
    }
}

// Property tests for empty segment and empty key filtering behavior
proptest! {
    /// Test that path_segments never returns empty string segments.
    #[test]
    fn path_segments_never_returns_empty(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        let segments: Vec<&str> = parsed.path_segments().collect();

        for segment in &segments {
            prop_assert!(
                !segment.is_empty(),
                "path_segments() returned empty segment for URL: {}",
                valid_url.url_string
            );
        }
    }

    /// Test that query_pairs never returns pairs with empty keys.
    #[test]
    fn query_pairs_never_returns_empty_keys(valid_url in valid_url_strategy()) {
        let parsed = Url::parse(&valid_url.url_string).expect("should parse");
        let pairs: Vec<(String, String)> = parsed.query_pairs().collect();

        for (key, _) in &pairs {
            prop_assert!(
                !key.is_empty(),
                "query_pairs() returned empty key for URL: {}",
                valid_url.url_string
            );
        }
    }
}

// Additional edge case tests using proptest for schemes with special characters.
proptest! {
    #[test]
    fn scheme_with_special_chars_parses(
        base in "[a-z]",
        special in prop::sample::select(vec!['+', '-', '.']),
        suffix in "[a-z0-9]{0,3}",
        port in any::<u16>()
    ) {
        let scheme = format!("{}{}{}", base, special, suffix);
        // Unknown schemes require an explicit port
        let url_string = format!("{}://example.com:{}", scheme, port);
        let parsed = Url::parse(&url_string);
        prop_assert!(
            parsed.is_ok(),
            "Failed to parse URL with scheme '{}': {:?}",
            scheme,
            parsed.as_ref().err()
        );
        let parsed = parsed.unwrap();
        prop_assert_eq!(parsed.scheme(), scheme);
    }
}

// Tests for invalid and arbitrary input strings.
// These ensure the parser never panics and correctly rejects invalid input.
proptest! {
    /// Test that parsing arbitrary random strings never panics.
    /// The result can be Ok or Err, but it must not panic.
    #[test]
    fn arbitrary_strings_never_panic(s in ".*") {
        // This should never panic, regardless of what string is passed
        let _result = Url::parse(&s);
        // If we get here without panicking, the test passes
    }

    /// Test that parsing arbitrary byte sequences (as UTF-8 strings) never panics.
    #[test]
    fn arbitrary_bytes_as_string_never_panic(bytes in prop::collection::vec(any::<u8>(), 0..200)) {
        // Convert bytes to a string, lossy conversion for invalid UTF-8
        let s = String::from_utf8_lossy(&bytes);
        let _result = Url::parse(&s);
        // If we get here without panicking, the test passes
    }

    /// Test that empty string is rejected.
    #[test]
    fn empty_string_fails(s in prop::string::string_regex("").unwrap()) {
        let result = Url::parse(&s);
        prop_assert!(result.is_err(), "Empty string should fail to parse");
    }

    /// Test that strings without "://" separator are rejected.
    #[test]
    fn missing_scheme_separator_fails(
        scheme in "[a-z]{1,10}",
        rest in "[a-z0-9./-]{0,50}"
    ) {
        // Create a URL-like string but without "://"
        let invalid_url = format!("{}{}", scheme, rest);
        let result = Url::parse(&invalid_url);
        prop_assert!(
            result.is_err(),
            "URL without '://' should fail: {}",
            invalid_url
        );
    }

    /// Test that schemes starting with a digit are rejected.
    #[test]
    fn scheme_starting_with_digit_fails(
        digit in "[0-9]",
        rest in "[a-z0-9]{0,10}",
        host in "[a-z]{2,10}"
    ) {
        let invalid_url = format!("{}{}://{}", digit, rest, host);
        let result = Url::parse(&invalid_url);
        prop_assert!(
            result.is_err(),
            "Scheme starting with digit should fail: {}",
            invalid_url
        );
    }

    /// Test that schemes with invalid characters are rejected.
    #[test]
    fn scheme_with_invalid_chars_fails(
        prefix in "[a-z]{1,5}",
        invalid_char in prop::sample::select(vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', ' ', '\t']),
        suffix in "[a-z]{0,5}",
        host in "[a-z]{2,10}"
    ) {
        let invalid_url = format!("{}{}{}://{}", prefix, invalid_char, suffix, host);
        let result = Url::parse(&invalid_url);
        prop_assert!(
            result.is_err(),
            "Scheme with invalid char '{}' should fail: {}",
            invalid_char,
            invalid_url
        );
    }

    /// Test that URLs with control characters are rejected.
    ///
    /// We skip cases with trailing whitespaces as they will be trimmed.
    #[test]
    fn control_characters_fail(
        prefix in "[a-z]{1,5}://[a-z]{2,10}",
        ctrl_char in 0u8..32u8,
        suffix in "[a-z]{0,10}"
    ) {
        if ctrl_char.is_ascii_whitespace() && suffix.is_empty() {
            return Ok(());
        }
        let invalid_url = format!("{}{}{}", prefix, ctrl_char as char, suffix);
        let result = Url::parse(&invalid_url);
        prop_assert!(
            result.is_err(),
            "URL with control char (0x{:02x}) should fail: {:?}",
            ctrl_char,
            invalid_url
        );
    }

    /// Test that URLs with non-ASCII characters are rejected.
    #[test]
    fn non_ascii_characters_fail(
        prefix in "[a-z]{1,5}://[a-z]{2,10}",
        non_ascii in 128u8..=255u8,
        suffix in "[a-z]{0,10}"
    ) {
        // Create a string with a non-ASCII byte
        let mut bytes = prefix.into_bytes();
        bytes.push(non_ascii);
        bytes.extend(suffix.bytes());
        let invalid_url = String::from_utf8_lossy(&bytes).into_owned();
        let result = Url::parse(&invalid_url);
        prop_assert!(
            result.is_err(),
            "URL with non-ASCII char should fail: {:?}",
            invalid_url
        );
    }

    /// Test that ports exceeding u16::MAX are rejected.
    #[test]
    fn port_overflow_fails(
        scheme in "[a-z]{1,5}",
        host in "[a-z]{2,10}",
        port in 65536u32..=999999u32
    ) {
        let invalid_url = format!("{}://{}:{}", scheme, host, port);
        let result = Url::parse(&invalid_url);
        prop_assert!(
            result.is_err(),
            "Port {} exceeds u16::MAX and should fail: {}",
            port,
            invalid_url
        );
    }

    /// Test that empty scheme (just "://") is rejected.
    #[test]
    fn empty_scheme_fails(host in "[a-z]{2,10}") {
        let invalid_url = format!("://{}", host);
        let result = Url::parse(&invalid_url);
        prop_assert!(
            result.is_err(),
            "Empty scheme should fail: {}",
            invalid_url
        );
    }

    /// Test various malformed URL patterns.
    #[test]
    fn malformed_url_patterns_fail(
        pattern in prop::sample::select(vec![
            "://example.com".to_string(),        // Empty scheme
            ":example.com".to_string(),          // Missing slashes
            "http//example.com".to_string(),     // Missing colon
            "http:/example.com".to_string(),     // Only one slash
            "http:example.com".to_string(),      // No slashes
            "123://example.com".to_string(),     // Scheme starts with digit
            "http ://example.com".to_string(),   // Space in scheme
            "ht tp://example.com".to_string(),   // Space in scheme
            "".to_string(),                      // Empty string
        ])
    ) {
        let result = Url::parse(&pattern);
        prop_assert!(
            result.is_err(),
            "Malformed pattern should fail: {:?}",
            pattern
        );
    }
}
