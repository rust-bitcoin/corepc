// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Generating ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `generatetoaddress`
#[macro_export]
macro_rules! impl_client_v17__generate_to_address {
    () => {
        impl Client {
            pub fn generate_to_address(
                &self,
                nblocks: usize,
                address: &bitcoin::Address,
            ) -> Result<GenerateToAddress> {
                self.call("generatetoaddress", &[nblocks.into(), into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `generate`
#[macro_export]
macro_rules! impl_client_v17__generate {
    () => {
        impl Client {
            pub fn generate(&self, nblocks: usize) -> Result<Generate> {
                self.call("generate", &[nblocks.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `invalidateblock`
// This method does not appear in the output of `bitcoin-cli help`.
#[macro_export]
macro_rules! impl_client_v17__invalidate_block {
    () => {
        impl Client {
            pub fn invalidate_block(&self, hash: BlockHash) -> Result<()> {
                match self.call("invalidateblock", &[into_json(hash)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}
