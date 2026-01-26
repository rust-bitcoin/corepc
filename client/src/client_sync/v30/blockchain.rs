// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v30`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getdescriptoractivity`.
#[macro_export]
macro_rules! impl_client_v30__get_descriptor_activity {
    () => {
        impl Client {
            pub fn get_descriptor_activity(
                &self,
                block_hashes: &[BlockHash],
                scan_objects: &[&str],
            ) -> Result<GetDescriptorActivity> {
                let params = vec![json!(block_hashes), json!(scan_objects)];
                self.call("getdescriptoractivity", &params)
            }
        }
    };
}
