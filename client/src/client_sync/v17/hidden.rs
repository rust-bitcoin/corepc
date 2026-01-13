// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is `== Hidden ==` methods that are not listed in the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `estimaterawfee`.
#[macro_export]
macro_rules! impl_client_v17__estimate_raw_fee {
    () => {
        impl Client {
            pub fn estimate_raw_fee(&self, conf_target: u32) -> Result<EstimateRawFee> {
                self.call("estimaterawfee", &[conf_target.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `waitforblock`.
#[macro_export]
macro_rules! impl_client_v17__wait_for_block {
    () => {
        impl Client {
            pub fn wait_for_block(&self, hash: &bitcoin::BlockHash) -> Result<WaitForBlock> {
                self.call("waitforblock", &[into_json(hash)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `waitforblockheight`.
#[macro_export]
macro_rules! impl_client_v17__wait_for_block_height {
    () => {
        impl Client {
            pub fn wait_for_block_height(&self, height: u64) -> Result<WaitForBlockHeight> {
                self.call("waitforblockheight", &[into_json(height)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `waitfornewblock`.
#[macro_export]
macro_rules! impl_client_v17__wait_for_new_block {
    () => {
        impl Client {
            pub fn wait_for_new_block(&self) -> Result<WaitForNewBlock> {
                self.call("waitfornewblock", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `syncwithvalidationinterfacequeue`.
#[macro_export]
macro_rules! impl_client_v17__sync_with_validation_interface_queue {
    () => {
        impl Client {
            pub fn sync_with_validation_interface_queue(&self) -> Result<()> {
                self.call("syncwithvalidationinterfacequeue", &[])
            }
        }
    };
}
