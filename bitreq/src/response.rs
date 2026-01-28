use alloc::collections::BTreeMap;
use core::str;
#[cfg(feature = "async")]
use std::future::Future;
#[cfg(feature = "std")]
use std::io::{self, BufReader, Bytes, Read};

#[cfg(feature = "async")]
use tokio::io::{AsyncRead, AsyncReadExt};

#[cfg(feature = "std")]
use crate::connection::HttpStream;
use crate::Error;

#[cfg(feature = "std")]
const BACKING_READ_BUFFER_LENGTH: usize = 16 * 1024;
#[cfg(feature = "std")]
const MAX_CONTENT_LENGTH: usize = 16 * 1024;

/// An HTTP response.
///
/// Returned by [`Request::send`](struct.Request.html#method.send).
///
/// # Example
///
/// ```no_run
/// # #[cfg(feature = "std")]
/// # fn main() -> Result<(), bitreq::Error> {
/// let response = bitreq::get("http://example.com").send()?;
/// println!("{}", response.as_str()?);
/// # Ok(()) }
/// # #[cfg(not(feature = "std"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Response {
    /// The status code of the response, eg. 404.
    pub status_code: i32,
    /// The reason phrase of the response, eg. "Not Found".
    pub reason_phrase: String,
    /// The headers of the response. The header field names (the
    /// keys) are all lowercase.
    pub headers: BTreeMap<String, String>,
    /// The URL of the resource returned in this response. May differ from the
    /// request URL if it was redirected or typo corrections were applied (e.g.
    /// <http://example.com?foo=bar> would be corrected to
    /// <http://example.com/?foo=bar>).
    pub url: String,

    body: Vec<u8>,
}

impl Response {
    #[cfg(feature = "std")]
    pub(crate) fn create(
        mut parent: ResponseLazy,
        is_head: bool,
        max_body_size: Option<usize>,
    ) -> Result<Response, Error> {
        let mut body = Vec::new();
        if !is_head && parent.status_code != 204 && parent.status_code != 304 {
            for byte in &mut parent {
                let (byte, length) = byte?;
                if max_body_size.is_some_and(|max| body.len().saturating_add(length) > max) {
                    return Err(Error::BodyOverflow);
                }
                body.reserve(length);
                body.push(byte);
            }
        }

        let ResponseLazy { status_code, reason_phrase, headers, url, .. } = parent;

        Ok(Response { status_code, reason_phrase, headers, url, body })
    }

    #[cfg(feature = "async")]
    /// Fully read a [`Response`] from an async stream.
    ///
    /// When this crate was originally made "async", it actually just spawned sync requests on
    /// background threads and waited on their completion rather than actually doing async reads.
    /// In order to avoid changing the API while fixing this, we read the full response but then
    /// return a "lazy" response that has the full contents pre-read.
    pub(crate) async fn create_async<R: AsyncRead + Unpin>(
        stream: R,
        is_head: bool,
        max_headers_size: Option<usize>,
        max_status_line_len: Option<usize>,
        max_body_size: Option<usize>,
    ) -> Result<Response, Error> {
        use HttpStreamState::*;

        let mut stream = tokio::io::BufReader::with_capacity(BACKING_READ_BUFFER_LENGTH, stream);

        let ResponseMetadata {
            status_code,
            reason_phrase,
            mut headers,
            state,
            max_trailing_headers_size,
        } = read_metadata_async(&mut stream, max_headers_size, max_status_line_len).await?;

        let mut body = Vec::new();
        if !is_head && status_code != 204 && status_code != 304 {
            match state {
                EndOnClose => {
                    while let Some(byte_result) = read_until_closed_async(&mut stream).await {
                        let (byte, length) = byte_result?;
                        if max_body_size.is_some_and(|max| body.len().saturating_add(length) > max)
                        {
                            return Err(Error::BodyOverflow);
                        }
                        body.reserve(length);
                        body.push(byte);
                    }
                }
                ContentLength(mut length) => {
                    while let Some(byte_result) =
                        read_with_content_length_async(&mut stream, &mut length).await
                    {
                        let (byte, expected_length) = byte_result?;
                        if max_body_size
                            .is_some_and(|max| body.len().saturating_add(expected_length) > max)
                        {
                            return Err(Error::BodyOverflow);
                        }
                        body.reserve(expected_length);
                        body.push(byte);
                    }
                }
                Chunked(mut expecting_chunks, mut chunk_length, mut content_length) =>
                    while let Some(byte_result) = read_chunked_async(
                        &mut stream,
                        &mut headers,
                        &mut expecting_chunks,
                        &mut chunk_length,
                        &mut content_length,
                        max_trailing_headers_size,
                    )
                    .await
                    {
                        let (byte, length) = byte_result?;
                        if max_body_size.is_some_and(|max| body.len().saturating_add(length) > max)
                        {
                            return Err(Error::BodyOverflow);
                        }
                        body.reserve(length);
                        body.push(byte);
                    },
            }
        }

        Ok(Response { status_code, reason_phrase, headers, url: String::new(), body })
    }

