// SPDX-License-Identifier: CC0-1.0

//! Integration-style tests for the async production client driven by an in-process stub
//! transport.
//!
//! These tests exist to verify the end-to-end shape of the async client — argument encoding,
//! method dispatch, error mapping, and the `into_model()` boundary — without needing a real
//! `bitcoind`. Real integration tests against `bitcoind` belong in the workspace's
//! `integration_test` crate; the l here is to give the production client a fast, hermetic
//! safety net for review.

#![cfg(feature = "client-async")]

use std::fmt;
use std::sync::Mutex;

use corepc_client::client_async::{Client, Error, Retryability};
use jsonrpc::client_async::{BoxFuture, Transport};
use jsonrpc::error::RpcError;
use jsonrpc::{Request, Response};
use serde_json::value::RawValue;

/// JSON-RPC layer Result alias used by the stub transport.
type StubResult<T> = std::result::Result<T, jsonrpc::Error>;
/// Boxed handler type used by the stub. Factored out because clippy flags the inline form.
type StubHandler = Box<dyn FnMut(&str, Option<&str>) -> StubResult<Response> + Send + 'static>;

/// A controllable in-process [`Transport`] implementation. Each test installs a closure that
/// receives the inbound `Request` and returns the `Response` (or a transport-level error).
struct StubTransport {
    handler: Mutex<StubHandler>,
    last_request: Mutex<Option<(String, Option<String>)>>,
}

impl StubTransport {
    fn new<F>(f: F) -> Self
    where
        F: FnMut(&str, Option<&str>) -> StubResult<Response> + Send + 'static,
    {
        Self { handler: Mutex::new(Box::new(f)), last_request: Mutex::new(None) }
    }

    /// Returns the most recent `(method, params_json)` pair the stub saw.
    fn last(&self) -> Option<(String, Option<String>)> { self.last_request.lock().unwrap().clone() }
}

impl Transport for StubTransport {
    fn send_request<'a>(&'a self, req: Request<'a>) -> BoxFuture<'a, StubResult<Response>> {
        let method = req.method.to_owned();
        let params = req.params.map(|r| r.get().to_owned());
        *self.last_request.lock().unwrap() = Some((method.clone(), params.clone()));
        let id = req.id.clone();
        let mut handler = self.handler.lock().unwrap();
        let mut response = match (*handler)(&method, params.as_deref()) {
            Ok(r) => r,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        response.id = id;
        Box::pin(async move { Ok(response) })
    }

    fn send_batch<'a>(&'a self, _: &'a [Request<'a>]) -> BoxFuture<'a, StubResult<Vec<Response>>> {
        Box::pin(async { Ok(vec![]) })
    }

    fn fmt_target(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "stub") }
}

fn ok_response(value: serde_json::Value) -> Response {
    Response {
        result: Some(RawValue::from_string(serde_json::to_string(&value).unwrap()).unwrap()),
        error: None,
        id: serde_json::Value::Null, // overwritten by the stub before send
        jsonrpc: Some("2.0".to_owned()),
    }
}

fn err_response(code: i32, message: &str) -> Response {
    Response {
        result: None,
        error: Some(RpcError { code, message: message.to_owned(), data: None }),
        id: serde_json::Value::Null,
        jsonrpc: Some("2.0".to_owned()),
    }
}

fn build_client_with<F>(handler: F) -> (Client, std::sync::Arc<StubTransport>)
where
    F: FnMut(&str, Option<&str>) -> StubResult<Response> + Send + 'static,
{
    // We need to keep a handle to the stub so tests can introspect it; but `Client::with_transport`
    // takes ownership. Wrap in `Arc<StubTransport>` and implement `Transport` on the `Arc` via
    // a trampoline.
    let stub = std::sync::Arc::new(StubTransport::new(handler));
    let arc_for_client = stub.clone();
    let client = Client::with_transport(ArcStub(arc_for_client));
    (client, stub)
}

