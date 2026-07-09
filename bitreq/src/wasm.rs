// This module is adapted from reqwest's wasm backend:
// https://github.com/seanmonstar/reqwest

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use core::cell::Cell;
use core::time::Duration;
use std::io;

use js_sys::{Function, Promise, Uint8Array};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::{wasm_bindgen, UnwrapThrowExt};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AbortController, AbortSignal, Headers, ReadableStreamDefaultReader, RequestInit};

use crate::request::{Method, ParsedRequest};
use crate::{Error, Response};

fn wasm_error(value: JsValue) -> Error { Error::Wasm(format!("{value:?}")) }

fn timeout_error() -> Error {
    Error::IoError(io::Error::new(
        io::ErrorKind::TimedOut,
        "the timeout of the request was reached",
    ))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setTimeout")]
    fn set_timeout(handler: &Function, timeout: i32) -> JsValue;

    #[wasm_bindgen(js_name = "clearTimeout")]
    fn clear_timeout(handle: JsValue) -> JsValue;
}

async fn promise<T, F>(promise: js_sys::Promise, map_rejection: F) -> Result<T, Error>
where
    T: JsCast,
    F: FnOnce(JsValue) -> Error,
{
    let js_val = wasm_bindgen_futures::JsFuture::from(promise).await.map_err(map_rejection)?;

    js_val
        .dyn_into::<T>()
        .map_err(|_js_val| Error::Wasm(String::from("promise resolved to unexpected type")))
}

struct AbortGuard {
    ctrl: AbortController,
    timed_out: Rc<Cell<bool>>,
    timeout: Option<(JsValue, Closure<dyn FnMut()>)>,
}

impl AbortGuard {
    fn new() -> Result<Self, Error> {
        Ok(AbortGuard {
            ctrl: AbortController::new().map_err(wasm_error)?,
            timed_out: Rc::new(Cell::new(false)),
            timeout: None,
        })
    }

    fn signal(&self) -> AbortSignal { self.ctrl.signal() }

    fn timeout(&mut self, timeout: Duration) {
        let ctrl = self.ctrl.clone();
        let timed_out = Rc::clone(&self.timed_out);
        let abort = Closure::once(move || {
            timed_out.set(true);
            ctrl.abort();
        });
        let timeout = set_timeout(
            abort.as_ref().unchecked_ref::<js_sys::Function>(),
            timeout.as_millis().try_into().unwrap_or(i32::MAX),
        );
        if let Some((id, _)) = self.timeout.replace((timeout, abort)) {
            clear_timeout(id);
        }
    }

    fn map_rejection(&self, value: JsValue) -> Error {
        if self.timed_out.get() {
            timeout_error()
        } else {
            wasm_error(value)
        }
    }
}

impl Drop for AbortGuard {
    fn drop(&mut self) {
        self.ctrl.abort();
        if let Some((id, _)) = self.timeout.take() {
            clear_timeout(id);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch)]
    fn fetch_with_request(input: &web_sys::Request) -> Promise;
}

fn js_fetch(req: &web_sys::Request) -> Promise {
    let global = js_sys::global();

    if let Some(scope) = global.dyn_ref::<web_sys::ServiceWorkerGlobalScope>() {
        scope.fetch_with_request(req)
    } else {
        fetch_with_request(req)
    }
}

async fn read_body(
    response: &web_sys::Response,
    max_body_size: Option<usize>,
    abort: &AbortGuard,
) -> Result<Vec<u8>, Error> {
    let Some(body) = response.body() else {
        return Ok(Vec::new());
    };

    let reader: ReadableStreamDefaultReader = body
        .get_reader()
        .dyn_into()
        .map_err(|_| Error::Wasm(String::from("response body reader is unexpected type")))?;

    let done_key = JsValue::from_str("done");
    let value_key = JsValue::from_str("value");
    let mut bytes = Vec::new();
    loop {
        let item: js_sys::Object =
            promise(reader.read(), |value| abort.map_rejection(value)).await?;
        let done = js_sys::Reflect::get(item.as_ref(), &done_key)
            .map_err(wasm_error)?
            .as_bool()
            .unwrap_or(false);
        if done {
            break;
        }

        let chunk: Uint8Array = js_sys::Reflect::get(item.as_ref(), &value_key)
            .map_err(wasm_error)?
            .dyn_into()
            .map_err(|_| Error::Wasm(String::from("response body chunk is unexpected type")))?;
        let chunk_len = chunk.length() as usize;
        if max_body_size.is_some_and(|max| bytes.len().saturating_add(chunk_len) > max) {
            return Err(Error::BodyOverflow);
        }

        let offset = bytes.len();
        bytes.resize(offset + chunk_len, 0);
        chunk.copy_to(&mut bytes[offset..]);
    }

    reader.release_lock();
    Ok(bytes)
}

pub(crate) async fn send(request: ParsedRequest) -> Result<Response, Error> {
    let init = RequestInit::new();
    init.set_method(&request.config.method.to_string());

    let js_headers = Headers::new().map_err(wasm_error)?;
    for (key, value) in &request.config.headers {
        js_headers.append(key, value).map_err(wasm_error)?;
    }
    init.set_headers(&js_headers);

    if let Some(body) = request.config.body {
        if !body.is_empty() {
            init.set_body(&JsValue::from(body));
        }
    }

    let mut abort = AbortGuard::new()?;
    if let Some(timeout) = request.timeout {
        abort.timeout(timeout);
    }
    init.set_signal(Some(&abort.signal()));

    let js_req =
        web_sys::Request::new_with_str_and_init(request.url.as_str(), &init).map_err(wasm_error)?;
    let p = js_fetch(&js_req);
    let response: web_sys::Response = promise(p, |value| abort.map_rejection(value)).await?;

    let status_code = i32::from(response.status());
    let reason_phrase = response.status_text();
    let url = response.url();
    let js_headers = response.headers();
    let mut remaining_headers_size = request.config.max_headers_size;
    let mut headers = BTreeMap::new();
    for item in js_headers.entries() {
        let item = item.expect_throw("headers iterator doesn't throw");
        let item: js_sys::Array = item.dyn_into().expect_throw("header item is an array");

        let name = item.get(0).as_string().expect_throw("header name is a string");

        let value = item.get(1).as_string().expect_throw("header value is a string");

        if let Some(remaining) = remaining_headers_size.as_mut() {
            let header_size = name.len().saturating_add(value.len()).saturating_add(4);
            if header_size > *remaining {
                return Err(Error::HeadersOverflow);
            }
            *remaining -= header_size;
        }

        headers.insert(name, value);
    }

    let should_read_body =
        request.config.method != Method::Head && status_code != 204 && status_code != 304;

    let bytes = if should_read_body {
        read_body(&response, request.config.max_body_size, &abort).await?
    } else {
        Vec::new()
    };

    Ok(Response::from_parts(status_code, reason_phrase, headers, url, bytes))
}