    /// Returns the body as an `&str`.
    ///
    /// # Errors
    ///
    /// Returns
    /// [`InvalidUtf8InBody`](enum.Error.html#variant.InvalidUtf8InBody)
    /// if the body is not UTF-8, with a description as to why the
    /// provided slice is not UTF-8.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "std")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let url = "http://example.org/";
    /// let response = bitreq::get(url).send()?;
    /// println!("{}", response.as_str()?);
    /// # Ok(()) }
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// ```
    pub fn as_str(&self) -> Result<&str, Error> {
        match str::from_utf8(&self.body) {
            Ok(s) => Ok(s),
            Err(err) => Err(Error::InvalidUtf8InBody(err)),
        }
    }

    /// Returns a reference to the contained bytes of the body. If you
    /// want the `Vec<u8>` itself, use
    /// [`into_bytes()`](#method.into_bytes) instead.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "std")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let url = "http://example.org/";
    /// let response = bitreq::get(url).send()?;
    /// println!("{:?}", response.as_bytes());
    /// # Ok(()) }
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// ```
    pub fn as_bytes(&self) -> &[u8] { &self.body }

    /// Turns the `Response` into the inner `Vec<u8>`, the bytes that
    /// make up the response's body. If you just need a `&[u8]`, use
    /// [`as_bytes()`](#method.as_bytes) instead.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "std")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let url = "http://example.org/";
    /// let response = bitreq::get(url).send()?;
    /// println!("{:?}", response.into_bytes());
    /// // This would error, as into_bytes consumes the Response:
    /// // let x = response.status_code;
    /// # Ok(()) }
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// ```
    pub fn into_bytes(self) -> Vec<u8> { self.body }

    /// Converts JSON body to a `struct` using Serde.
    ///
    /// # Errors
    ///
    /// Returns
    /// [`SerdeJsonError`](enum.Error.html#variant.SerdeJsonError) if
    /// Serde runs into a problem, or
    /// [`InvalidUtf8InBody`](enum.Error.html#variant.InvalidUtf8InBody)
    /// if the body is not UTF-8.
    ///
    /// # Example
    /// In case compiler cannot figure out return type you might need to declare it explicitly:
    ///
    /// ```no_run
    /// use serde_json::Value;
    ///
    /// # fn main() -> Result<(), bitreq::Error> {
    /// # let url_to_json_resource = "http://example.org/resource.json";
    /// // Value could be any type that implements Deserialize!
    /// let user = bitreq::get(url_to_json_resource).send()?.json::<Value>()?;
    /// println!("User name is '{}'", user["name"]);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "json-using-serde")]
    pub fn json<'a, T>(&'a self) -> Result<T, Error>
    where
        T: serde::de::Deserialize<'a>,
    {
        match serde_json::from_slice(self.as_bytes()) {
            Ok(json) => Ok(json),
            Err(err) => Err(Error::SerdeJsonError(err)),
        }
    }
}

