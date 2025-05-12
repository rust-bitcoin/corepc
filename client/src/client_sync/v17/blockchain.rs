// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getblockchaininfo`.
#[macro_export]
macro_rules! impl_client_v17__get_blockchain_info {
    () => {
        impl Client {
            pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
                self.call("getblockchaininfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getbestblockhash`.
#[macro_export]
macro_rules! impl_client_v17__get_best_block_hash {
    () => {
        impl Client {
            /// Gets the blockhash of the current chain tip.
            pub fn best_block_hash(&self) -> Result<bitcoin::BlockHash> {
                let json = self.get_best_block_hash()?;
                Ok(json.block_hash()?)
            }

            pub fn get_best_block_hash(&self) -> Result<GetBestBlockHash> {
                self.call("getbestblockhash", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblock`.
#[macro_export]
macro_rules! impl_client_v17__get_block {
    () => {
        impl Client {
            /// Gets a block by blockhash.
            pub fn get_block(&self, hash: BlockHash) -> Result<Block> {
                let json = self.get_block_verbose_zero(hash)?;
                Ok(json.block()?)
            }

            /// Gets a block by blockhash with verbose set to 0.
            pub fn get_block_verbose_zero(&self, hash: BlockHash) -> Result<GetBlockVerboseZero> {
                self.call("getblock", &[into_json(hash)?, 0.into()])
            }

            /// Gets a block by blockhash with verbose set to 1.
            pub fn get_block_verbose_one(&self, hash: BlockHash) -> Result<GetBlockVerboseOne> {
                self.call("getblock", &[into_json(hash)?, 1.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockcount`.
#[macro_export]
macro_rules! impl_client_v17__get_block_count {
    () => {
        impl Client {
            pub fn get_block_count(&self) -> Result<GetBlockCount> {
                self.call("getblockcount", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockhash`.
#[macro_export]
macro_rules! impl_client_v17__get_block_hash {
    () => {
        impl Client {
            pub fn get_block_hash(&self, height: u64) -> Result<GetBlockHash> {
                self.call("getblockhash", &[into_json(height)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockheader`.
#[macro_export]
macro_rules! impl_client_v17__get_block_header {
    () => {
        impl Client {
            pub fn get_block_header(&self, hash: &BlockHash) -> Result<GetBlockHeader> {
                self.call("getblockheader", &[into_json(hash)?, into_json(false)?])
            }

            // This is the same as calling getblockheader with verbose==true.
            pub fn get_block_header_verbose(
                &self,
                hash: &BlockHash,
            ) -> Result<GetBlockHeaderVerbose> {
                self.call("getblockheader", &[into_json(hash)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockstats`.
#[macro_export]
macro_rules! impl_client_v17__get_block_stats {
    () => {
        impl Client {
            pub fn get_block_stats_by_height(&self, height: u32) -> Result<GetBlockStats> {
                self.call("getblockstats", &[into_json(height)?])
            }

            pub fn get_block_stats_by_block_hash(&self, hash: &BlockHash) -> Result<GetBlockStats> {
                self.call("getblockstats", &[into_json(hash)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getchaintips`.
#[macro_export]
macro_rules! impl_client_v17__get_chain_tips {
    () => {
        impl Client {
            pub fn get_chain_tips(&self) -> Result<GetChainTips> { self.call("getchaintips", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getchaintxstats`.
#[macro_export]
macro_rules! impl_client_v17__get_chain_tx_stats {
    () => {
        impl Client {
            pub fn get_chain_tx_stats(&self) -> Result<GetChainTxStats> {
                self.call("getchaintxstats", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdifficulty`.
#[macro_export]
macro_rules! impl_client_v17__get_difficulty {
    () => {
        impl Client {
            pub fn get_difficulty(&self) -> Result<GetDifficulty> {
                self.call("getdifficulty", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolancestors`.
#[macro_export]
macro_rules! impl_client_v17__get_mempool_ancestors {
    () => {
        impl Client {
            pub fn get_mempool_ancestors(&self, txid: Txid) -> Result<GetMempoolAncestors> {
                // Equivalent to self.call("getmempoolancestors", &[into_json(txid)?, into_json(false)?])
                self.call("getmempoolancestors", &[into_json(txid)?])
            }

            pub fn get_mempool_ancestors_verbose(
                &self,
                txid: Txid,
            ) -> Result<GetMempoolAncestorsVerbose> {
                self.call("getmempoolancestors", &[into_json(txid)?, into_json(true)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempooldescendants`.
#[macro_export]
macro_rules! impl_client_v17__get_mempool_descendants {
    () => {
        impl Client {
            pub fn get_mempool_descendants(&self, txid: Txid) -> Result<GetMempoolDescendants> {
                // Equivalent to self.call("getmempooldescendants", &[into_json(txid)?, into_json(false)?])
                self.call("getmempooldescendants", &[into_json(txid)?])
            }

            pub fn get_mempool_descendants_verbose(
                &self,
                txid: Txid,
            ) -> Result<GetMempoolDescendantsVerbose> {
                self.call("getmempooldescendants", &[into_json(txid)?, into_json(true)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolentry`.
#[macro_export]
macro_rules! impl_client_v17__get_mempool_entry {
    () => {
        impl Client {
            pub fn get_mempool_entry(&self, txid: Txid) -> Result<GetMempoolEntry> {
                self.call("getmempoolentry", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolinfo`.
#[macro_export]
macro_rules! impl_client_v17__get_mempool_info {
    () => {
        impl Client {
            pub fn get_mempool_info(&self) -> Result<GetMempoolInfo> {
                self.call("getmempoolinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawmempool`.
#[macro_export]
macro_rules! impl_client_v17__get_raw_mempool {
    () => {
        impl Client {
            pub fn get_raw_mempool(&self) -> Result<GetRawMempool> {
                // Equivalent to self.call("getrawmempool", &[into_json(false)?])
                self.call("getrawmempool", &[])
            }
            pub fn get_raw_mempool_verbose(&self) -> Result<GetRawMempoolVerbose> {
                self.call("getrawmempool", &[into_json(true)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxout`.
#[macro_export]
macro_rules! impl_client_v17__get_tx_out {
    () => {
        impl Client {
            pub fn get_tx_out(&self, txid: Txid, vout: u64) -> Result<GetTxOut> {
                self.call("gettxout", &[into_json(txid)?, into_json(vout)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxoutproof`.
#[macro_export]
macro_rules! impl_client_v17__get_tx_out_proof {
    () => {
        impl Client {
            pub fn get_tx_out_proof(&self, txids: &[Txid]) -> Result<String> {
                self.call("gettxoutproof", &[into_json(txids)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxoutsetinfo`.
#[macro_export]
macro_rules! impl_client_v17__get_tx_out_set_info {
    () => {
        impl Client {
            pub fn get_tx_out_set_info(&self) -> Result<GetTxOutSetInfo> {
                self.call("gettxoutsetinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `preciousblock`.
#[macro_export]
macro_rules! impl_client_v17__precious_block {
    () => {
        impl Client {
            pub fn precious_block(&self, hash: BlockHash) -> Result<()> {
                match self.call("preciousblock", &[into_json(hash)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `pruneblockchain`.
#[macro_export]
macro_rules! impl_client_v17__prune_blockchain {
    () => {
        impl Client {
            /// Instructs the node to prune the blockchain up to a specified height or timestamp.
            pub fn prune_blockchain(&self, target: u64) -> Result<PruneBlockchain> {
                self.call("pruneblockchain", &[target.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `savemempool`.
#[macro_export]
macro_rules! impl_client_v17__save_mempool {
    () => {
        impl Client {
            pub fn save_mempool(&self) -> Result<()> {
                match self.call("savemempool", &[]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `scantxoutset`
#[macro_export]
macro_rules! impl_client_v17__scan_tx_out_set {
    () => {
        impl Client {
            /// Aborts an ongoing `scantxoutset` scan.
            pub fn scan_tx_out_set_abort(&self) -> Result<ScanTxOutSetAbort> {
                self.call("scantxoutset", &[into_json("abort")?])
            }

            /// Starts a scan of the UTXO set for specified descriptors.
            pub fn scan_tx_out_set_start(&self, scan_objects: &[&str]) -> Result<ScanTxOutSetStart> {
                self.call("scantxoutset", &[into_json("start")?, into_json(scan_objects)?])
            }

            /// Checks the status of an ongoing `scantxoutset` scan.
            pub fn scan_tx_out_set_status(&self) -> Result<Option<ScanTxOutSetStatus>> {
                self.call("scantxoutset", &[into_json("status")?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `verifychain`
#[macro_export]
macro_rules! impl_client_v17__verify_chain {
    () => {
        impl Client {
            pub fn verify_chain(&self) -> Result<VerifyChain> { self.call("verifychain", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `verifytxoutproof`.
#[macro_export]
macro_rules! impl_client_v17__verify_tx_out_proof {
    () => {
        impl Client {
            // `proof` is the hex-encoded proof generated by `gettxoutproof`.
            pub fn verify_tx_out_proof(&self, proof: &str) -> Result<VerifyTxOutProof> {
                self.call("verifytxoutproof", &[into_json(proof)?])
            }
        }
    };
}
