use core::time::Duration;
#[cfg(feature = "async")]
use std::future::Future;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
#[cfg(feature = "async")]
use std::pin::Pin;
#[cfg(feature = "async")]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(feature = "async")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "async")]
use std::task::{Context, Poll};
use std::time::Instant;

#[cfg(all(feature = "async", feature = "proxy"))]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async")]
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadHalf, WriteHalf};
#[cfg(feature = "async")]
use tokio::net::TcpStream as AsyncTcpStream;
#[cfg(feature = "async")]
use tokio::sync::Mutex as AsyncMutex;

#[cfg(feature = "async")]
use crate::client::ClientConfig;
use crate::request::{ConnectionParams, OwnedConnectionParams, ParsedRequest};
#[cfg(feature = "async")]
use crate::Response;
use crate::{Error, Method, ResponseLazy};

type UnsecuredStream = TcpStream;

#[cfg(feature = "rustls")]
pub(crate) mod certificates;
#[cfg(feature = "rustls")]
mod rustls_stream;
#[cfg(feature = "rustls")]
type SecuredStream = rustls_stream::SecuredStream;

pub(crate) enum HttpStream {
    Unsecured(UnsecuredStream, Option<Instant>),
    #[cfg(feature = "rustls")]
    Secured(Box<SecuredStream>, Option<Instant>),
    #[cfg(feature = "async")]
    Buffer(std::io::Cursor<Vec<u8>>),
}

impl HttpStream {
    fn create_unsecured(reader: UnsecuredStream, timeout_at: Option<Instant>) -> HttpStream {
        HttpStream::Unsecured(reader, timeout_at)
    }

    #[cfg(feature = "async")]
    pub(crate) fn create_buffer(buffer: Vec<u8>) -> HttpStream {
        HttpStream::Buffer(std::io::Cursor::new(buffer))
    }
}

fn timeout_err() -> io::Error {
    io::Error::new(io::ErrorKind::TimedOut, "the timeout of the request was reached")
}

fn timeout_at_to_duration(timeout_at: Option<Instant>) -> Result<Option<Duration>, io::Error> {
    if let Some(timeout_at) = timeout_at {
        if let Some(duration) = timeout_at.checked_duration_since(Instant::now()) {
            Ok(Some(duration))
        } else {
            Err(timeout_err())
        }
    } else {
        Ok(None)
    }
}

impl Read for HttpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let timeout = |tcp: &TcpStream, timeout_at: Option<Instant>| -> io::Result<()> {
            let _ = tcp.set_read_timeout(timeout_at_to_duration(timeout_at)?);
            Ok(())
        };

        let result = match self {
            HttpStream::Unsecured(inner, timeout_at) => {
                timeout(inner, *timeout_at)?;
                inner.read(buf)
            }
            #[cfg(feature = "rustls")]
            HttpStream::Secured(inner, timeout_at) => {
                timeout(inner.get_ref(), *timeout_at)?;
                inner.read(buf)
            }
            #[cfg(feature = "async")]
            HttpStream::Buffer(cursor) => std::io::Read::read(cursor, buf),
        };
        match result {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // We're a blocking socket, so EWOULDBLOCK indicates a timeout
                Err(timeout_err())
            }
            r => r,
        }
    }
}

fn set_socket_write_timeout(tcp: &TcpStream, timeout_at: Option<Instant>) -> io::Result<()> {
    tcp.set_write_timeout(timeout_at_to_duration(timeout_at)?)?;
    Ok(())
}

