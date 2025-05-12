// SPDX-License-Identifier: CC0-1.0

use std::collections::BTreeMap;

use bitcoin::hex::{self, FromHex as _};
use bitcoin::{bip158, Amount, BlockHash, Network, ScriptBuf, Txid, Work, Wtxid};

use super::error::{
    GetBlockFilterError, GetBlockchainInfoError, MapMempoolEntryError, MempoolEntryError,
    MempoolEntryFeesError,
};
use super::{
    GetBlockFilter, GetBlockchainInfo, GetChainTxStats, GetChainTxStatsError, GetMempoolAncestors,
    GetMempoolAncestorsVerbose, GetMempoolDescendants, GetMempoolDescendantsVerbose,
    GetMempoolEntry, GetMempoolInfo, GetMempoolInfoError, GetRawMempool, GetRawMempoolVerbose,
    MempoolEntry, MempoolEntryFees, ScanTxOutSetError, ScanTxOutSetStart, ScanTxOutSetUnspent,
};
use crate::model;

impl GetBlockchainInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&self.chain).map_err(E::Chain)?;
        let best_block_hash =
            self.best_block_hash.parse::<BlockHash>().map_err(E::BestBlockHash)?;
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;
        let prune_height =
            self.prune_height.map(|h| crate::to_u32(h, "prune_height")).transpose()?;
        let prune_target_size =
            self.prune_target_size.map(|h| crate::to_u32(h, "prune_target_size")).transpose()?;
        let softforks = BTreeMap::new(); // TODO: Handle softforks stuff.

        Ok(model::GetBlockchainInfo {
            chain,
            blocks: crate::to_u32(self.blocks, "blocks")?,
            headers: crate::to_u32(self.headers, "headers")?,
            best_block_hash,
            bits: None,
            target: None,
            difficulty: self.difficulty,
            time: None,
            median_time: crate::to_u32(self.median_time, "median_time")?,
            verification_progress: self.verification_progress,
            initial_block_download: self.initial_block_download,
            chain_work,
            size_on_disk: self.size_on_disk,
            pruned: self.pruned,
            prune_height,
            automatic_pruning: self.automatic_pruning,
            prune_target_size,
            softforks,
            signet_challenge: None,
            warnings: vec![self.warnings],
        })
    }
}

impl GetBlockFilter {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockFilter, GetBlockFilterError> {
        use GetBlockFilterError as E;

        let filter = Vec::from_hex(&self.filter).map_err(E::Filter)?;
        let header = self.header.parse::<bip158::FilterHash>().map_err(E::Header)?;
        Ok(model::GetBlockFilter { filter, header })
    }
}

impl GetChainTxStats {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetChainTxStats, GetChainTxStatsError> {
        use GetChainTxStatsError as E;

        let window_final_block_hash =
            self.window_final_block_hash.parse::<BlockHash>().map_err(E::WindowFinalBlockHash)?;
        let window_final_block_height =
            crate::to_u32(self.window_final_block_height, "window_final_block_height")?;
        let window_tx_count =
            self.window_tx_count.map(|h| crate::to_u32(h, "window_tx_count")).transpose()?;
        let window_interval =
            self.window_interval.map(|h| crate::to_u32(h, "window_interval")).transpose()?;
        let tx_rate = self.tx_rate.map(|h| crate::to_u32(h, "tx_rate")).transpose()?;

        Ok(model::GetChainTxStats {
            time: crate::to_u32(self.time, "time")?,
            tx_count: crate::to_u32(self.tx_count, "tx_count")?,
            window_final_block_hash,
            window_final_block_height: Some(window_final_block_height),
            window_block_count: crate::to_u32(self.window_block_count, "window_block_count")?,
            window_tx_count,
            window_interval,
            tx_rate,
        })
    }
}

impl GetMempoolAncestors {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolAncestors, hex::HexToArrayError> {
        let v = self.0.iter().map(|t| t.parse::<Txid>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetMempoolAncestors(v))
    }
}

impl GetMempoolAncestorsVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolAncestorsVerbose, MapMempoolEntryError> {
        use MapMempoolEntryError as E;

        let mut map = BTreeMap::new();
        for (k, v) in self.0.into_iter() {
            let txid = k.parse::<Txid>().map_err(E::Txid)?;
            let relative = v.into_model().map_err(E::MempoolEntry)?;
            map.insert(txid, relative);
        }
        Ok(model::GetMempoolAncestorsVerbose(map))
    }
}

impl GetMempoolDescendants {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolDescendants, hex::HexToArrayError> {
        let v = self.0.iter().map(|t| t.parse::<Txid>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetMempoolDescendants(v))
    }
}

impl GetMempoolDescendantsVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolDescendantsVerbose, MapMempoolEntryError> {
        use MapMempoolEntryError as E;

        let mut map = BTreeMap::new();
        for (k, v) in self.0.into_iter() {
            let txid = k.parse::<Txid>().map_err(E::Txid)?;
            let relative = v.into_model().map_err(E::MempoolEntry)?;
            map.insert(txid, relative);
        }
        Ok(model::GetMempoolDescendantsVerbose(map))
    }
}

