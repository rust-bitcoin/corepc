// SPDX-License-Identifier: CC0-1.0

use std::collections::BTreeMap;

use bitcoin::hex::{self, FromHex as _};
use bitcoin::{bip158, Amount, BlockHash, Network, Txid, Work, Wtxid};

use super::error::{
    GetBlockFilterError, GetBlockchainInfoError, MapMempoolEntryError, MempoolEntryError,
    MempoolEntryFeesError,
};
use super::{
    GetBlockFilter, GetBlockchainInfo, GetMempoolAncestors, GetMempoolAncestorsVerbose,
    GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, MempoolEntry,
    MempoolEntryFees,
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