/// An HTTP response, which is loaded lazily.
///
/// In comparison to [`Response`](struct.Response.html), this is
/// returned from
/// [`send_lazy()`](struct.Request.html#method.send_lazy), where as
/// [`Response`](struct.Response.html) is returned from
/// [`send()`](struct.Request.html#method.send).
///
/// In practice, "lazy loading" means that the bytes are only loaded
/// as you iterate through them. The bytes are provided in the form of
/// a `Result<(u8, usize), bitreq::Error>`, as the reading operation
/// can fail in various ways. The `u8` is the actual byte that was
/// read, and `usize` is how many bytes we are expecting to read in
/// the future (including this byte). Note, however, that the `usize`
/// can change, particularly when the `Transfer-Encoding` is
/// `chunked`: then it will reflect how many bytes are left of the
/// current chunk. The expected size is capped at 16 KiB to avoid
/// server-side DoS attacks targeted at clients accidentally reserving
/// too much memory.
///
/// # Example
/// ```no_run
/// // This is how the normal Response works behind the scenes, and
/// // how you might use ResponseLazy.
/// # #[cfg(feature = "std")]
/// # fn main() -> Result<(), bitreq::Error> {
/// let response = bitreq::get("http://example.com").send_lazy()?;
/// let mut vec = Vec::new();
/// for result in response {
///     let (byte, length) = result?;
///     vec.reserve(length);
///     vec.push(byte);
/// }
/// # Ok(())
/// # }
/// # #[cfg(not(feature = "std"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
/// ```
#[cfg(feature = "std")]
pub struct ResponseLazy {
    /// The status code of the response, eg. 404.
    pub status_code: i32,
    /// The reason phrase of the response, eg. "Not Found".
    pub reason_phrase: String,
    /// The headers of the response. The header field names (the
    /// keys) are all lowercase.
    pub headers: BTreeMap<String, String>,
    /// The URL of the resource returned in this response. May differ from the
    /// request URL if it was redirected or typo corrections were applied (e.g.
    /// <http://example.com?foo=bar> would be corrected to
    /// <http://example.com/?foo=bar>).
    pub url: String,

    stream: HttpStreamBytes,
    state: HttpStreamState,
    max_trailing_headers_size: Option<usize>,
    max_body_size: Option<usize>,
    bytes_read: usize,
}

#[cfg(feature = "std")]
type HttpStreamBytes = Bytes<BufReader<HttpStream>>;

#[cfg(feature = "std")]
impl ResponseLazy {
    pub(crate) fn from_stream(
        stream: HttpStream,
        max_headers_size: Option<usize>,
        max_status_line_len: Option<usize>,
        max_body_size: Option<usize>,
    ) -> Result<ResponseLazy, Error> {
        let mut stream = BufReader::with_capacity(BACKING_READ_BUFFER_LENGTH, stream).bytes();
        let ResponseMetadata {
            status_code,
            reason_phrase,
            headers,
            state,
            max_trailing_headers_size,
        } = read_metadata(&mut stream, max_headers_size, max_status_line_len)?;

        Ok(ResponseLazy {
            status_code,
            reason_phrase,
            headers,
            url: String::new(),
            stream,
            state,
            max_trailing_headers_size,
            max_body_size,
            bytes_read: 0,
        })
    }

    #[cfg(feature = "async")]
    pub(crate) fn dummy_from_response(response: Response) -> ResponseLazy {
        let http_stream = HttpStream::create_buffer(response.body);
        ResponseLazy {
            status_code: response.status_code,
            reason_phrase: response.reason_phrase,
            headers: response.headers,
            url: response.url,
            stream: BufReader::with_capacity(1, http_stream).bytes(),
            state: HttpStreamState::EndOnClose,
            max_trailing_headers_size: None,
            // Body was already fully loaded and size-checked by send_async
            max_body_size: None,
            bytes_read: 0,
        }
    }
}

#[cfg(feature = "std")]
impl Iterator for ResponseLazy {
    type Item = Result<(u8, usize), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        use HttpStreamState::*;
        let result = match self.state {
            EndOnClose => read_until_closed(&mut self.stream),
            ContentLength(ref mut length) => read_with_content_length(&mut self.stream, length),
            Chunked(ref mut expecting_chunks, ref mut length, ref mut content_length) =>
                read_chunked(
                    &mut self.stream,
                    &mut self.headers,
                    expecting_chunks,
                    length,
                    content_length,
                    self.max_trailing_headers_size,
                ),
        };

