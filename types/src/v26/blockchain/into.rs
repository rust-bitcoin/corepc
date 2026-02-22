// SPDX-License-Identifier: CC0-1.0

use bitcoin::hashes::sha256;
use bitcoin::{Amount, BlockHash};

use super::{
    DumpTxOutSet, DumpTxOutSetError, GetChainStates, GetChainStatesError, GetTxOutSetInfo,
    GetTxOutSetInfoError, LoadTxOutSet, LoadTxOutSetError, ScanBlocksStart,
};
use crate::model;
use crate::v25::ScanBlocksStartError;

impl GetChainStates {
    /// Converts v26 GetChainStates (and its ChainState subtypes) to model::GetChainStates
    pub fn into_model(self) -> Result<model::GetChainStates, GetChainStatesError> {
        use GetChainStatesError as E;

        Ok(model::GetChainStates {
            headers: crate::to_u32(self.headers, "headers")?,
            chain_states: self
                .chain_states
                .into_iter()
                .map(|s| {
                    Ok(model::ChainState {
                        blocks: crate::to_u32(s.blocks, "blocks")?,
                        best_block_hash: s.best_block_hash.parse().map_err(E::BestBlockHash)?,
                        bits: None,   // v29 and later only.
                        target: None, // v29 and later only.
                        difficulty: s.difficulty,
                        verification_progress: s.verification_progress,
                        snapshot_block_hash: match s.snapshot_block_hash {
                            Some(s) => Some(s.parse().map_err(E::SnapshotBlockHash)?),
                            None => None,
                        },
                        coins_db_cache_bytes: s.coins_db_cache_bytes,
                        coins_tip_cache_bytes: s.coins_tip_cache_bytes,
                        validated: s.validated,
                    })
                })
                .collect::<Result<_, E>>()?,
        })
    }
}

impl DumpTxOutSet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DumpTxOutSet, DumpTxOutSetError> {
        use DumpTxOutSetError as E;

        let coins_written = Amount::from_btc(self.coins_written).map_err(E::CoinsWritten)?;
        let base_hash = self.base_hash.parse::<BlockHash>().map_err(E::BaseHash)?;
        let base_height = crate::to_u32(self.base_height, "base_height")?;
        let tx_out_set_hash =
            self.tx_out_set_hash.parse::<sha256::Hash>().map_err(E::TxOutSetHash)?;
        let n_chain_tx = crate::to_u32(self.n_chain_tx, "n_chain_tx")?;

        Ok(model::DumpTxOutSet {
            coins_written,
            base_hash,
            base_height,
            path: self.path,
            tx_out_set_hash,
            n_chain_tx,
        })
    }
}

impl GetTxOutSetInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxOutSetInfo, GetTxOutSetInfoError> {
        use GetTxOutSetInfoError as E;

        let height = crate::to_u32(self.height, "height")?;
        let best_block = self.best_block.parse::<BlockHash>().map_err(E::BestBlock)?;
        let transactions =
            self.transactions.map(|v| crate::to_u32(v, "transactions")).transpose()?;
        let tx_outs = crate::to_u32(self.tx_outs, "tx_outs")?;
        let bogo_size = crate::to_u32(self.bogo_size, "bogo_size")?;
        let disk_size = self.disk_size.map(|v| crate::to_u32(v, "disk_size")).transpose()?;
        let total_amount = Amount::from_btc(self.total_amount).map_err(E::TotalAmount)?;
        let total_unspendable_amount = self
            .total_unspendable_amount
            .map(|v| Amount::from_btc(v).map_err(E::TotalUnspendableAmount))
            .transpose()?;
        let block_info = match self.block_info {
            Some(b) => {
                let prevout_spent = Amount::from_btc(b.prevout_spent).map_err(E::PrevoutSpent)?;
                let coinbase = Amount::from_btc(b.coinbase).map_err(E::Coinbase)?;
                let new_outputs_ex_coinbase =
                    Amount::from_btc(b.new_outputs_ex_coinbase).map_err(E::NewOutputsExCoinbase)?;
                let unspendable = Amount::from_btc(b.unspendable).map_err(E::Unspendable)?;
                let unspendables = model::GetTxOutSetInfoUnspendables {
                    genesis_block: Amount::from_btc(b.unspendables.genesis_block)
                        .map_err(E::UnspendablesGenesisBlock)?,
                    bip30: Amount::from_btc(b.unspendables.bip30).map_err(E::UnspendablesBip30)?,
                    scripts: Amount::from_btc(b.unspendables.scripts)
                        .map_err(E::UnspendablesScripts)?,
                    unclaimed_rewards: Amount::from_btc(b.unspendables.unclaimed_rewards)
                        .map_err(E::UnspendablesUnclaimedRewards)?,
                };

                Some(model::GetTxOutSetInfoBlockInfo {
                    prevout_spent,
                    coinbase,
                    new_outputs_ex_coinbase,
                    unspendable,
                    unspendables,
                })
            }
            None => None,
        };

        Ok(model::GetTxOutSetInfo {
            height,
            best_block,
            transactions,
            tx_outs,
            bogo_size,
            hash_serialized_2: None, // v17 to v25 only.
            hash_serialized_3: self.hash_serialized_3,
            disk_size,
            total_amount,
            muhash: self.muhash,
            total_unspendable_amount,
            block_info,
        })
    }
}

impl LoadTxOutSet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::LoadTxOutSet, LoadTxOutSetError> {
        use LoadTxOutSetError as E;

        let tip_hash = self.tip_hash.parse::<BlockHash>().map_err(E::TipHash)?;
        let base_height = crate::to_u32(self.base_height, "base_height")?;
        let coins_loaded = Amount::from_btc(self.coins_loaded).map_err(E::CoinsLoaded)?;

        Ok(model::LoadTxOutSet { coins_loaded, tip_hash, base_height, path: self.path })
    }
}

impl ScanBlocksStart {
    pub fn into_model(self) -> Result<model::ScanBlocksStart, ScanBlocksStartError> {
        use ScanBlocksStartError as E;

        let relevant_blocks = self
            .relevant_blocks
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::RelevantBlocks)?;

        Ok(model::ScanBlocksStart {
            from_height: crate::to_u32(self.from_height, "from_height")?,
            to_height: crate::to_u32(self.to_height, "to_height")?,
            relevant_blocks,
            completed: Some(self.completed),
        })
    }
}