impl Write for HttpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = match self {
            HttpStream::Unsecured(inner, timeout_at) => {
                set_socket_write_timeout(inner, *timeout_at)?;
                inner.write(buf)
            }
            #[cfg(feature = "rustls")]
            HttpStream::Secured(inner, timeout_at) => {
                set_socket_write_timeout(inner.get_ref(), *timeout_at)?;
                inner.write(buf)
            }
            #[cfg(feature = "async")]
            HttpStream::Buffer(_) => {
                debug_assert!(false, "We shouldn't write to a pre-loaded stream");
                Ok(buf.len())
            }
        };
        match result {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // We're a blocking socket, so EWOULDBLOCK indicates a timeout
                Err(timeout_err())
            }
            r => r,
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let result = match self {
            HttpStream::Unsecured(inner, timeout_at) => {
                set_socket_write_timeout(inner, *timeout_at)?;
                inner.flush()
            }
            #[cfg(feature = "rustls")]
            HttpStream::Secured(inner, timeout_at) => {
                set_socket_write_timeout(inner.get_ref(), *timeout_at)?;
                inner.flush()
            }
            #[cfg(feature = "async")]
            HttpStream::Buffer(_) => {
                debug_assert!(false, "We shouldn't write to a pre-loaded stream");
                Ok(())
            }
        };
        match result {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // We're a blocking socket, so EWOULDBLOCK indicates a timeout
                Err(timeout_err())
            }
            r => r,
        }
    }
}

#[cfg(feature = "tokio-rustls")]
type AsyncSecuredStream = rustls_stream::AsyncSecuredStream;

#[cfg(feature = "async")]
pub(crate) enum AsyncHttpStream {
    Unsecured(AsyncTcpStream),
    #[cfg(feature = "tokio-rustls")]
    Secured(Box<AsyncSecuredStream>),
}

#[cfg(feature = "async")]
impl AsyncRead for AsyncHttpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match &mut *self {
            AsyncHttpStream::Unsecured(inner) => Pin::new(inner).poll_read(cx, buf),
            #[cfg(feature = "tokio-rustls")]
            AsyncHttpStream::Secured(inner) => Pin::new(inner).poll_read(cx, buf),
        }
    }
}

#[cfg(feature = "async")]
impl AsyncWrite for AsyncHttpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            AsyncHttpStream::Unsecured(inner) => Pin::new(inner).poll_write(cx, buf),
            #[cfg(feature = "tokio-rustls")]
            AsyncHttpStream::Secured(inner) => Pin::new(inner).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            AsyncHttpStream::Unsecured(inner) => Pin::new(inner).poll_flush(cx),
            #[cfg(feature = "tokio-rustls")]
            AsyncHttpStream::Secured(inner) => Pin::new(inner).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            AsyncHttpStream::Unsecured(inner) => Pin::new(inner).poll_shutdown(cx),
            #[cfg(feature = "tokio-rustls")]
            AsyncHttpStream::Secured(inner) => Pin::new(inner).poll_shutdown(cx),
        }
    }
}

#[cfg(feature = "async")]
struct AsyncConnectionState {
    write: AsyncMutex<WriteHalf<AsyncHttpStream>>,
    read: AsyncMutex<ReadHalf<AsyncHttpStream>>,
    /// The ID of the next request we'll send. If this reaches [`usize::MAX`] no further requests
    /// can be sent on this socket and a new connection must be made. Thus, in order to limit the
    /// connection to sending N new requests, this may be set to [`usize::MAX`] - N.
    next_request_id: AtomicUsize,
    /// The ID of the next request which is readable from the socket. If we're pipelining this may
    /// be a few behind [`Self::next_request_id`]. If this is [`usize::MAX`], the socket is in an
    /// indeterminate state and no further reading is allowed. Any pending requests must either be
    /// retried or failed.
    readable_request_id: AtomicUsize,
    /// If we have a pipelined request which has the requesting future dropped, we won't finish
    /// reading the response and thus later responses will need to be retried over a fresh
    /// connection.
    /// Here we track the minimum request_id for which the future was dropped without finishing our
    /// read - if we go to read a request_id higher than this we have to abort and retry.
    min_dropped_reader_id: AtomicUsize,
    /// The time at which we should stop sending new requests over this socket and should instead
    /// connect again.
    /// Defaults to 60 seconds after open to align with nginx's default timeout of 75 seconds, but
    /// can be overridden by the `Keep-Alive` header.
    socket_new_requests_timeout: Mutex<Instant>,
}