        // Check body size limit before returning the byte
        if let Some(Ok((_, expected_length))) = &result {
            if self.max_body_size.is_some_and(|max| self.bytes_read + expected_length > max) {
                return Some(Err(Error::BodyOverflow));
            }
            self.bytes_read += 1;
        }

        result
    }
}

#[cfg(feature = "std")]
impl Read for ResponseLazy {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut index = 0;
        for res in self {
            // there is no use for the estimated length in the read implementation
            // so it is ignored.
            let (byte, _) = res.map_err(|e| match e {
                Error::IoError(e) => e,
                _ => io::Error::new(io::ErrorKind::Other, e),
            })?;

            buf[index] = byte;
            index += 1;

            // if the buffer is full, it should stop reading
            if index >= buf.len() {
                break;
            }
        }

        // index of the next byte is the number of bytes thats have been read
        Ok(index)
    }
}

#[cfg(feature = "std")]
enum HttpStreamState {
    // No Content-Length, and Transfer-Encoding != chunked, so we just
    // read unti lthe server closes the connection (this should be the
    // fallback, if I read the rfc right).
    EndOnClose,
    // Content-Length was specified, read that amount of bytes
    ContentLength(usize),
    // Transfer-Encoding == chunked, so we need to save two pieces of
    // information: are we expecting more chunks, how much is there
    // left of the current chunk, and how much have we read? The last
    // number is needed in order to provide an accurate Content-Length
    // header after loading all the bytes.
    Chunked(bool, usize, usize),
}

// This struct is just used in the Response and ResponseLazy
// constructors, but not in their structs, for api-cleanliness
// reasons. (Eg. response.status_code is much cleaner than
// response.meta.status_code or similar.)
#[cfg(feature = "std")]
struct ResponseMetadata {
    status_code: i32,
    reason_phrase: String,
    headers: BTreeMap<String, String>,
    state: HttpStreamState,
    max_trailing_headers_size: Option<usize>,
}

macro_rules! maybe_await {
    ($e: expr, await) => {
        $e.await
    };
    ($e: expr,) => {
        $e
    };
}

#[cfg(feature = "async")]
/// We need to mungle [`AsyncRead`] to look like an iterator, which we do here.
trait AsyncIteratorReadExt {
    fn next(&mut self) -> impl Future<Output = Option<Result<u8, io::Error>>>;
}

#[cfg(feature = "async")]
impl<T: AsyncReadExt + Unpin> AsyncIteratorReadExt for T {
    async fn next(&mut self) -> Option<Result<u8, io::Error>> { Some(self.read_u8().await) }
}

