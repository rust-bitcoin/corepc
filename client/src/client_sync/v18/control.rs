//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Control ==` section of the
//! API docs of Bitcoin Core `v0.18`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getrpcinfo`.
#[macro_export]
macro_rules! impl_client_v18__get_rpc_info {
    () => {
        impl Client {
            pub fn get_rpc_info(&self) -> Result<GetRpcInfo> { self.call("getrpcinfo", &[]) }
        }
    };
}