#[cfg(feature = "async")]
struct PendingReader<'a> {
    min_dropped_reader_id: &'a AtomicUsize,
    id: Option<usize>,
}

#[cfg(feature = "async")]
impl<'a> Drop for PendingReader<'a> {
    fn drop(&mut self) {
        if let Some(reader_id) = self.id {
            self.min_dropped_reader_id.fetch_min(reader_id, Ordering::AcqRel);
        }
    }
}

/// An async connection to the server for sending
/// [`Request`](struct.Request.html)s.
#[cfg(feature = "async")]
pub struct AsyncConnection(Mutex<Arc<AsyncConnectionState>>);

#[cfg(feature = "async")]
impl AsyncConnection {
    /// Creates a new `AsyncConnection`.
    pub(crate) async fn new(
        params: ConnectionParams<'_>,
        timeout_at: Option<Instant>,
        client_config: Option<ClientConfig>,
    ) -> Result<AsyncConnection, Error> {
        let future = async move {
            let socket = Self::connect(params).await?;

            if params.https {
                Self::wrap_async_stream(socket, params.host, client_config).await
            } else {
                Ok(AsyncHttpStream::Unsecured(socket))
            }
        };
        let stream = if let Some(timeout_at) = timeout_at {
            tokio::time::timeout_at(timeout_at.into(), future)
                .await
                .unwrap_or(Err(Error::IoError(timeout_err())))?
        } else {
            future.await?
        };
        let (read, write) = tokio::io::split(stream);

        Ok(AsyncConnection(Mutex::new(Arc::new(AsyncConnectionState {
            read: AsyncMutex::new(read),
            write: AsyncMutex::new(write),
            next_request_id: AtomicUsize::new(0),
            readable_request_id: AtomicUsize::new(0),
            min_dropped_reader_id: AtomicUsize::new(usize::MAX),
            socket_new_requests_timeout: Mutex::new(Instant::now() + Duration::from_secs(60)),
        }))))
    }

    /// Call the correct wrapper function depending on whether client_configs are present
    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    async fn wrap_async_stream(
        socket: AsyncTcpStream,
        host: &str,
        client_config: Option<ClientConfig>,
    ) -> Result<AsyncHttpStream, Error> {
        if let Some(client_config) = client_config {
            rustls_stream::wrap_async_stream_with_configs(socket, host, client_config).await
        } else {
            rustls_stream::wrap_async_stream(socket, host).await
        }
    }

    /// Error treatment function, should not be called under normal circustances
    #[cfg(not(all(feature = "rustls", feature = "tokio-rustls")))]
    async fn wrap_async_stream(
        _socket: AsyncTcpStream,
        _host: &str,
        _client_config: Option<ClientConfig>,
    ) -> Result<AsyncHttpStream, Error> {
        Err(Error::HttpsFeatureNotEnabled)
    }

    async fn tcp_connect(host: &str, port: u16) -> Result<AsyncTcpStream, Error> {
        #[cfg(feature = "log")]
        log::trace!("Looking up host {host}");

        let addrs = tokio::net::lookup_host((host, port)).await.map_err(Error::IoError)?;
        let addrs: Vec<_> = addrs.collect();
        let addrs_count = addrs.len();

        if addrs.is_empty() {
            return Err(Error::AddressNotFound);
        }

        // Try all resolved addresses. Return the first one to which we could connect. If all
        // failed return the last error encountered.
        for (i, addr) in addrs.iter().enumerate() {
            #[cfg(feature = "log")]
            log::trace!("Attempting to connect to {addr} for {host}");

            match AsyncTcpStream::connect(addr).await {
                Ok(s) => {
                    #[cfg(feature = "log")]
                    log::trace!("Connected to {addr} for {host}");
                    return Ok(s);
                }
                Err(e) =>
                    if i == addrs_count - 1 {
                        return Err(Error::IoError(e));
                    },
            }
        }

        Err(Error::AddressNotFound)
    }

