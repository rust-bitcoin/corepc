// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

use bitreq::Url as BitreqUrl;

#[inline]
pub fn do_test(data: &[u8]) {
    // Convert the byte slice to a string, ignoring invalid UTF-8
    let input = String::from_utf8_lossy(data);

    // Try to parse with both implementations
    let bitreq_result = BitreqUrl::parse(&input);
    let url_result = url::Url::parse(&input);

    match (bitreq_result, url_result) {
        (Ok(bitreq_url), Ok(ref_url)) => {
            // Both parsed successfully - compare all accessors

            assert_eq!(
                bitreq_url.scheme(),
                ref_url.scheme(),
                "Scheme mismatch for input: {input:?}",
            );

            assert_eq!(
                bitreq_url.username(),
                ref_url.username(),
                "Username mismatch for input: {input:?}",
            );

            assert_eq!(
                bitreq_url.password(),
                ref_url.password(),
                "Password mismatch for input: {input:?}",
            );

            if let Some(ref_host) = ref_url.host_str() {
                assert_eq!(bitreq_url.base_url(), ref_host, "Host mismatch for input: {input:?}",);
            }

            // Port handling: url crate returns Option<u16> for explicit port,
            // while ours returns the actual port (explicit or default).
            // If url crate has an explicit port, it should match ours.
            if let Some(ref_port) = ref_url.port() {
                assert_eq!(bitreq_url.port(), ref_port, "Port mismatch for input: {input:?}",);
            }

            assert_eq!(bitreq_url.path(), ref_url.path(), "Path mismatch for input: {input:?}",);

            assert_eq!(
                bitreq_url.query(),
                ref_url.query(),
                "Query mismatch for input: {:?}",
                input
            );

            assert_eq!(
                bitreq_url.fragment(),
                ref_url.fragment(),
                "Fragment mismatch for input: {:?}",
                input
            );

            let _ = format!("{}", bitreq_url);
            let _ = bitreq_url.as_str();

            let bitreq_segments: Vec<_> = bitreq_url.path_segments().collect();
            let ref_segments: Vec<_> =
                ref_url.path_segments().map(|s| s.collect::<Vec<_>>()).unwrap_or_default();
            assert_eq!(
                bitreq_segments, ref_segments,
                "Path segments mismatch for input: {:?}",
                input
            );

            let bitreq_pairs: Vec<(String, String)> = bitreq_url.query_pairs().collect();
            let ref_pairs: Vec<(String, String)> =
                ref_url.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();
            assert_eq!(bitreq_pairs, ref_pairs, "Query pairs mismatch for input: {:?}", input);

            // Test append_query_params - use parts of the input as key/value
            // This exercises the expect in append_query_params
            {
                let mut url_clone = bitreq_url.clone();
                // Use the input itself as both key and value to exercise encoding
                url_clone.append_query_params([(&*input, &*input)]);
                // Verify the URL is still valid by accessing its fields
                let _ = url_clone.query();
                let _ = url_clone.as_str();

                // Test with empty strings
                let mut url_clone2 = bitreq_url.clone();
                url_clone2.append_query_params([("".into(), "".into())]);
                let _ = url_clone2.as_str();

                // Test appending multiple params in one call
                let mut url_clone3 = bitreq_url.clone();
                url_clone3.append_query_params([("key1", "value1"), ("key2", &input)]);
                let _ = url_clone3.as_str();
            }

            // Test preserve_fragment_from - exercises the expect in preserve_fragment_from
            {
                // Create a URL with a fragment to test preserving from
                if let Ok(url_with_frag) = BitreqUrl::parse("http://example.com#testfrag") {
                    let mut url_clone = bitreq_url.clone();
                    url_clone.preserve_fragment_from(&url_with_frag);
                    let new_frag = url_clone.fragment().unwrap();
                    assert_eq!(new_frag, "testfrag");
                    let _ = url_clone.as_str();
                }

                // Test with the original URL as the source (may or may not have fragment)
                let mut url_clone2 = bitreq_url.clone();
                url_clone2.preserve_fragment_from(&bitreq_url);
                let _ = url_clone2.as_str();

                // Test preserve_fragment_from with a URL that has no fragment
                if let Ok(url_no_frag) = BitreqUrl::parse("http://example.com/path") {
                    let mut url_clone3 = bitreq_url.clone();
                    url_clone3.preserve_fragment_from(&url_no_frag);
                    let _ = url_clone3.as_str();
                    assert_eq!(url_clone2.fragment(), url_clone3.fragment());
                }
            }
        }
        (Ok(v), Err(e)) => {
            panic!("bitreq parsed, URL did not. Input {input:?}. Err {e:?}");
        }
        (Err(e), Ok(v)) => match e {
            bitreq::UrlParseError::InvalidCharacter(_) => {
                // InvalidCharacter errors are currently expected as bitreq::Url only handles ASCII
                // characters.
            }
            bitreq::UrlParseError::MissingScheme | bitreq::UrlParseError::InvalidScheme => {
                // MissingScheme or InvalidScheme errors are expected as bitreq::Url parses the scheme more
                // strictly.
            }
            _ => {
                panic!("URL parsed, bitreq did not. Input {input:?}. Err {e:?}");
            }
        },
        (Err(_), Err(_)) => {
            // Both failed to parse - this is fine
        }
    }
}

#[no_mangle]
pub extern "C" fn url_parse_run(data: *const u8, datalen: usize) {
    do_test(unsafe { std::slice::from_raw_parts(data, datalen) });
}
