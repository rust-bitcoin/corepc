// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v29`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getdescriptoractivity`
#[macro_export]
macro_rules! impl_client_v29__getdescriptoractivity {
    () => {
        impl Client {
            pub fn get_descriptor_activity(
                &self,
                blockhashes: &[BlockHash],
                scan_objects: &[&str],
                include_mempool: bool,
            ) -> Result<GetDescriptorActivity> {
                let blockhashes_val = json!(blockhashes);
                let scan_objects_val = json!(scan_objects);
                let include_mempool_val = json!(include_mempool);

                let params = vec![blockhashes_val, scan_objects_val, include_mempool_val];

                self.call("getdescriptoractivity", &params)
            }
        }
    };
}