    /// Asynchronously connect to the server.
    async fn connect(params: ConnectionParams<'_>) -> Result<AsyncTcpStream, Error> {
        #[cfg(feature = "proxy")]
        match &params.proxy {
            Some(proxy) => {
                // do proxy things
                let mut tcp = Self::tcp_connect(&proxy.server, proxy.port).await?;

                let proxy_request = proxy.connect(params.host, params.port.port());
                tcp.write_all(proxy_request.as_bytes()).await?;
                tcp.flush().await?;

                // Max proxy response size to prevent unbounded memory allocation
                const MAX_PROXY_RESPONSE_SIZE: usize = 16 * 1024;
                let mut proxy_response = Vec::new();
                let mut buf = [0; 256];

                loop {
                    let n = tcp.read(&mut buf).await?;
                    if n == 0 {
                        // EOF reached
                        break;
                    }
                    proxy_response.extend_from_slice(&buf[..n]);
                    if proxy_response.len() > MAX_PROXY_RESPONSE_SIZE {
                        return Err(Error::ProxyConnect);
                    }
                    if n < buf.len() {
                        // Partial read indicates end of response
                        break;
                    }
                }

                crate::Proxy::verify_response(&proxy_response)?;

                Ok(tcp)
            }
            None => Self::tcp_connect(params.host, params.port.port()).await,
        }

        #[cfg(not(feature = "proxy"))]
        Self::tcp_connect(&params.host, params.port.port()).await
    }

    async fn timeout<O, F: Future<Output = O>>(timeout: Option<Instant>, f: F) -> Result<O, Error> {
        if let Some(time) = timeout {
            tokio::time::timeout_at(time.into(), f).await.map_err(|_| Error::IoError(timeout_err()))
        } else {
            Ok(f.await)
        }
    }

