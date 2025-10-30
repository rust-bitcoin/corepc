// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v25`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `scanblocks`
#[macro_export]
macro_rules! impl_client_v25__scan_blocks {
    () => {
        impl Client {
            /// Aborts an ongoing `scanblocks` scan.
            pub fn scan_blocks_abort(&self) -> Result<ScanBlocksAbort> {
                self.call("scanblocks", &[into_json("abort")?])
            }

            /// Starts a scan of blocks for specified descriptors.
            pub fn scan_blocks_start(&self, scan_objects: &[&str]) -> Result<ScanBlocksStart> {
                self.call("scanblocks", &[into_json("start")?, into_json(scan_objects)?])
            }

            /// Checks the status of an ongoing `scanblocks` scan.
            pub fn scan_blocks_status(&self) -> Result<Option<ScanBlocksStatus>> {
                self.call("scanblocks", &[into_json("status")?])
            }
        }
    };
}
