// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `combinepsbt`.
#[macro_export]
macro_rules! impl_client_v17__combine_psbt {
    () => {
        impl Client {
            pub fn combine_psbt(&self, txs: &[bitcoin::Psbt]) -> Result<CombinePsbt> {
                let txs = txs.iter().map(|psbt| format!("{}", psbt)).collect::<Vec<String>>();
                self.call("combinepsbt", &[txs.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `combinerawtransaction`.
#[macro_export]
macro_rules! impl_client_v17__combine_raw_transaction {
    () => {
        impl Client {
            pub fn combine_raw_transaction(
                &self,
                txs: &[bitcoin::Transaction],
            ) -> Result<CombineRawTransaction> {
                let encoded = txs
                    .iter()
                    .map(|tx| bitcoin::consensus::encode::serialize_hex(tx))
                    .collect::<Vec<String>>();
                self.call("combinerawtransaction", &[into_json(encoded)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `converttopsbt`.
#[macro_export]
macro_rules! impl_client_v17__convert_to_psbt {
    () => {
        impl Client {
            pub fn convert_to_psbt(&self, tx: &bitcoin::Transaction) -> Result<ConvertToPsbt> {
                let hex = bitcoin::consensus::encode::serialize_hex(tx);
                self.call("converttopsbt", &[hex.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createpsbt`.
#[macro_export]
macro_rules! impl_client_v17__create_psbt {
    () => {
        impl Client {
            pub fn create_psbt(&self, inputs: &[Input], outputs: &[Output]) -> Result<CreatePsbt> {
                self.call("createpsbt", &[into_json(inputs)?, into_json(outputs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createrawtransaction`.
#[macro_export]
macro_rules! impl_client_v17__create_raw_transaction {
    () => {
        impl Client {
            pub fn create_raw_transaction(
                &self,
                inputs: &[Input],
                outputs: &[Output],
            ) -> Result<CreateRawTransaction> {
                self.call("createrawtransaction", &[into_json(inputs)?, into_json(outputs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `decodepsbt`.
#[macro_export]
macro_rules! impl_client_v17__decode_psbt {
    () => {
        impl Client {
            pub fn decode_psbt(&self, psbt: &str) -> Result<DecodePsbt> {
                self.call("decodepsbt", &[psbt.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `finalizepsbt`.
#[macro_export]
macro_rules! impl_client_v17__finalize_psbt {
    () => {
        impl Client {
            pub fn finalize_psbt(&self, psbt: &bitcoin::Psbt) -> Result<FinalizePsbt> {
                let psbt = format!("{}", psbt);
                // Pass extract=false so Core returns the PSBT field in the response.
                self.call("finalizepsbt", &[psbt.into(), false.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `decoderawtransaction`.
#[macro_export]
macro_rules! impl_client_v17__decode_raw_transaction {
    () => {
        impl Client {
            pub fn decode_raw_transaction(
                &self,
                tx: &bitcoin::Transaction,
            ) -> Result<DecodeRawTransaction> {
                let hex = bitcoin::consensus::encode::serialize_hex(tx);
                self.call("decoderawtransaction", &[hex.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `decodescript`.
#[macro_export]
macro_rules! impl_client_v17__decode_script {
    () => {
        impl Client {
            // Arg is the hex encoded script we want to decode.
            pub fn decode_script(&self, script: &str) -> Result<DecodeScript> {
                self.call("decodescript", &[script.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `fundrawtransaction`.
#[macro_export]
macro_rules! impl_client_v17__fund_raw_transaction {
    () => {
        impl Client {
            pub fn fund_raw_transaction(
                &self,
                tx: &bitcoin::Transaction,
            ) -> Result<FundRawTransaction> {
                let hex = bitcoin::consensus::encode::serialize_hex(tx);
                self.call("fundrawtransaction", &[hex.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawtransaction`.
#[macro_export]
macro_rules! impl_client_v17__get_raw_transaction {
    () => {
        impl Client {
            pub fn get_raw_transaction(&self, txid: bitcoin::Txid) -> Result<GetRawTransaction> {
                self.call("getrawtransaction", &[into_json(&txid)?, false.into()])
            }

            pub fn get_raw_transaction_verbose(
                &self,
                txid: Txid,
            ) -> Result<GetRawTransactionVerbose> {
                self.call("getrawtransaction", &[into_json(&txid)?, true.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendrawtransaction`.
#[macro_export]
macro_rules! impl_client_v17__send_raw_transaction {
    () => {
        impl Client {
            pub fn send_raw_transaction(
                &self,
                tx: &bitcoin::Transaction,
            ) -> Result<SendRawTransaction> {
                let hex = bitcoin::consensus::encode::serialize_hex(tx);
                self.call("sendrawtransaction", &[hex.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signrawtransaction`.
#[macro_export]
macro_rules! impl_client_v17__sign_raw_transaction {
    () => {
        impl Client {
            pub fn sign_raw_transaction(
                &self,
                tx: &bitcoin::Transaction,
            ) -> Result<SignRawTransaction> {
                let hex = bitcoin::consensus::encode::serialize_hex(tx);
                self.call("signrawtransaction", &[hex.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signrawtransactionwithkey`.
#[macro_export]
macro_rules! impl_client_v17__sign_raw_transaction_with_key {
    () => {
        impl Client {
            pub fn sign_raw_transaction_with_key(
                &self,
                tx: &bitcoin::Transaction,
                keys: &[bitcoin::PrivateKey],
            ) -> Result<SignRawTransactionWithKey> {
                let hex = bitcoin::consensus::encode::serialize_hex(tx);
                let keys = keys.iter().map(|k| format!("{}", k)).collect::<Vec<String>>();
                self.call("signrawtransactionwithkey", &[hex.into(), into_json(keys)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `testmempoolaccept`.
#[macro_export]
macro_rules! impl_client_v17__test_mempool_accept {
    () => {
        impl Client {
            pub fn test_mempool_accept(
                &self,
                txs: &[bitcoin::Transaction],
            ) -> Result<TestMempoolAccept> {
                let encoded = txs
                    .iter()
                    .map(|tx| bitcoin::consensus::encode::serialize_hex(tx))
                    .collect::<Vec<String>>();
                self.call("testmempoolaccept", &[into_json(encoded)?])
            }
        }
    };
}