    /// Sends the [`Request`](struct.Request.html) asynchronously using HTTP.
    pub(crate) fn send<'a>(
        &'a self,
        request: ParsedRequest,
    ) -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send + 'a>> {
        Box::pin(async move {
            let conn = Arc::clone(&*self.0.lock().unwrap());
            #[cfg(debug_assertions)]
            {
                let next_read = conn.readable_request_id.load(Ordering::Acquire);
                let next_request = conn.next_request_id.load(Ordering::Acquire);
                debug_assert!(
                    next_request >= next_read,
                    "At all times, the next_request_id should be higher than the readable id"
                );
            }

            // Note that we do not have a top-level timeout as we need to handle timeouts by
            // resetting the socket state to ensure no other requests try to read the response to
            // our request or reuse the socket at all if we leave it in an indeterminate state.
            // Instead, we have to manually time out all `await`s and, after we write to the
            // socket, handle error explicitly.

            let mut read = None;
            let mut write = None;
            if !request.config.pipelining {
                // If we're not pipelining, wait for any existing pipelined requests to complete.
                // Specifically, wait until we have both locks and either we're going to build a
                // new connection (because `next_request_id` is `usize::MAX`) or there are no
                // pending readers (because `next_request_id` and `readable_request_id` are the
                // same).
                read = Some(Self::timeout(request.timeout_at, conn.read.lock()).await?);
                write = Some(Self::timeout(request.timeout_at, conn.write.lock()).await?);
                while {
                    let next_read = conn.readable_request_id.load(Ordering::Relaxed);
                    let next_request = conn.next_request_id.load(Ordering::Relaxed);
                    next_request != usize::MAX && next_read < next_request
                } {
                    read.take();
                    write.take();
                    tokio::task::yield_now().await;
                    read = Some(Self::timeout(request.timeout_at, conn.read.lock()).await?);
                    write = Some(Self::timeout(request.timeout_at, conn.write.lock()).await?);
                }
            }

            macro_rules! retry_new_connection {
                (CONNECTION_STATE_UNDEFINED) => {
                    // The connection may next return bytes for a request which timed out, thus no
                    // more reads are allowed.
                    conn.next_request_id.store(usize::MAX, Ordering::Release);
                    conn.readable_request_id.store(usize::MAX, Ordering::Release);
                    retry_new_connection!(_internal);
                };
                (CONNECTION_STILL_READABLE, $write_lock: ident) => {
                    // Make sure new requests don't try to use the old connection (but allow
                    // requests that have already been sent to continue trying to read from it).
                    conn.next_request_id.store(usize::MAX, Ordering::Release);
                    core::mem::drop($write_lock);
                    retry_new_connection!(_internal);
                };
                (_internal) => {
                    let new_connection =
                        AsyncConnection::new(request.connection_params(), request.timeout_at, None)
                            .await?;
                    *self.0.lock().unwrap() = Arc::clone(&*new_connection.0.lock().unwrap());
                    core::mem::drop(read);
                    // Note that this cannot recurse infinitely as we'll always be able to send at
                    // least one request on the new socket (though some other request may race us
                    // and go first).
                    return self.send(request).await;
                };
            }

            let request_id;
            let mut this_request =
                PendingReader { min_dropped_reader_id: &conn.min_dropped_reader_id, id: None };
            {
                let mut write = if let Some(write) = write {
                    write
                } else {
                    Self::timeout(request.timeout_at, conn.write.lock()).await?
                };

                let socket_timeout = *conn.socket_new_requests_timeout.lock().unwrap();
                let socket_timed_out = Instant::now() > socket_timeout;

                request_id = conn.next_request_id.fetch_add(1, Ordering::Relaxed);
                if request_id == usize::MAX || socket_timed_out {
                    // We can't send additional requests on the socket or the socket timed out and
                    // need to resend the request on a new connection.
                    retry_new_connection!(CONNECTION_STILL_READABLE, write);
                }
                #[cfg(feature = "log")]
                log::trace!(
                    "Writing HTTP request id {request_id} on connection to {:?}.",
                    request.connection_params(),
                );
                this_request.id = Some(request_id);
                let write_res =
                    Self::timeout(request.timeout_at, write.write_all(&request.as_bytes())).await;
                match write_res {
                    Err(e) => {
                        // If we failed to write the request, mark the socket as dead for future
                        // requests.
                        conn.next_request_id.store(usize::MAX, Ordering::Release);
                        return Err(e);
                    }
                    Ok(Err(ioe)) => {
                        conn.next_request_id.store(usize::MAX, Ordering::Release);
                        return Err(Error::IoError(ioe));
                    }
                    Ok(Ok(())) => {}
                }
            }

            let mut should_retry = false;
            let response = Self::timeout(request.timeout_at, async {
                if read.is_none() {
                    read = Some(Self::timeout(request.timeout_at, conn.read.lock()).await?);
                }

                while {
                    let readable = conn.readable_request_id.load(Ordering::Acquire);
                    if readable == usize::MAX {
                        // We got a `Connection: close` before our pipelined request could be handled
                        // and need to retry on a new connection.
                        debug_assert!(
                            request.config.pipelining,
                            "We should never need to re-send a non-pipelined request (as both locks were held and no other pending requests were in-flight)",
                        );
                        should_retry = true;
                        return Err(Error::Other("Retrying pipelining failure"));
                    }
                    readable < request_id
                } {
                    // There's a race where we can finish writing but see a context switch between
                    // dropping the write lock and getting to the waiter that can lead to waiters being
                    // registered out of order. Thus, if we're not actually ready to read, wake another
                    // waiter and see if we're ready when we get the semaphore back.
                    debug_assert!(
                        request.config.pipelining,
                        "Non-pipelined requests should never need to wait as both locks were held and no other requests were in-filght"
                    );
                    read.take();
                    if conn.min_dropped_reader_id.load(Ordering::Acquire) < request_id {
                        // An earlier reader was dropped before we could finish reading the
                        // response. Thus we need to retry on a fresh connection.
                        should_retry = true;
                        return Err(Error::Other("Retrying pipelining failure"));
                    }
                    tokio::task::yield_now().await;
                    read = Some(conn.read.lock().await);
                }
                let mut read = read.take().unwrap();

                // Receive response
                #[cfg(feature = "log")]
                log::trace!(
                    "Reading HTTP response for request id {request_id} on connection to {:?}.",
                    request.connection_params(),
                );

                let response = Response::create_async(
                    &mut *read,
                    request.config.method == Method::Head,
                    request.config.max_headers_size,
                    request.config.max_status_line_len,
                    request.config.max_body_size,
                )
                .await?;

                let mut found_keep_alive = false;
                if let Some(header) = response.headers.get("connection") {
                    if header.eq_ignore_ascii_case("keep-alive") {
                        found_keep_alive = true;
                    }
                }
                if !found_keep_alive {
                    conn.next_request_id.store(usize::MAX, Ordering::Release);
                    conn.readable_request_id.store(usize::MAX, Ordering::Release);
                } else {
                    conn.readable_request_id.fetch_add(1, Ordering::Release);
                }

                if let Some(header) = response.headers.get("keep-alive") {
                    for param in header.split(',') {
                        if let Some((k, v)) = param.trim().split_once('=') {
                            if let Ok(v) = v.parse::<usize>() {
                                match k.trim() {
                                    "timeout" => {
                                        let timeout_secs = (v as u64).saturating_sub(1);
                                        *conn.socket_new_requests_timeout.lock().unwrap() =
                                            Instant::now()
                                                .checked_add(Duration::from_secs(timeout_secs))
                                                .unwrap_or(Instant::now());
                                    }
                                    "max" => {
                                        conn.next_request_id.fetch_max(
                                            usize::MAX.saturating_sub(v),
                                            Ordering::AcqRel,
                                        );
                                    }
                                    _ => {
                                        // If we can't parse the keep-alive header, don't send any
                                        // new requests over this socket, but don't give up on
                                        // reading pending responses.
                                        conn.next_request_id.store(usize::MAX, Ordering::Release);
                                    }
                                }
                            } else {
                                // If we can't parse the keep-alive header, don't send any new
                                // requests over this socket, but don't give up on reading pending
                                // responses.
                                conn.next_request_id.store(usize::MAX, Ordering::Release);
                            }
                        } else {
                            // If we can't parse the keep-alive header, don't send any new requests
                            // over this socket, but don't give up on reading pending responses.
                            conn.next_request_id.store(usize::MAX, Ordering::Release);
                        }
                    }
                }

                // Now that we've processed the response, if the future is cancelled there's no
                // need to kill the connection
                this_request.id = None;

                Ok(response)
            })
            .await;
            let response = match response {
                Ok(Ok(response)) => response,
                Err(e) | Ok(Err(e)) => {
                    if should_retry {
                        retry_new_connection!(CONNECTION_STATE_UNDEFINED);
                    } else {
                        // If we failed to read the response after reading the request, the socket
                        // is in an indeterminate state. Thus, we have to force every other waiting
                        // request to retry on a new socket.
                        conn.next_request_id.store(usize::MAX, Ordering::Release);
                        conn.readable_request_id.store(usize::MAX, Ordering::Relaxed);
                        return Err(e);
                    }
                }
            };

            core::mem::drop(read);
            async_handle_redirects(self, request, response).await
        })
    }
}