impl GetMempoolEntry {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolEntry, MempoolEntryError> {
        Ok(model::GetMempoolEntry(self.0.into_model()?))
    }
}

impl MempoolEntry {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolEntry, MempoolEntryError> {
        use MempoolEntryError as E;

        let vsize = Some(crate::to_u32(self.vsize, "vsize")?);
        let size = None;
        let weight = Some(crate::to_u32(self.weight, "weight")?);
        let time = crate::to_u32(self.time, "time")?;
        let height = crate::to_u32(self.height, "height")?;
        let descendant_count = crate::to_u32(self.descendant_count, "descendant_count")?;
        let descendant_size = crate::to_u32(self.descendant_size, "descendant_size")?;
        let ancestor_count = crate::to_u32(self.ancestor_count, "ancestor_count")?;
        let ancestor_size = crate::to_u32(self.ancestor_size, "ancestor_size")?;
        let wtxid = self.wtxid.parse::<Wtxid>().map_err(E::Wtxid)?;
        let fees = self.fees.into_model().map_err(E::Fees)?;
        let depends = self
            .depends
            .iter()
            .map(|txid| txid.parse::<Txid>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Depends)?;
        let spent_by = self
            .spent_by
            .iter()
            .map(|txid| txid.parse::<Txid>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::SpentBy)?;

        Ok(model::MempoolEntry {
            vsize,
            size,
            weight,
            time,
            height,
            descendant_count,
            descendant_size,
            ancestor_count,
            ancestor_size,
            wtxid,
            fees,
            depends,
            spent_by,
            bip125_replaceable: Some(self.bip125_replaceable),
            unbroadcast: None,
        })
    }
}

impl MempoolEntryFees {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolEntryFees, MempoolEntryFeesError> {
        use MempoolEntryFeesError as E;

        Ok(model::MempoolEntryFees {
            base: Amount::from_btc(self.base).map_err(E::Base)?,
            modified: Amount::from_btc(self.modified).map_err(E::Modified)?,
            ancestor: Amount::from_btc(self.ancestor).map_err(E::MempoolEntry)?,
            descendant: Amount::from_btc(self.descendant).map_err(E::Descendant)?,
        })
    }
}

impl GetMempoolInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolInfo, GetMempoolInfoError> {
        let size = crate::to_u32(self.size, "size")?;
        let bytes = crate::to_u32(self.bytes, "bytes")?;
        let usage = crate::to_u32(self.usage, "usage")?;
        let max_mempool = crate::to_u32(self.max_mempool, "max_mempool")?;
        let mempool_min_fee = crate::btc_per_kb(self.mempool_min_fee)?;
        let min_relay_tx_fee = crate::btc_per_kb(self.min_relay_tx_fee)?;

        Ok(model::GetMempoolInfo {
            loaded: Some(self.loaded),
            size,
            bytes,
            usage,
            total_fee: None,
            max_mempool,
            mempool_min_fee,
            min_relay_tx_fee,
            incremental_relay_fee: None,
            unbroadcast_count: None,
            full_rbf: None,
            permit_bare_multisig: None,
            max_data_carrier_size: None,
        })
    }
}

impl GetRawMempool {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetRawMempool, hex::HexToArrayError> {
        let v = self.0.iter().map(|t| t.parse::<Txid>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetRawMempool(v))
    }
}

impl GetRawMempoolVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetRawMempoolVerbose, MapMempoolEntryError> {
        use MapMempoolEntryError as E;

        let mut map = BTreeMap::new();
        for (k, v) in self.0.into_iter() {
            let txid = k.parse::<Txid>().map_err(E::Txid)?;
            let relative = v.into_model().map_err(E::MempoolEntry)?;
            map.insert(txid, relative);
        }
        Ok(model::GetRawMempoolVerbose(map))
    }
}

impl ScanTxOutSetStart {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ScanTxOutSetStart, ScanTxOutSetError> {
        use ScanTxOutSetError as E;

        let bestblock = self.best_block.parse::<BlockHash>().map_err(E::BestBlockHash)?;

        let unspents = self
            .unspents
            .into_iter()
            .map(|u| u.into_model())
            .collect::<Result<Vec<_>, _>>()?;

        let total_amount = Amount::from_btc(self.total_amount).map_err(E::TotalAmount)?;

        Ok(model::ScanTxOutSetStart {
            success: Some(self.success),
            txouts: Some(self.txouts),
            height: Some(self.height),
            bestblock: Some(bestblock),
            unspents,
            total_amount,
        })
    }
}

impl ScanTxOutSetUnspent {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ScanTxOutSetUnspent, ScanTxOutSetError> {
        use ScanTxOutSetError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubKey)?;

        Ok(model::ScanTxOutSetUnspent {
            txid,
            vout: self.vout,
            script_pubkey,
            desc: Some(self.descriptor),
            amount,
            coinbase: None,
            height: self.height,
            blockhash: None,
            confirmations: None,
        })
    }
}