struct ArcStub(std::sync::Arc<StubTransport>);

impl Transport for ArcStub {
    fn send_request<'a>(&'a self, req: Request<'a>) -> BoxFuture<'a, StubResult<Response>> {
        self.0.send_request(req)
    }
    fn send_batch<'a>(
        &'a self,
        reqs: &'a [Request<'a>],
    ) -> BoxFuture<'a, StubResult<Vec<Response>>> {
        self.0.send_batch(reqs)
    }
    fn fmt_target(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt_target(f) }
}

#[tokio::test]
async fn get_block_count_deserializes_height() {
    let (client, stub) = build_client_with(|method, _| {
        assert_eq!(method, "getblockcount");
        Ok(ok_response(serde_json::json!(842_017)))
    });

    let count = client.get_block_count().await.unwrap();
    assert_eq!(count.0, 842_017);
    let (method, _params) = stub.last().unwrap();
    assert_eq!(method, "getblockcount");
}

#[tokio::test]
async fn get_block_count_sends_empty_array_params() {
    // bitcoind accepts either an absent `params` field or an empty array; the sync client always
    // sends `[]`, and this client matches that behaviour. Sending an explicit JSON `null` is the
    // shape we deliberately avoid because some Core builds reject it.
    let (client, stub) = build_client_with(|_, params| {
        assert_eq!(params, Some("[]"), "expected empty array, got {:?}", params);
        Ok(ok_response(serde_json::json!(0)))
    });
    let _ = client.get_block_count().await.unwrap();
    let (_, params) = stub.last().unwrap();
    assert_eq!(params.as_deref(), Some("[]"));
}

#[tokio::test]
async fn rpc_application_error_is_classified_as_block_not_found() {
    let (client, _) = build_client_with(|_, _| Ok(err_response(-5, "Block not found")));

    let err = client.get_block_count().await.unwrap_err();
    assert!(err.is_block_not_found(), "expected block-not-found, got {:?}", err);
    assert_eq!(err.retryability(), Retryability::NonRetryable);
    assert_eq!(err.method(), Some("getblockcount"));
}

#[tokio::test]
async fn rpc_warmup_error_is_retryable() {
    let (client, _) = build_client_with(|_, _| Ok(err_response(-28, "Loading block index...")));

    let err = client.get_block_count().await.unwrap_err();
    assert_eq!(err.retryability(), Retryability::Retryable);
    matches_rpc(err, -28);
}

#[tokio::test]
async fn transport_failure_is_retryable() {
    let (client, _) =
        build_client_with(|_, _| Err(jsonrpc::Error::Transport(Box::new(StubTransportFailure))));

    let err = client.get_block_count().await.unwrap_err();
    assert!(matches!(err, Error::Transport(_)), "{:?}", err);
    assert_eq!(err.retryability(), Retryability::Retryable);
}

#[tokio::test]
async fn call_raw_can_invoke_arbitrary_methods() {
    let (client, stub) = build_client_with(|method, params| {
        assert_eq!(method, "uptime");
        assert_eq!(params, Some("[]"));
        Ok(ok_response(serde_json::json!(12_345)))
    });

    let secs: u64 = client.call_raw("uptime", &[(); 0]).await.unwrap();
    assert_eq!(secs, 12_345);
    assert_eq!(stub.last().unwrap().0, "uptime");
}

