// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Mining ==` section of the
//! API docs of Bitcoin Core `v0.18`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `submitheader`.
#[macro_export]
macro_rules! impl_client_v18__submit_header {
    () => {
        impl Client {
            pub fn submit_header(&self, header: &bitcoin::block::Header) -> Result<()> {
                let hexdata = bitcoin::consensus::encode::serialize_hex(header);
                match self.call("submitheader", &[hexdata.into()]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}
