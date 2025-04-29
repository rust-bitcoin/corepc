// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getblockchaininfo`
#[macro_export]
macro_rules! impl_client_v17__getblockchaininfo {
    () => {
        impl Client {
            pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
                self.call("getblockchaininfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getbestblockhash`
#[macro_export]
macro_rules! impl_client_v17__getbestblockhash {
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

/// Implements Bitcoin Core JSON-RPC API method `getblock`
#[macro_export]
macro_rules! impl_client_v17__getblock {
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

/// Implements Bitcoin Core JSON-RPC API method `getblockcount`
#[macro_export]
macro_rules! impl_client_v17__getblockcount {
    () => {
        impl Client {
            pub fn get_block_count(&self) -> Result<GetBlockCount> {
                self.call("getblockcount", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockhash`
#[macro_export]
macro_rules! impl_client_v17__getblockhash {
    () => {
        impl Client {
            pub fn get_block_hash(&self, height: u64) -> Result<GetBlockHash> {
                self.call("getblockhash", &[into_json(height)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockheader`
#[macro_export]
macro_rules! impl_client_v17__getblockheader {
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

/// Implements Bitcoin Core JSON-RPC API method `getblockstats`
#[macro_export]
macro_rules! impl_client_v17__getblockstats {
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

/// Implements Bitcoin Core JSON-RPC API method `getchaintips`
#[macro_export]
macro_rules! impl_client_v17__getchaintips {
    () => {
        impl Client {
            pub fn get_chain_tips(&self) -> Result<GetChainTips> { self.call("getchaintips", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getchaintxstats`
#[macro_export]
macro_rules! impl_client_v17__getchaintxstats {
    () => {
        impl Client {
            pub fn get_chain_tx_stats(&self) -> Result<GetChainTxStats> {
                self.call("getchaintxstats", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdifficulty`
#[macro_export]
macro_rules! impl_client_v17__getdifficulty {
    () => {
        impl Client {
            pub fn get_difficulty(&self) -> Result<GetDifficulty> {
                self.call("getdifficulty", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolancestors`
#[macro_export]
macro_rules! impl_client_v17__getmempoolancestors {
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

/// Implements Bitcoin Core JSON-RPC API method `getmempooldescendants`
#[macro_export]
macro_rules! impl_client_v17__getmempooldescendants {
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

/// Implements Bitcoin Core JSON-RPC API method `getmempoolentry`
#[macro_export]
macro_rules! impl_client_v17__getmempoolentry {
    () => {
        impl Client {
            pub fn get_mempool_entry(&self, txid: Txid) -> Result<GetMempoolEntry> {
                self.call("getmempoolentry", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolinfo`
#[macro_export]
macro_rules! impl_client_v17__getmempoolinfo {
    () => {
        impl Client {
            pub fn get_mempool_info(&self) -> Result<GetMempoolInfo> {
                self.call("getmempoolinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawmempool`
#[macro_export]
macro_rules! impl_client_v17__getrawmempool {
    () => {
        impl Client {
            pub fn get_raw_mempool(&self) -> Result<GetRawMempool> {
                // Equivalent to self.call("getrawmempool", &[into_json(false)?])
                self.call("getrawmempool", &[])
            }
            pub fn get_raw_mempool_verbose(&self) -> Result<GetRawMempool> {
                self.call("getrawmempool", &[into_json(true)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxout`
#[macro_export]
macro_rules! impl_client_v17__gettxout {
    () => {
        impl Client {
            pub fn get_tx_out(&self, txid: Txid, vout: u64) -> Result<GetTxOut> {
                self.call("gettxout", &[into_json(txid)?, into_json(vout)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxoutproof`
#[macro_export]
macro_rules! impl_client_v17__gettxoutproof {
    () => {
        impl Client {
            pub fn get_tx_out_proof(&self, txids: &[Txid]) -> Result<String> {
                self.call("gettxoutproof", &[into_json(txids)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxoutsetinfo`
#[macro_export]
macro_rules! impl_client_v17__gettxoutsetinfo {
    () => {
        impl Client {
            pub fn get_tx_out_set_info(&self) -> Result<GetTxOutSetInfo> {
                self.call("gettxoutsetinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `preciousblock`
#[macro_export]
macro_rules! impl_client_v17__preciousblock {
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

/// Implements Bitcoin Core JSON-RPC API method `verifytxoutproof`
#[macro_export]
macro_rules! impl_client_v17__verifytxoutproof {
    () => {
        impl Client {
            // `proof` is the hex-encoded proof generated by `gettxoutproof`.
            pub fn verify_tx_out_proof(&self, proof: &str) -> Result<VerifyTxOutProof> {
                self.call("verifytxoutproof", &[into_json(proof)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `pruneblockchain`
#[macro_export]
macro_rules! impl_client_v17__pruneblockchain {
    () => {
        impl Client {
            /// Instructs the node to prune the blockchain up to a specified height or timestamp.
            pub fn prune_blockchain(&self, target: u64) -> Result<PruneBlockchain> {
                self.call("pruneblockchain", &[target.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `savemempool`
#[macro_export]
macro_rules! impl_client_v17__savemempool {
    () => {
        impl Client {
            // Dumps the mempool to disk (v17 - v22)
            pub fn save_mempool(&self) -> Result<SaveMempool> {
                match self.call("savemempool", &[]) {
                    Ok(serde_json::Value::Null) => Ok(SaveMempool),
                    Ok(_) => Ok(SaveMempool),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `verifychain`
#[macro_export]
macro_rules! impl_client_v17__verifychain {
    () => {
        impl Client {
            /// Verifies blockchain database using default checklevel and nblocks.
            /// Returns true if verification finised successfully.
            pub fn verify_chain_default(&self) -> Result<VerifyChain> {
                // Core returns a boolean directly
                self.call("verifychain", &[])
            }

            /// Verifies blockchain database with specified checklevel and nblocks.
            ///
            /// # Arguments
            /// * `checklevel`: (0-4) How thorough the verification is (optional, defaults to 3).
            /// * `nblocks`: Number of blocks to check (optional, defaults to 6, 0=all).
            ///
            /// Returns true if verification finished successfully.
            pub fn verify_chain(
                &self,
                checklevel: Option<u32>,
                nblocks: Option<u32>,
            ) -> Result<VerifyChain> {
                // Construct params carefully for optional args
                // Bitcoin Core often uses positional nulls for omitted optional args
                let params = match (checklevel, nblocks) {
                    (Some(level), Some(num)) => vec![level.into(), num.into()],
                    (Some(level), None) => vec![level.into()],
                    (None, Some(num)) => vec![serde_json::Value::Null, num.into()],
                    (None, None) => vec![],
                };

                self.call("verifychain", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `scantxoutset`
#[macro_export]
macro_rules! impl_client_v17__scantxoutset {
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