#[tokio::test]
async fn with_overload_fills_skipped_positional_args_with_null() {
    // Bitcoin Core RPC arguments are positional: `createwallet "name" ( disable_private_keys blank
    // ... )`. To set a *later* optional (here `blank`, arg 3) without an *earlier* one
    // (`disable_private_keys`, arg 2), the slot for the earlier arg cannot be omitted — the wire
    // array would otherwise shift `blank` into the wrong position. The generated `_with` overload
    // therefore emits one positional slot per optional field and serializes an unset (`None`) field
    // as JSON `null`, which Core reads as "argument not provided, use the default". This test pins
    // that contract: set only `blank`, and assert the gap before it is a literal `null`.
    use corepc_client::client_async::v30::CreateWalletOptions;

    let (client, stub) = build_client_with(|method, _| {
        assert_eq!(method, "createwallet");
        Ok(ok_response(serde_json::json!({ "name": "w", "warnings": [] })))
    });

    let opts = CreateWalletOptions { blank: Some(true), ..Default::default() };
    let _ = client.create_wallet_with("w".to_owned(), opts).await.unwrap();

    let (_, params) = stub.last().unwrap();
    let params = params.expect("createwallet must send positional params");
    let arr: Vec<serde_json::Value> = serde_json::from_str(&params).unwrap();

    // wallet_name, then disable_private_keys (skipped -> null), then blank (set -> true), then the
    // remaining unset optionals as null.
    assert_eq!(arr[0], serde_json::json!("w"));
    assert_eq!(arr[1], serde_json::Value::Null, "skipped earlier arg must be null, not absent");
    assert_eq!(arr[2], serde_json::json!(true), "the later arg we set must land in its own slot");
    assert!(
        arr[3..].iter().all(serde_json::Value::is_null),
        "every other unset optional stays null: {arr:?}"
    );
}

#[tokio::test]
async fn with_overload_object_mode_omits_unset_fields() {
    // The other construction mode: when a method's optional parameter is itself a JSON object
    // (`bumpfee "txid" ( options )`), the `_with` overload sends the whole options struct as one
    // trailing object argument, and every field carries `skip_serializing_if = "Option::is_none"`.
    // Here positions are named, not ordered, so setting a "later" field (`replaceable`, the last
    // one) without an "earlier" one (`conf_target`, the first) needs no null padding at all — the
    // unset fields simply do not appear in the object. Pin that: only `replaceable` is sent.
    use corepc_client::client_async::v30::BumpFeeOptions;

    let (client, stub) = build_client_with(|method, _| {
        assert_eq!(method, "bumpfee");
        Ok(ok_response(
            serde_json::json!({ "txid": "00", "origfee": 0.0, "fee": 0.0, "errors": [] }),
        ))
    });

    let opts = BumpFeeOptions { replaceable: Some(true), ..Default::default() };
    let _ = client.bump_fee_with("abc".to_owned(), opts).await.unwrap();

    let (_, params) = stub.last().unwrap();
    let arr: Vec<serde_json::Value> = serde_json::from_str(&params.unwrap()).unwrap();
    assert_eq!(arr[0], serde_json::json!("abc"));
    // The options object holds only the field we set; the unset earlier fields are absent (not null).
    assert_eq!(arr[1], serde_json::json!({ "replaceable": true }));
}

#[tokio::test]
async fn call_raw_passes_params_through() {
    let (client, _) = build_client_with(|method, params| {
        assert_eq!(method, "echo");
        assert_eq!(params, Some(r#"[1,"two",true]"#));
        Ok(ok_response(serde_json::json!("ok")))
    });

    let _: String = client.call_raw("echo", &(1u64, "two", true)).await.unwrap();
}

#[tokio::test]
async fn malformed_response_surfaces_decode_error() {
    let (client, _) = build_client_with(|_, _| {
        // u64 cannot decode from a string; this is a decode-side mismatch.
        Ok(ok_response(serde_json::json!("not-a-number")))
    });

    let err = client.get_block_count().await.unwrap_err();
    assert!(matches!(err, Error::MalformedResponse { .. }), "{:?}", err);
    assert_eq!(err.retryability(), Retryability::NonRetryable);
}

fn matches_rpc(err: Error, expected_code: i32) {
    match err {
        Error::Rpc { code, .. } => assert_eq!(code, expected_code),
        other => panic!("expected RPC error, got {:?}", other),
    }
}

#[derive(Debug)]
struct StubTransportFailure;
impl fmt::Display for StubTransportFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "simulated transport failure")
    }
}
impl std::error::Error for StubTransportFailure {}
