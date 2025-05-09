// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Mining ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getblocktemplate`
#[macro_export]
macro_rules! impl_client_v17__getblocktemplate {
    () => {
        impl Client {
            pub fn get_block_template(
                &self,
                request: &TemplateRequest,
            ) -> Result<GetBlockTemplate> {
                self.call("getblocktemplate", &[into_json(request)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmininginfo`
#[macro_export]
macro_rules! impl_client_v17__getmininginfo {
    () => {
        impl Client {
            pub fn get_mining_info(&self) -> Result<GetMiningInfo> {
                self.call("getmininginfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnetworkhashps`
#[macro_export]
macro_rules! impl_client_v17__getnetworkhashps {
    () => {
        impl Client {
            pub fn get_network_hash_ps(&self) -> Result<f64> { self.call("getnetworkhashps", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `prioritisetransaction`
#[macro_export]
macro_rules! impl_client_v17__prioritisetransaction {
    () => {
        impl Client {
            pub fn prioritise_transaction(
                &self,
                txid: &Txid,
                fee_delta: bitcoin::SignedAmount,
            ) -> Result<bool> {
                let sats = fee_delta.to_sat();
                self.call("prioritisetransaction", &[into_json(txid)?, 0.into(), sats.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `submitblock`
#[macro_export]
macro_rules! impl_client_v17__submitblock {
    () => {
        impl Client {
            pub fn submit_block(&self, block: &Block) -> Result<()> {
                let hex: String = bitcoin::consensus::encode::serialize_hex(block);
                match self.call("submitblock", &[into_json(hex)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}
