//! WASM-specific HTTP implementation using extern C functions
//!
//! This module provides HTTP functionality for WebAssembly environments
//! by delegating the actual network operations to JavaScript through
//! extern C function calls.

use crate::{Error, Response};
use crate::request::ParsedRequest;
use alloc::string::String;
use alloc::vec;

/// Maximum size for HTTP response bodies in WASM
const MAX_RESPONSE_SIZE: usize = 10 * 1024 * 1024; // 10MB

// Extern C functions that must be implemented by the JavaScript host environment
extern "C" {
    /// Performs an HTTP request and returns the response.
    ///
    /// # Parameters
    ///
    /// - method: pointer to null-terminated method string (GET, POST, etc.)
    /// - url: pointer to null-terminated URL string
    /// - headers: pointer to null-terminated headers string (key:value\nkey:value format)
    /// - body: pointer to request body bytes
    /// - body_len: length of request body
    /// - response_buf: buffer to write response into
    /// - response_buf_len: size of response buffer
    ///
    /// # Returns
    ///
    /// - Positive value: actual response length written to buffer
    /// - Negative value: error code
    /// - 0: empty response
    fn bitreq_wasm_http_request(
        method: *const u8,
        url: *const u8,
        headers: *const u8,
        body: *const u8,
        body_len: usize,
        response_buf: *mut u8,
        response_buf_len: usize,
    ) -> i32;

    /// Gets the HTTP status code from the last request.
    fn bitreq_wasm_get_status_code() -> i32;

    /// Gets the response headers from the last request
    ///
    /// # Parameters
    ///
    /// - headers_buf: buffer to write headers into (key:value\nkey:value format)
    /// - headers_buf_len: size of headers buffer
    ///
    /// # Returns
    ///
    /// - Positive value: actual headers length written to buffer
    /// - Negative value: error code
    /// - 0: no headers
    fn bitreq_wasm_get_response_headers(
        headers_buf: *mut u8,
        headers_buf_len: usize,
    ) -> i32;
}

/// Sends an HTTP request using WASM extern functions.
pub(crate) fn send_request(parsed_request: ParsedRequest) -> Result<Response, Error> {
    // Convert method to C string
    let method_str = format!("{}\0", parsed_request.config.method);
    let method_ptr = method_str.as_ptr();

    // Build full URL
    let full_url = build_full_url(&parsed_request);
    let url_str = format!("{}\0", full_url);
    let url_ptr = url_str.as_ptr();

    // Convert headers to string format
    let headers_str = build_headers_string(&parsed_request);
    let headers_ptr = headers_str.as_ptr();

    // Get request body
    let body = parsed_request.get_body().map(|b| b.as_slice()).unwrap_or(&[]);
    let body_ptr = body.as_ptr();
    let body_len = body.len();

    // Prepare response buffer
    let mut response_buf = vec![0u8; MAX_RESPONSE_SIZE];
    let response_buf_ptr = response_buf.as_mut_ptr();
    let response_buf_len = response_buf.len();

    // Make the extern C call
    let result = unsafe {
        bitreq_wasm_http_request(
            method_ptr,
            url_ptr,
            headers_ptr,
            body_ptr,
            body_len,
            response_buf_ptr,
            response_buf_len,
        )
    };

    // Handle the result
    if result < 0 {
        return Err(Error::Other("WASM HTTP request failed"));
    }

    let response_len = result as usize;
    response_buf.truncate(response_len);

    // Get status code
    let status_code = unsafe { bitreq_wasm_get_status_code() };

    // Get response headers
    let response_headers = get_response_headers()?;

    // Build and return Response
    Ok(Response::new(
        status_code,
        get_reason_phrase(status_code),
        response_headers,
        full_url,
        response_buf,
    ))
}

/// Builds the full URL including path and query parameters.
fn build_full_url(parsed_request: &ParsedRequest) -> String {
    parsed_request.url.as_str().to_string()
}

/// Builds headers string in `key:value\nkey:value` format.
fn build_headers_string(parsed_request: &ParsedRequest) -> String {
    let mut headers_str = String::new();

    for (key, value) in parsed_request.get_headers() {
        if !headers_str.is_empty() {
            headers_str.push('\n');
        }
        headers_str.push_str(key);
        headers_str.push(':');
        headers_str.push_str(value);
    }

    headers_str.push('\0'); // Null terminate
    headers_str
}

/// Retrieves response headers from the WASM environment.
fn get_response_headers() -> Result<crate::alloc::collections::BTreeMap<String, String>, Error> {
    use alloc::collections::BTreeMap;

    let mut headers_buf = vec![0u8; 8192]; // 8KB for headers
    let headers_buf_ptr = headers_buf.as_mut_ptr();
    let headers_buf_len = headers_buf.len();

    let result = unsafe {
        bitreq_wasm_get_response_headers(headers_buf_ptr, headers_buf_len)
    };

    if result < 0 {
        return Err(Error::Other("Failed to get response headers"));
    }

    let headers_len = result as usize;
    headers_buf.truncate(headers_len);

    // Parse headers string
    let headers_str = String::from_utf8(headers_buf)
        .map_err(|_| Error::Other("Invalid UTF-8 in response headers"))?;

    let mut headers = BTreeMap::new();

    for line in headers_str.lines() {
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_lowercase();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    Ok(headers)
}

/// Gets a reason phrase for the given status code.
fn get_reason_phrase(status_code: i32) -> String {
    match status_code {
        200 => "OK".to_string(),
        201 => "Created".to_string(),
        202 => "Accepted".to_string(),
        204 => "No Content".to_string(),
        301 => "Moved Permanently".to_string(),
        302 => "Found".to_string(),
        303 => "See Other".to_string(),
        304 => "Not Modified".to_string(),
        307 => "Temporary Redirect".to_string(),
        308 => "Permanent Redirect".to_string(),
        400 => "Bad Request".to_string(),
        401 => "Unauthorized".to_string(),
        403 => "Forbidden".to_string(),
        404 => "Not Found".to_string(),
        405 => "Method Not Allowed".to_string(),
        429 => "Too Many Requests".to_string(),
        500 => "Internal Server Error".to_string(),
        502 => "Bad Gateway".to_string(),
        503 => "Service Unavailable".to_string(),
        504 => "Gateway Timeout".to_string(),
        _ => "Unknown".to_string(),
    }
}