/// A connection to the server for sending
/// [`Request`](struct.Request.html)s.
pub struct Connection {
    stream: HttpStream,
}

impl Connection {
    /// Creates a new `Connection`. See [Request] and [ParsedRequest]
    /// for specifics about *what* is being sent.
    pub(crate) fn new(
        params: ConnectionParams<'_>,
        timeout_at: Option<Instant>,
    ) -> Result<Connection, Error> {
        let socket = Self::connect(params, timeout_at)?;

        let stream = if params.https {
            #[cfg(not(feature = "rustls"))]
            return Err(Error::HttpsFeatureNotEnabled);
            #[cfg(feature = "rustls")]
            {
                let tls = rustls_stream::wrap_stream(socket, params.host)?;
                HttpStream::Secured(Box::new(tls), timeout_at)
            }
        } else {
            HttpStream::create_unsecured(socket, timeout_at)
        };

        Ok(Connection { stream })
    }

    fn tcp_connect(host: &str, port: u16, timeout_at: Option<Instant>) -> Result<TcpStream, Error> {
        #[cfg(feature = "log")]
        log::trace!("Looking up host {host}");

        let addrs = (host, port).to_socket_addrs().map_err(Error::IoError)?;
        let addrs_count = addrs.len();

        // Try all resolved addresses. Return the first one to which we could connect. If all
        // failed return the last error encountered.
        for (i, addr) in addrs.enumerate() {
            #[cfg(feature = "log")]
            log::trace!("Attempting to connect to {addr} for {host}");

            let stream = if let Some(timeout) = timeout_at_to_duration(timeout_at)? {
                TcpStream::connect_timeout(&addr, timeout)
            } else {
                TcpStream::connect(addr)
            };

            match stream {
                Ok(s) => {
                    #[cfg(feature = "log")]
                    log::trace!("Connected to {addr} for {host}");
                    return Ok(s);
                }
                Err(e) =>
                    if i == addrs_count - 1 {
                        return Err(Error::IoError(e));
                    },
            }
        }

        Err(Error::AddressNotFound)
    }