macro_rules! define_read_methods {
    (($read_until_closed: ident, $read_with_content_length: ident, $read_trailers: ident, $read_chunked: ident, $read_metadata: ident, $read_line: ident)<$($arg: ident : $($argty: path $(|)?)*),*>, $stream_type: ident $(, $async: tt, $await: tt)?) => {
        $($async)? fn $read_until_closed<$($arg: $($argty +)*),*>(
            bytes: &mut $stream_type,
        ) -> Option<<ResponseLazy as Iterator>::Item> {
            if let Some(byte) = maybe_await!(bytes.next(), $($await)?) {
                match byte {
                    Ok(byte) => Some(Ok((byte, 1))),
                    Err(err) => Some(Err(Error::IoError(err))),
                }
            } else {
                None
            }
        }

        $($async)? fn $read_with_content_length<$($arg: $($argty +)*),*>(
            bytes: &mut $stream_type,
            content_length: &mut usize,
        ) -> Option<<ResponseLazy as Iterator>::Item> {
            if *content_length > 0 {
                *content_length -= 1;

                if let Some(byte) = maybe_await!(bytes.next(), $($await)?) {
                    match byte {
                        // Cap Content-Length to 16KiB, to avoid out-of-memory issues.
                        Ok(byte) => return Some(Ok((byte, (*content_length).min(MAX_CONTENT_LENGTH) + 1))),
                        Err(err) => return Some(Err(Error::IoError(err))),
                    }
                }
            }
            None
        }

        $($async)? fn $read_trailers<$($arg: $($argty +)*),*>(
            bytes: &mut $stream_type,
            headers: &mut BTreeMap<String, String>,
            mut max_headers_size: Option<usize>,
        ) -> Result<(), Error> {
            loop {
                let trailer_line = maybe_await!($read_line(bytes, max_headers_size, Error::HeadersOverflow), $($await)?)?;
                if let Some(ref mut max_headers_size) = max_headers_size {
                    *max_headers_size -= trailer_line.len() + 2;
                }
                if let Some((header, value)) = parse_header(trailer_line) {
                    headers.insert(header, value);
                } else {
                    break;
                }
            }
            Ok(())
        }

        $($async)? fn $read_chunked<$($arg: $($argty +)*),*>(
            bytes: &mut $stream_type,
            headers: &mut BTreeMap<String, String>,
            expecting_more_chunks: &mut bool,
            chunk_length: &mut usize,
            content_length: &mut usize,
            max_trailing_headers_size: Option<usize>,
        ) -> Option<<ResponseLazy as Iterator>::Item> {
            if !*expecting_more_chunks && *chunk_length == 0 {
                return None;
            }

            if *chunk_length == 0 {
                // Max length of the chunk length line is 1KB: not too long to
                // take up much memory, long enough to tolerate some chunk
                // extensions (which are ignored).

                // Get the size of the next chunk
                let length_line = match maybe_await!($read_line(bytes, Some(1024), Error::MalformedChunkLength), $($await)?) {
                    Ok(line) => line,
                    Err(err) => return Some(Err(err)),
                };

                // Note: the trim() and check for empty lines shouldn't be
                // needed according to the RFC, but we might as well, it's a
                // small change and it fixes a few servers.
                let incoming_length = if length_line.is_empty() {
                    0
                } else {
                    let length = if let Some(i) = length_line.find(';') {
                        length_line[..i].trim()
                    } else {
                        length_line.trim()
                    };
                    match usize::from_str_radix(length, 16) {
                        Ok(length) => length,
                        Err(_) => return Some(Err(Error::MalformedChunkLength)),
                    }
                };

                if incoming_length == 0 {
                    if let Err(err) = maybe_await!($read_trailers(bytes, headers, max_trailing_headers_size), $($await)?) {
                        return Some(Err(err));
                    }

                    *expecting_more_chunks = false;
                    headers.insert("content-length".to_string(), (*content_length).to_string());
                    headers.remove("transfer-encoding");
                    return None;
                }
                *chunk_length = incoming_length;
                *content_length = content_length.saturating_add(incoming_length);
            }

            if *chunk_length > 0 {
                *chunk_length -= 1;
                if let Some(byte) = maybe_await!(bytes.next(), $($await)?) {
                    match byte {
                        Ok(byte) => {
                            // If we're at the end of the chunk...
                            if *chunk_length == 0 {
                                //...read the trailing \r\n of the chunk, and
                                // possibly return an error instead.

                                // TODO: Maybe this could be written in a way
                                // that doesn't discard the last ok byte if
                                // the \r\n reading fails?
                                if let Err(err) = maybe_await!($read_line(bytes, Some(2), Error::MalformedChunkEnd), $($await)?) {
                                    return Some(Err(err));
                                }
                            }

                            return Some(Ok((byte, (*chunk_length).min(MAX_CONTENT_LENGTH) + 1)));
                        }
                        Err(err) => return Some(Err(Error::IoError(err))),
                    }
                }
            }

            None
        }

        #[cfg(feature = "std")]
        $($async)? fn $read_metadata<$($arg: $($argty +)*),*>(
            stream: &mut $stream_type,
            mut max_headers_size: Option<usize>,
            max_status_line_len: Option<usize>,
        ) -> Result<ResponseMetadata, Error> {
            let line = maybe_await!($read_line(stream, max_status_line_len, Error::StatusLineOverflow), $($await)?)?;
            let (status_code, reason_phrase) = parse_status_line(&line);

            let mut headers = BTreeMap::new();
            loop {
                let line = maybe_await!($read_line(stream, max_headers_size, Error::HeadersOverflow), $($await)?)?;
                if line.is_empty() {
                    // Body starts here
                    break;
                }
                if let Some(ref mut max_headers_size) = max_headers_size {
                    *max_headers_size -= line.len() + 2;
                }
                if let Some(header) = parse_header(line) {
                    headers.insert(header.0, header.1);
                }
            }

            let mut chunked = false;
            let mut content_length = None;
            for (header, value) in &headers {
                // Handle the Transfer-Encoding header
                if header.to_lowercase().trim() == "transfer-encoding"
                    && value.to_lowercase().trim() == "chunked"
                {
                    chunked = true;
                }

                // Handle the Content-Length header
                if header.to_lowercase().trim() == "content-length" {
                    match str::parse::<usize>(value.trim()) {
                        Ok(length) => content_length = Some(length),
                        Err(_) => return Err(Error::MalformedContentLength),
                    }
                }
            }

            let state = if chunked {
                HttpStreamState::Chunked(true, 0, 0)
            } else if let Some(length) = content_length {
                HttpStreamState::ContentLength(length)
            } else {
                HttpStreamState::EndOnClose
            };

            Ok(ResponseMetadata {
                status_code,
                reason_phrase,
                headers,
                state,
                max_trailing_headers_size: max_headers_size,
            })
        }

        #[cfg(feature = "std")]
        $($async)? fn $read_line<$($arg: $($argty +)*),*>(
            stream: &mut $stream_type,
            max_len: Option<usize>,
            overflow_error: Error,
        ) -> Result<String, Error> {
            let mut bytes = Vec::with_capacity(32);
            while let Some(byte) = maybe_await!(stream.next(), $($await)?) {
                match byte {
                    Ok(byte) => {
                        if let Some(max_len) = max_len {
                            if bytes.len() >= max_len {
                                return Err(overflow_error);
                            }
                        }
                        if byte == b'\n' {
                            if let Some(b'\r') = bytes.last() {
                                bytes.pop();
                            }
                            break;
                        } else {
                            bytes.push(byte);
                        }
                    }
                    Err(err) => return Err(Error::IoError(err)),
                }
            }
            String::from_utf8(bytes).map_err(|_error| Error::InvalidUtf8InResponse)
        }
    }
}

