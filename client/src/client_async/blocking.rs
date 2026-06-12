// SPDX-License-Identifier: CC0-1.0

//! A blocking, sync-API client for Bitcoin Core `v30` that drives the **generated async client**.
//!
//! This exposes the same method surface as [`crate::client_sync::v30`] (same names, argument types
//! and curated return types the integration tests expect), but each method calls the corresponding
//! generated async method on [`crate::client_async::Client`] and blocks on it. The generated
//! return type (`types::v30::generated::*`) is then bridged to the curated `vtype` type via a JSON
//! round-trip (the private `reserialize` helper). This means the unchanged integration tests
//! actually exercise the async client's generated code.
//!
//! `bitcoind` swaps `node.client` to this type under its async feature.
//!
//! Methods whose RPC has no generated async wrapper (the `== Generating ==`, `== Hidden ==` and
//! `== Zmq ==` sections, plus a few with no async counterpart) fall back to the sync
//! `impl_client_v*` macros at the bottom, which call the raw transport directly.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{sign_message, Amount, Block, BlockHash, PublicKey, Txid};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};

use crate::client_async::{Auth as AsyncAuth, Builder};
use crate::client_sync::error::Error;
use crate::client_sync::{Auth, Result};
use crate::types::v30::*;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::client_sync::{
    v17::{
      AddNodeCommand, ImportMultiRequest, ImportMultiScriptPubKey, ImportMultiTimestamp, Input, Output, SetBanCommand, WalletCreateFundedPsbtInput,
      FeeEstimateMode,
    },
    v21::ImportDescriptorsRequest,
    v23::AddressType,
    v29::{TemplateRequest, TemplateRules}
};

/// Bridges a generated async return type to its curated `v30` counterpart by round-tripping through
/// JSON. Both are serde representations of the same Core response, so for the common case this
/// reproduces exactly what the sync client would have deserialized.
fn reserialize<G, C>(value: G) -> Result<C>
where
    G: Serialize,
    C: DeserializeOwned,
{
    Ok(serde_json::from_value(serde_json::to_value(value)?)?)
}

/// Shorthand for converting a value into a `serde_json::Value` (used by the fallback macros).
fn into_json<T>(val: T) -> Result<Value>
where
    T: Serialize,
{
    Ok(serde_json::to_value(val)?)
}

/// A blocking JSON-RPC client that drives the async `v30` generated client.
pub struct Client {
    inner: crate::client_async::Client,
    rt: tokio::runtime::Runtime,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "corepc_client::client_async::blocking::Client({:?})", self.inner)
    }
}

impl Client {
    /// Creates a blocking async-backed client (signature matches `client_sync`'s `new_with_auth`).
    pub fn new_with_auth(url: &str, auth: Auth) -> Result<Self> {
        let auth = match auth {
            Auth::None => AsyncAuth::None,
            Auth::UserPass(user, pass) => AsyncAuth::UserPass(user, pass),
            Auth::CookieFile(path) => AsyncAuth::CookieFile(path),
        };
        let inner = Builder::new()
            .url(url)
            .map_err(|e| Error::Returned(e.to_string()))?
            .auth(auth)
            // Match the sync client's transport timeout: the integration suite runs many nodes in
            // parallel and slow setup calls (mining a chain) are calibrated against that value;
            // the async builder's shorter default times them out under load.
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| Error::Returned(e.to_string()))?;
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| Error::Returned(e.to_string()))?;
        Ok(Self { inner, rt })
    }

    /// Maps an async-client error into the sync error type used by the tests.
    ///
    /// A JSON-RPC application error keeps its code and message in the sync `JsonRpc` variant so
    /// callers that inspect Core error codes behave the same against either client; everything
    /// else is flattened into `Returned` with the async error's display text.
    fn map_err(e: crate::client_async::Error) -> Error {
        match e {
            crate::client_async::Error::Rpc { code, message, .. } =>
                Error::JsonRpc(jsonrpc::error::Error::Rpc(jsonrpc::error::RpcError {
                    code,
                    message,
                    data: None,
                })),
            other => Error::Returned(other.to_string()),
        }
    }

    /// Raw escape hatch used directly by some tests and by the fallback macros below.
    pub fn call<T: DeserializeOwned>(&self, method: &str, args: &[Value]) -> Result<T> {
        self.rt.block_on(self.inner.call_raw(method, args)).map_err(Self::map_err)
    }
}