    /// Connect to the server.
    fn connect(
        params: ConnectionParams<'_>,
        timeout_at: Option<Instant>,
    ) -> Result<TcpStream, Error> {
        #[cfg(feature = "proxy")]
        match &params.proxy {
            Some(proxy) => {
                // do proxy things
                let mut tcp = Self::tcp_connect(&proxy.server, proxy.port, timeout_at)?;

                write!(tcp, "{}", proxy.connect(params.host, params.port.port()))?;
                tcp.flush()?;

                // Max proxy response size to prevent unbounded memory allocation
                const MAX_PROXY_RESPONSE_SIZE: usize = 16 * 1024;
                let mut proxy_response = Vec::new();
                let mut buf = [0; 256];

                loop {
                    let n = tcp.read(&mut buf)?;
                    if n == 0 {
                        // EOF reached
                        break;
                    }
                    proxy_response.extend_from_slice(&buf[..n]);
                    if proxy_response.len() > MAX_PROXY_RESPONSE_SIZE {
                        return Err(Error::ProxyConnect);
                    }
                    if n < buf.len() {
                        // Partial read indicates end of response
                        break;
                    }
                }

                crate::Proxy::verify_response(&proxy_response)?;

                Ok(tcp)
            }
            None => Self::tcp_connect(params.host, params.port.port(), timeout_at),
        }

        #[cfg(not(feature = "proxy"))]
        Self::tcp_connect(params.host, params.port.port(), timeout_at)
    }

    /// Sends the [`Request`](struct.Request.html), consumes this
    /// connection, and returns a [`Response`](struct.Response.html).
    pub(crate) fn send(mut self, request: ParsedRequest) -> Result<ResponseLazy, Error> {
        enforce_timeout(request.timeout_at, move || {
            // Send request
            #[cfg(feature = "log")]
            log::trace!("Writing HTTP request.");
            self.stream.write_all(&request.as_bytes())?;

            // Receive response
            #[cfg(feature = "log")]
            log::trace!("Reading HTTP response.");
            let response = ResponseLazy::from_stream(
                self.stream,
                request.config.max_headers_size,
                request.config.max_status_line_len,
                request.config.max_body_size,
            )?;
            handle_redirects(request, response)
        })
    }
}

fn handle_redirects(
    request: ParsedRequest,
    mut response: ResponseLazy,
) -> Result<ResponseLazy, Error> {
    let status_code = response.status_code;
    let url = response.headers.get("location");
    match get_redirect(request, status_code, url) {
        NextHop::Redirect(request) => {
            let (request, _) = request?;
            Connection::new(request.connection_params(), request.timeout_at)?.send(request)
        }
        NextHop::Destination(request) => {
            let dst_url = request.url;
            dst_url.write_base_url_to(&mut response.url).unwrap();
            dst_url.write_resource_to(&mut response.url).unwrap();
            Ok(response)
        }
    }
}