#[cfg(feature = "std")]
define_read_methods!((read_until_closed, read_with_content_length, read_trailers, read_chunked, read_metadata, read_line)<>, HttpStreamBytes);
#[cfg(feature = "async")]
define_read_methods!((read_until_closed_async, read_with_content_length_async, read_trailers_async, read_chunked_async, read_metadata_async, read_line_async)<R: AsyncRead | Unpin>, R, async, await);

#[cfg(feature = "std")]
fn parse_status_line(line: &str) -> (i32, String) {
    // sample status line format
    // HTTP/1.1 200 OK
    let mut status_code = String::with_capacity(3);
    let mut reason_phrase = String::with_capacity(2);

    let mut spaces = 0;

    for c in line.chars() {
        if spaces >= 2 {
            reason_phrase.push(c);
        }

        if c == ' ' {
            spaces += 1;
        } else if spaces == 1 {
            status_code.push(c);
        }
    }

    if let Ok(status_code) = status_code.parse::<i32>() {
        return (status_code, reason_phrase);
    }

    (503, "Server did not provide a status line".to_string())
}

#[cfg(feature = "std")]
fn parse_header(mut line: String) -> Option<(String, String)> {
    if let Some(location) = line.find(':') {
        // Trim the first character of the header if it is a space,
        // otherwise return everything after the ':'. This should
        // preserve the behavior in versions <=2.0.1 in most cases
        // (namely, ones where it was valid), where the first
        // character after ':' was always cut off.
        let value = if let Some(sp) = line.get(location + 1..location + 2) {
            if sp == " " {
                line[location + 2..].to_string()
            } else {
                line[location + 1..].to_string()
            }
        } else {
            line[location + 1..].to_string()
        };

        line.truncate(location);
        // Headers should be ascii, I'm pretty sure. If not, please open an issue.
        line.make_ascii_lowercase();
        return Some((line, value));
    }
    None
}
