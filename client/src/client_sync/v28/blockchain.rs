// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v28`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `scantxoutset`
#[macro_export]
macro_rules! impl_client_v28__scantxoutset {
    () => {
        impl Client {
            pub fn scan_tx_out_set(
                &self,
                action: ScanAction,
                scan_objects: Option<&[ScanObject]>,
            ) -> Result<ScanTxOutSet> {
                let params = match action {
                    ScanAction::Start => match scan_objects {
                        Some(objects) =>
                            vec![serde_json::to_value(action)?, serde_json::to_value(objects)?],
                        None =>
                            return Err(Error::Returned(
                                "scan_objects required for 'start'".to_string(),
                            )),
                    },
                    ScanAction::Abort | ScanAction::Status => {
                        vec![serde_json::to_value(action)?]
                    }
                };
                self.call("scantxoutset", &params)
            }
        }
    };
}
