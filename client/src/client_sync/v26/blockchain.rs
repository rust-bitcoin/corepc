// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v26`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `dumptxoutset`.
#[macro_export]
macro_rules! impl_client_v26__dump_tx_out_set {
    () => {
        impl Client {
            pub fn dump_tx_out_set(&self, path: &str) -> Result<DumpTxOutSet> {
                self.call("dumptxoutset", &[path.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getchainstates`.
#[macro_export]
macro_rules! impl_client_v26__get_chain_states {
    () => {
        impl Client {
            pub fn get_chain_states(&self) -> Result<GetChainStates> {
                self.call("getchainstates", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxoutsetinfo`.
#[macro_export]
macro_rules! impl_client_v26__get_tx_out_set_info {
    () => {
        impl Client {
            pub fn get_tx_out_set_info(&self) -> Result<GetTxOutSetInfo> {
                self.call("gettxoutsetinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importmempool`.
#[macro_export]
macro_rules! impl_client_v26__import_mempool {
    () => {
        impl Client {
            pub fn import_mempool(&self, filepath: &str) -> Result<()> {
                match self.call("importmempool", &[filepath.into()]) {
                    Ok(serde_json::Value::Object(ref map)) if map.is_empty() => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `loadtxoutset`.
#[macro_export]
macro_rules! impl_client_v26__load_tx_out_set {
    () => {
        impl Client {
            pub fn load_tx_out_set(&self, path: &str) -> Result<LoadTxOutSet> {
                self.call("loadtxoutset", &[path.into()])
            }
        }
    };
}
