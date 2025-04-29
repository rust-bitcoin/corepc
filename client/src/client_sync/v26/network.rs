// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of Bitcoin Core `v26`.
//!
//! See, or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `addnode`
#[macro_export]
macro_rules! impl_client_v26__addnode {
    () => {
        impl Client {
            pub fn add_node(
                &self,
                node: &str,
                command: AddNodeCommand,
                v2transport: Option<bool>,
            ) -> Result<AddNode> {
                let mut params = vec![node.into(), serde_json::to_value(command)?];

                if let Some(v2) = v2transport {
                    params.push(v2.into());
                }

                match self.call("addnode", &params) {
                    Ok(serde_json::Value::Null) => Ok(AddNode),
                    Ok(ref val) if val.is_null() => Ok(AddNode),
                    Ok(other) =>
                        Err(Error::Returned(format!("addnode expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}