#[cfg(feature = "async")]
async fn async_handle_redirects(
    connection: &AsyncConnection,
    request: ParsedRequest,
    mut response: Response,
) -> Result<Response, Error> {
    let status_code = response.status_code;
    let url = response.headers.get("location");
    match async_get_redirect(request, status_code, url) {
        NextHopAsync::Redirect(request) => {
            let (request, needs_new_connection) = request?;
            let mut connection = connection;
            let new_connection;
            if needs_new_connection {
                new_connection =
                    AsyncConnection::new(request.connection_params(), request.timeout_at, None)
                        .await?;
                connection = &new_connection;
            }
            connection.send(request).await
        }
        NextHopAsync::Destination(request) => {
            let dst_url = request.url;
            dst_url.write_base_url_to(&mut response.url).unwrap();
            dst_url.write_resource_to(&mut response.url).unwrap();
            Ok(response)
        }
    }
}

macro_rules! redirect_utils {
    ($get_redirect: ident, $NextHop: ident, $Response: ident) => {
        enum $NextHop {
            Redirect(Result<(ParsedRequest, bool), Error>),
            Destination(ParsedRequest),
        }

        fn $get_redirect(
            mut request: ParsedRequest,
            status_code: i32,
            url: Option<&String>,
        ) -> $NextHop {
            match status_code {
                301 | 302 | 303 | 307 => {
                    let url = match url {
                        Some(url) => url,
                        None => return $NextHop::Redirect(Err(Error::RedirectLocationMissing)),
                    };
                    #[cfg(feature = "log")]
                    log::debug!("Redirecting ({}) to: {}", status_code, url);

                    // TODO: Do this check without allocating a whole new params object
                    let previous_params: OwnedConnectionParams = request.connection_params().into();

                    match request.redirect_to(url.as_str()) {
                        Ok(()) => {
                            if status_code == 303 {
                                match request.config.method {
                                    Method::Post | Method::Put | Method::Delete => {
                                        request.config.method = Method::Get;
                                    }
                                    _ => {}
                                }
                            }

                            let needs_new_conn = previous_params != request.connection_params();
                            $NextHop::Redirect(Ok((request, needs_new_conn)))
                        }
                        Err(err) => $NextHop::Redirect(Err(err)),
                    }
                }
                _ => $NextHop::Destination(request),
            }
        }
    };
}

redirect_utils!(get_redirect, NextHop, ResponseLazy);
#[cfg(feature = "async")]
redirect_utils!(async_get_redirect, NextHopAsync, Response);

/// Enforce the timeout by running the function in a new thread and
/// parking the current one with a timeout.
///
/// While bitreq does use timeouts (somewhat) properly, some
/// interfaces such as [ToSocketAddrs] don't allow for specifying the
/// timeout. Hence this.
fn enforce_timeout<F, R>(timeout_at: Option<Instant>, f: F) -> Result<R, Error>
where
    F: 'static + Send + FnOnce() -> Result<R, Error>,
    R: 'static + Send,
{
    use std::sync::mpsc::{channel, RecvTimeoutError};

    match timeout_at {
        Some(deadline) => {
            let (sender, receiver) = channel();
            let thread = std::thread::spawn(move || {
                let result = f();
                let _ = sender.send(());
                result
            });
            if let Some(timeout_duration) = deadline.checked_duration_since(Instant::now()) {
                match receiver.recv_timeout(timeout_duration) {
                    Ok(()) => thread.join().unwrap(),
                    Err(err) => match err {
                        RecvTimeoutError::Timeout => Err(Error::IoError(timeout_err())),
                        RecvTimeoutError::Disconnected =>
                            Err(Error::Other("request connection paniced")),
                    },
                }
            } else {
                Err(Error::IoError(timeout_err()))
            }
        }
        None => f(),
    }
}