// ===========================================================================
// Methods routed through the generated async wrappers.
// ===========================================================================

impl Client {
    // == Blockchain ==

    pub fn get_best_block_hash(&self) -> Result<GetBestBlockHash> {
        let g = self.rt.block_on(self.inner.get_best_block_hash()).map_err(Self::map_err)?;
        reserialize(g)
    }

    /// Convenience wrapper bundled into the sync `get_best_block_hash` macro.
    pub fn best_block_hash(&self) -> Result<bitcoin::BlockHash> {
        Ok(self.get_best_block_hash()?.block_hash()?)
    }

    pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
        let g = self.rt.block_on(self.inner.get_blockchain_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_count(&self) -> Result<GetBlockCount> {
        let g = self.rt.block_on(self.inner.get_block_count()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_hash(&self, height: u64) -> Result<GetBlockHash> {
        let g =
            self.rt.block_on(self.inner.get_block_hash(height as i64)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_filter(&self, block: BlockHash) -> Result<GetBlockFilter> {
        let g = self
            .rt
            .block_on(self.inner.get_block_filter(block.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_header(&self, hash: &BlockHash) -> Result<GetBlockHeader> {
        let g = self
            .rt
            .block_on(self.inner.get_block_header(hash.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_header_verbose(&self, hash: &BlockHash) -> Result<GetBlockHeaderVerbose> {
        let g = self
            .rt
            .block_on(self.inner.get_block_header_verbose_one(hash.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_verbose_zero(&self, hash: BlockHash) -> Result<GetBlockVerboseZero> {
        let g = self.rt.block_on(self.inner.get_block(hash.to_string())).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_verbose_one(&self, hash: BlockHash) -> Result<GetBlockVerboseOne> {
        let g = self
            .rt
            .block_on(self.inner.get_block_verbose_one(hash.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_verbose_two(&self, hash: BlockHash) -> Result<GetBlockVerboseTwo> {
        let g = self
            .rt
            .block_on(self.inner.get_block_verbose_two(hash.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_block_verbose_three(&self, hash: BlockHash) -> Result<GetBlockVerboseThree> {
        let g = self
            .rt
            .block_on(self.inner.get_block_verbose_three(hash.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    /// Convenience wrapper bundled into the sync `get_block` macro: decode verbosity 0 to a `Block`.
    pub fn get_block(&self, hash: BlockHash) -> Result<Block> {
        Ok(self.get_block_verbose_zero(hash)?.block()?)
    }

    pub fn get_chain_states(&self) -> Result<GetChainStates> {
        let g = self.rt.block_on(self.inner.get_chain_states()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_chain_tips(&self) -> Result<GetChainTips> {
        let g = self.rt.block_on(self.inner.get_chain_tips()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_chain_tx_stats(&self) -> Result<GetChainTxStats> {
        let g = self.rt.block_on(self.inner.get_chain_tx_stats()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_difficulty(&self) -> Result<GetDifficulty> {
        let g = self.rt.block_on(self.inner.get_difficulty()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_mempool_ancestors(&self, txid: Txid) -> Result<GetMempoolAncestors> {
        let g = self
            .rt
            .block_on(self.inner.get_mempool_ancestors(txid.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_mempool_ancestors_verbose(&self, txid: Txid) -> Result<GetMempoolAncestorsVerbose> {
        let g = self
            .rt
            .block_on(self.inner.get_mempool_ancestors_verbose_one(txid.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_mempool_descendants(&self, txid: Txid) -> Result<GetMempoolDescendants> {
        let g = self
            .rt
            .block_on(self.inner.get_mempool_descendants(txid.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_mempool_descendants_verbose(
        &self,
        txid: Txid,
    ) -> Result<GetMempoolDescendantsVerbose> {
        let g = self
            .rt
            .block_on(self.inner.get_mempool_descendants_verbose_one(txid.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_mempool_entry(&self, txid: Txid) -> Result<GetMempoolEntry> {
        let g = self
            .rt
            .block_on(self.inner.get_mempool_entry(txid.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_mempool_info(&self) -> Result<GetMempoolInfo> {
        let g = self.rt.block_on(self.inner.get_mempool_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_tx_out(&self, txid: Txid, vout: u64) -> Result<GetTxOut> {
        let g = self
            .rt
            .block_on(self.inner.get_tx_out(txid.to_string(), vout as i64))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_tx_out_proof(&self, txids: &[Txid]) -> Result<String> {
        let txids = txids.iter().map(|t| t.to_string()).collect();
        let g = self.rt.block_on(self.inner.get_tx_out_proof(txids)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_tx_out_set_info(&self) -> Result<GetTxOutSetInfo> {
        let g = self.rt.block_on(self.inner.get_tx_out_set_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_tx_spending_prevout(
        &self,
        outputs: &[bitcoin::OutPoint],
    ) -> Result<GetTxSpendingPrevout> {
        let outputs = outputs
            .iter()
            .map(|o| json!({ "txid": o.txid.to_string(), "vout": o.vout }))
            .collect::<Vec<_>>();
        let outputs = serde_json::from_value(json!(outputs))?;
        let g =
            self.rt.block_on(self.inner.get_tx_spending_prevout(outputs)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn precious_block(&self, hash: BlockHash) -> Result<()> {
        self.rt.block_on(self.inner.precious_block(hash.to_string())).map_err(Self::map_err)
    }

    pub fn prune_blockchain(&self, target: u64) -> Result<PruneBlockchain> {
        let g =
            self.rt.block_on(self.inner.prune_blockchain(target as i64)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn save_mempool(&self) -> Result<SaveMempool> {
        let g = self.rt.block_on(self.inner.save_mempool()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn verify_chain(&self) -> Result<VerifyChain> {
        let g = self.rt.block_on(self.inner.verify_chain()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn verify_tx_out_proof(&self, proof: &str) -> Result<VerifyTxOutProof> {
        let g = self
            .rt
            .block_on(self.inner.verify_tx_out_proof(proof.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn wait_for_block(&self, hash: &bitcoin::BlockHash) -> Result<WaitForBlock> {
        let g =
            self.rt.block_on(self.inner.wait_for_block(hash.to_string())).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn wait_for_block_height(&self, height: u64) -> Result<WaitForBlockHeight> {
        let g = self
            .rt
            .block_on(self.inner.wait_for_block_height(height as i64))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn wait_for_new_block(&self) -> Result<WaitForNewBlock> {
        let g = self.rt.block_on(self.inner.wait_for_new_block()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn import_mempool(&self, filepath: &str) -> Result<()> {
        self.rt.block_on(self.inner.import_mempool(filepath.to_string())).map_err(Self::map_err)?;
        Ok(())
    }

    pub fn load_tx_out_set(&self, path: &str) -> Result<LoadTxOutSet> {
        let g = self
            .rt
            .block_on(self.inner.load_tx_out_set(path.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn dump_tx_out_set(&self, path: &str, snapshot_type: &str) -> Result<DumpTxOutSet> {
        let opts = crate::client_async::v30::blockchain::DumpTxOutSetOptions {
            type_: Some(snapshot_type.to_string()),
            options: None,
        };
        let g = self
            .rt
            .block_on(self.inner.dump_tx_out_set_with(path.to_string(), opts))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    // == Control ==

    pub fn get_memory_info(&self) -> Result<GetMemoryInfoStats> {
        let g = self.rt.block_on(self.inner.get_memory_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_rpc_info(&self) -> Result<GetRpcInfo> {
        let g = self.rt.block_on(self.inner.get_rpc_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn help(&self) -> Result<String> {
        let g = self.rt.block_on(self.inner.help()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn logging(&self) -> Result<Logging> {
        let g = self.rt.block_on(self.inner.logging()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn stop(&self) -> Result<String> {
        let g = self.rt.block_on(self.inner.stop()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn uptime(&self) -> Result<u32> {
        let g = self.rt.block_on(self.inner.uptime()).map_err(Self::map_err)?;
        reserialize(g)
    }

    // == Mining ==

    pub fn get_mining_info(&self) -> Result<GetMiningInfo> {
        let g = self.rt.block_on(self.inner.get_mining_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_network_hash_ps(&self) -> Result<f64> {
        let g = self.rt.block_on(self.inner.get_network_hash_ps()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_prioritised_transactions(&self) -> Result<GetPrioritisedTransactions> {
        let g =
            self.rt.block_on(self.inner.get_prioritised_transactions()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn prioritise_transaction(
        &self,
        txid: &Txid,
        fee_delta: bitcoin::SignedAmount,
    ) -> Result<bool> {
        let g = self
            .rt
            .block_on(self.inner.prioritise_transaction(txid.to_string(), fee_delta.to_sat()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn submit_block(&self, block: &Block) -> Result<()> {
        let hex = bitcoin::consensus::encode::serialize_hex(block);
        self.rt.block_on(self.inner.submit_block(hex)).map_err(Self::map_err)?;
        Ok(())
    }

    pub fn submit_header(&self, header: &bitcoin::block::Header) -> Result<()> {
        let hex = bitcoin::consensus::encode::serialize_hex(header);
        self.rt.block_on(self.inner.submit_header(hex)).map_err(Self::map_err)
    }

    // == Network ==

    pub fn clear_banned(&self) -> Result<()> {
        self.rt.block_on(self.inner.clear_banned()).map_err(Self::map_err)
    }

    pub fn get_added_node_info(&self) -> Result<GetAddedNodeInfo> {
        let g = self.rt.block_on(self.inner.get_added_node_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_addr_man_info(&self) -> Result<GetAddrManInfo> {
        let g = self.rt.block_on(self.inner.get_addr_man_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_connection_count(&self) -> Result<GetConnectionCount> {
        let g = self.rt.block_on(self.inner.get_connection_count()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_net_totals(&self) -> Result<GetNetTotals> {
        let g = self.rt.block_on(self.inner.get_net_totals()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_network_info(&self) -> Result<GetNetworkInfo> {
        let g = self.rt.block_on(self.inner.get_network_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    /// Convenience wrapper bundled into the sync `get_network_info` macro.
    pub fn server_version(&self) -> Result<usize> { Ok(self.get_network_info()?.version) }

    pub fn get_node_addresses(&self) -> Result<GetNodeAddresses> {
        let g = self.rt.block_on(self.inner.get_node_addresses()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_peer_info(&self) -> Result<GetPeerInfo> {
        let g = self.rt.block_on(self.inner.get_peer_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_banned(&self) -> Result<ListBanned> {
        let g = self.rt.block_on(self.inner.list_banned()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn ping(&self) -> Result<()> { self.rt.block_on(self.inner.ping()).map_err(Self::map_err) }

    pub fn set_network_active(&self, state: bool) -> Result<SetNetworkActive> {
        let g = self.rt.block_on(self.inner.set_network_active(state)).map_err(Self::map_err)?;
        reserialize(g)
    }

    // == Raw transactions ==

    pub fn combine_psbt(&self, txs: &[bitcoin::Psbt]) -> Result<CombinePsbt> {
        let txs = txs.iter().map(|p| p.to_string()).collect();
        let g = self.rt.block_on(self.inner.combine_psbt(txs)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn combine_raw_transaction(
        &self,
        txs: &[bitcoin::Transaction],
    ) -> Result<CombineRawTransaction> {
        let txs = txs.iter().map(bitcoin::consensus::encode::serialize_hex).collect();
        let g = self.rt.block_on(self.inner.combine_raw_transaction(txs)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn convert_to_psbt(&self, tx: &bitcoin::Transaction) -> Result<ConvertToPsbt> {
        let hex = bitcoin::consensus::encode::serialize_hex(tx);
        let g = self.rt.block_on(self.inner.convert_to_psbt(hex)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn create_psbt(&self, inputs: &[Input], outputs: &[Output]) -> Result<CreatePsbt> {
        let inputs = serde_json::from_value(into_json(inputs)?)?;
        let outputs = serde_json::from_value(into_json(outputs)?)?;
        let g = self.rt.block_on(self.inner.create_psbt(inputs, outputs)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn create_raw_transaction(
        &self,
        inputs: &[Input],
        outputs: &[Output],
    ) -> Result<CreateRawTransaction> {
        let inputs = serde_json::from_value(into_json(inputs)?)?;
        let outputs = serde_json::from_value(into_json(outputs)?)?;
        let g = self
            .rt
            .block_on(self.inner.create_raw_transaction(inputs, outputs))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn decode_psbt(&self, psbt: &str) -> Result<DecodePsbt> {
        let g =
            self.rt.block_on(self.inner.decode_psbt(psbt.to_string())).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn decode_raw_transaction(
        &self,
        tx: &bitcoin::Transaction,
    ) -> Result<DecodeRawTransaction> {
        let hex = bitcoin::consensus::encode::serialize_hex(tx);
        let g = self.rt.block_on(self.inner.decode_raw_transaction(hex)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn decode_script(&self, script: &str) -> Result<DecodeScript> {
        let g = self
            .rt
            .block_on(self.inner.decode_script(script.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn finalize_psbt(&self, psbt: &bitcoin::Psbt) -> Result<FinalizePsbt> {
        let g =
            self.rt.block_on(self.inner.finalize_psbt(psbt.to_string())).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn fund_raw_transaction(&self, tx: &bitcoin::Transaction) -> Result<FundRawTransaction> {
        let hex = bitcoin::consensus::encode::serialize_hex(tx);
        let g = self.rt.block_on(self.inner.fund_raw_transaction(hex)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_raw_transaction(&self, txid: bitcoin::Txid) -> Result<GetRawTransaction> {
        let g = self
            .rt
            .block_on(self.inner.get_raw_transaction(txid.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_raw_transaction_verbose(&self, txid: Txid) -> Result<GetRawTransactionVerbose> {
        let g = self
            .rt
            .block_on(self.inner.get_raw_transaction_verbose_one(txid.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn send_raw_transaction(&self, tx: &bitcoin::Transaction) -> Result<SendRawTransaction> {
        let hex = bitcoin::consensus::encode::serialize_hex(tx);
        let g = self.rt.block_on(self.inner.send_raw_transaction(hex)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn sign_raw_transaction_with_key(
        &self,
        tx: &bitcoin::Transaction,
        keys: &[bitcoin::PrivateKey],
    ) -> Result<SignRawTransactionWithKey> {
        let hex = bitcoin::consensus::encode::serialize_hex(tx);
        let keys = keys.iter().map(|k| k.to_string()).collect();
        let g = self
            .rt
            .block_on(self.inner.sign_raw_transaction_with_key(hex, keys))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn test_mempool_accept(&self, txs: &[bitcoin::Transaction]) -> Result<TestMempoolAccept> {
        let txs = txs.iter().map(bitcoin::consensus::encode::serialize_hex).collect();
        let g = self.rt.block_on(self.inner.test_mempool_accept(txs)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn analyze_psbt(&self, psbt: &bitcoin::Psbt) -> Result<AnalyzePsbt> {
        let g =
            self.rt.block_on(self.inner.analyze_psbt(psbt.to_string())).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn join_psbts(&self, psbts: &[bitcoin::Psbt]) -> Result<JoinPsbts> {
        let psbts = psbts.iter().map(|p| p.to_string()).collect();
        let g = self.rt.block_on(self.inner.join_psbts(psbts)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn utxo_update_psbt(&self, psbt: &bitcoin::Psbt) -> Result<UtxoUpdatePsbt> {
        let g = self
            .rt
            .block_on(self.inner.utxo_update_psbt(psbt.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn submit_package(
        &self,
        package: &[bitcoin::Transaction],
        _max_fee_rate: Option<bitcoin::FeeRate>,
        _max_burn_amount: Option<bitcoin::Amount>,
    ) -> Result<SubmitPackage> {
        let package = package.iter().map(bitcoin::consensus::encode::serialize_hex).collect();
        let g = self.rt.block_on(self.inner.submit_package(package)).map_err(Self::map_err)?;
        reserialize(g)
    }

    // == Signer ==

    pub fn enumerate_signers(&self) -> Result<EnumerateSigners> {
        let g = self.rt.block_on(self.inner.enumerate_signers()).map_err(Self::map_err)?;
        reserialize(g)
    }

    // == Util ==

    pub fn create_multisig(&self, nrequired: u32, keys: Vec<PublicKey>) -> Result<CreateMultisig> {
        let keys = keys.iter().map(|k| k.to_string()).collect();
        let g = self
            .rt
            .block_on(self.inner.create_multisig(i64::from(nrequired), keys))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_descriptor_info(&self, descriptor: &str) -> Result<GetDescriptorInfo> {
        let g = self
            .rt
            .block_on(self.inner.get_descriptor_info(descriptor.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_index_info(&self) -> Result<GetIndexInfo> {
        let g = self.rt.block_on(self.inner.get_index_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn sign_message_with_privkey(
        &self,
        privkey: &bitcoin::PrivateKey,
        message: &str,
    ) -> Result<SignMessageWithPrivKey> {
        let g = self
            .rt
            .block_on(
                self.inner.sign_message_with_priv_key(privkey.to_string(), message.to_string()),
            )
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn validate_address(&self, address: &Address<NetworkChecked>) -> Result<ValidateAddress> {
        let g = self
            .rt
            .block_on(self.inner.validate_address(address.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn verify_message(
        &self,
        address: &Address<NetworkChecked>,
        signature: &sign_message::MessageSignature,
        message: &str,
    ) -> Result<VerifyMessage> {
        let g = self
            .rt
            .block_on(self.inner.verify_message(
                address.to_string(),
                signature.to_string(),
                message.to_string(),
            ))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    // == Wallet (the methods whose async wrapper takes only simple args) ==

    pub fn abandon_transaction(&self, txid: Txid) -> Result<()> {
        self.rt.block_on(self.inner.abandon_transaction(txid.to_string())).map_err(Self::map_err)
    }

    pub fn abort_rescan(&self) -> Result<AbortRescan> {
        let g = self.rt.block_on(self.inner.abort_rescan()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn backup_wallet(&self, destination: &Path) -> Result<()> {
        let dest = destination.to_string_lossy().into_owned();
        self.rt.block_on(self.inner.backup_wallet(dest)).map_err(Self::map_err)?;
        Ok(())
    }

    pub fn bump_fee(&self, txid: Txid) -> Result<BumpFee> {
        let g = self.rt.block_on(self.inner.bump_fee(txid.to_string())).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn encrypt_wallet(&self, passphrase: &str) -> Result<EncryptWallet> {
        let g = self
            .rt
            .block_on(self.inner.encrypt_wallet(passphrase.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_addresses_by_label(&self, label: &str) -> Result<GetAddressesByLabel> {
        let g = self
            .rt
            .block_on(self.inner.get_addresses_by_label(label.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_address_info(&self, address: &Address) -> Result<GetAddressInfo> {
        let g = self
            .rt
            .block_on(self.inner.get_address_info(address.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_balance(&self) -> Result<GetBalance> {
        let g = self.rt.block_on(self.inner.get_balance()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_balances(&self) -> Result<GetBalances> {
        let g = self.rt.block_on(self.inner.get_balances()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_hd_keys(&self) -> Result<GetHdKeys> {
        let g = self.rt.block_on(self.inner.get_hd_keys()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_received_by_address(
        &self,
        address: &Address<NetworkChecked>,
    ) -> Result<GetReceivedByAddress> {
        let g = self
            .rt
            .block_on(self.inner.get_received_by_address(address.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_received_by_label(&self, label: &str) -> Result<GetReceivedByLabel> {
        let g = self
            .rt
            .block_on(self.inner.get_received_by_label(label.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_raw_change_address(&self) -> Result<GetRawChangeAddress> {
        let g = self.rt.block_on(self.inner.get_raw_change_address()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_transaction(&self, txid: Txid) -> Result<GetTransaction> {
        let g = self
            .rt
            .block_on(self.inner.get_transaction(txid.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn get_wallet_info(&self) -> Result<GetWalletInfo> {
        let g = self.rt.block_on(self.inner.get_wallet_info()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn import_pruned_funds(&self, raw_transaction: &str, tx_out_proof: &str) -> Result<()> {
        self.rt
            .block_on(
                self.inner
                    .import_pruned_funds(raw_transaction.to_string(), tx_out_proof.to_string()),
            )
            .map_err(Self::map_err)
    }

    pub fn key_pool_refill(&self) -> Result<()> {
        self.rt.block_on(self.inner.key_pool_refill()).map_err(Self::map_err)
    }

    pub fn list_address_groupings(&self) -> Result<ListAddressGroupings> {
        let g = self.rt.block_on(self.inner.list_address_groupings()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_descriptors(&self) -> Result<ListDescriptors> {
        let g = self.rt.block_on(self.inner.list_descriptors()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_labels(&self) -> Result<ListLabels> {
        let g = self.rt.block_on(self.inner.list_labels()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_lock_unspent(&self) -> Result<ListLockUnspent> {
        let g = self.rt.block_on(self.inner.list_lock_unspent()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_received_by_address(&self) -> Result<ListReceivedByAddress> {
        let g = self.rt.block_on(self.inner.list_received_by_address()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_received_by_label(&self) -> Result<ListReceivedByLabel> {
        let g = self.rt.block_on(self.inner.list_received_by_label()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_since_block(&self) -> Result<ListSinceBlock> {
        let g = self.rt.block_on(self.inner.list_since_block()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_transactions(&self) -> Result<ListTransactions> {
        let g = self.rt.block_on(self.inner.list_transactions()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_unspent(&self) -> Result<ListUnspent> {
        let g = self.rt.block_on(self.inner.list_unspent()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_wallet_dir(&self) -> Result<ListWalletDir> {
        let g = self.rt.block_on(self.inner.list_wallet_dir()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn list_wallets(&self) -> Result<ListWallets> {
        let g = self.rt.block_on(self.inner.list_wallets()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn load_wallet(&self, wallet: &str) -> Result<LoadWallet> {
        let g =
            self.rt.block_on(self.inner.load_wallet(wallet.to_string())).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn migrate_wallet(&self, _wallet_name: &str) -> Result<MigrateWallet> {
        let g = self.rt.block_on(self.inner.migrate_wallet()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn psbt_bump_fee(&self, txid: &bitcoin::Txid) -> Result<PsbtBumpFee> {
        let g =
            self.rt.block_on(self.inner.psbt_bump_fee(txid.to_string())).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn remove_pruned_funds(&self, txid: Txid) -> Result<()> {
        self.rt.block_on(self.inner.remove_pruned_funds(txid.to_string())).map_err(Self::map_err)
    }

    pub fn rescan_blockchain(&self) -> Result<RescanBlockchain> {
        let g = self.rt.block_on(self.inner.rescan_blockchain()).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn restore_wallet(&self, wallet_name: &str, backup_file: &Path) -> Result<RestoreWallet> {
        let backup = backup_file.to_string_lossy().into_owned();
        let g = self
            .rt
            .block_on(self.inner.restore_wallet(wallet_name.to_string(), backup))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn set_wallet_flag(&self, flag: &str) -> Result<SetWalletFlag> {
        let g = self
            .rt
            .block_on(self.inner.set_wallet_flag(flag.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn sign_raw_transaction_with_wallet(
        &self,
        tx: &bitcoin::Transaction,
    ) -> Result<SignRawTransactionWithWallet> {
        let hex = bitcoin::consensus::encode::serialize_hex(tx);
        let g = self
            .rt
            .block_on(self.inner.sign_raw_transaction_with_wallet(hex))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn simulate_raw_transaction(&self, rawtxs: &[String]) -> Result<SimulateRawTransaction> {
        // `rawtxs` is positionally required by Core even though the spec marks it optional, so route
        // through the `_with` method that actually sends it.
        let opts = crate::client_async::v30::wallet::SimulateRawTransactionOptions {
            raw_txs: Some(rawtxs.to_vec()),
            options: None,
        };
        let g = self
            .rt
            .block_on(self.inner.simulate_raw_transaction_with(opts))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn unload_wallet(&self, wallet: &str) -> Result<UnloadWallet> {
        // The async client talks to the base endpoint, so the wallet name must travel as a parameter.
        let opts = crate::client_async::v30::wallet::UnloadWalletOptions {
            wallet_name: Some(wallet.to_owned()),
            load_on_startup: None,
        };
        let g = self.rt.block_on(self.inner.unload_wallet_with(opts)).map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn wallet_display_address(&self, address: &str) -> Result<WalletDisplayAddress> {
        let g = self
            .rt
            .block_on(self.inner.wallet_display_address(address.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }

    pub fn wallet_lock(&self) -> Result<()> {
        self.rt.block_on(self.inner.wallet_lock()).map_err(Self::map_err)
    }

    pub fn wallet_passphrase(&self, passphrase: &str, timeout: u64) -> Result<()> {
        self.rt
            .block_on(self.inner.wallet_passphrase(passphrase.to_string(), timeout as i64))
            .map_err(Self::map_err)
    }

    pub fn wallet_passphrase_change(
        &self,
        old_passphrase: &str,
        new_passphrase: &str,
    ) -> Result<()> {
        self.rt
            .block_on(
                self.inner.wallet_passphrase_change(
                    old_passphrase.to_string(),
                    new_passphrase.to_string(),
                ),
            )
            .map_err(Self::map_err)
    }

    pub fn wallet_process_psbt(&self, psbt: &bitcoin::Psbt) -> Result<WalletProcessPsbt> {
        let g = self
            .rt
            .block_on(self.inner.wallet_process_psbt(psbt.to_string()))
            .map_err(Self::map_err)?;
        reserialize(g)
    }
}

// ===========================================================================
// Fallback: methods with no generated async wrapper (Generating / Hidden / Zmq sections) or whose
// async shape differs too much (option structs, enum args, bundled convenience methods). These use
// the sync macros, which call the raw transport directly.
// ===========================================================================

crate::impl_client_check_expected_server_version!({ [300000, 300100, 300200] });

// == Blockchain (no clean async wrapper) ==
crate::impl_client_v17__get_block_stats!();
crate::impl_client_v23__get_deployment_info!();
crate::impl_client_v30__get_descriptor_activity!();
crate::impl_client_v21__get_raw_mempool!();
crate::impl_client_v17__estimate_raw_fee!();
crate::impl_client_v25__scan_blocks!();
crate::impl_client_v17__scan_tx_out_set!();
crate::impl_client_v23__get_block_from_peer!();

// == Control == (all migrated above)

// == Generating == (no async wrappers exist)
crate::impl_client_v25__generate_block!();
crate::impl_client_v17__generate_to_address!();
crate::impl_client_v20__generate_to_descriptor!();
crate::impl_client_v17__invalidate_block!();

// == Hidden == (no async wrappers exist)
crate::impl_client_v27__add_connection!();
crate::impl_client_v21__add_peer_address!();
crate::impl_client_v29__get_orphan_txs!();
crate::impl_client_v29__get_orphan_txs_verbosity_1!();
crate::impl_client_v29__get_orphan_txs_verbosity_2!();
crate::impl_client_v26__get_raw_addrman!();
crate::impl_client_v20__mock_scheduler!();
crate::impl_client_v17__reconsider_block!();
crate::impl_client_v17__sync_with_validation_interface_queue!();

// == Mining (complex arg) ==
crate::impl_client_v17__get_block_template!();

// == Network (enum/arg mismatch) ==
crate::impl_client_v17__add_node!();
crate::impl_client_v17__disconnect_node!();
crate::impl_client_v17__set_ban!();

// == Util (bundled multipath / option-struct mode, no clean async) ==
crate::impl_client_v29__derive_addresses!();
crate::impl_client_v17__estimate_smart_fee!();
// list_descriptors migrated above (remove the macro to avoid a duplicate definition).

// == Wallet (option structs / rich Amount-Address args / bundled convenience) ==
crate::impl_client_v17__get_new_address!();
crate::impl_client_v22__create_wallet!();
crate::impl_client_v23__create_wallet!();
crate::impl_client_v28__create_wallet_descriptor!();
crate::impl_client_v21__import_descriptors!();
crate::impl_client_v17__lock_unspent!();
crate::impl_client_v17__send_many!();
crate::impl_client_v21__send_many_verbose!();
crate::impl_client_v17__send_to_address!();
crate::impl_client_v21__send!();
crate::impl_client_v24__send_all!();
crate::impl_client_v17__set_tx_fee!();
crate::impl_client_v17__sign_message!();
crate::impl_client_v17__wallet_create_funded_psbt!();

// == Zmq == (no async wrapper exists)
crate::impl_client_v17__get_zmq_notifications!();
