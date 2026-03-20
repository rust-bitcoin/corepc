// SPDX-License-Identifier: CC0-1.0

use alloc::collections::BTreeMap;

use bitcoin::{hex, Amount, BlockHash, FeeRate, Network, Txid, Weight, Work, Wtxid};

use super::{
    Bip9Info, Bip9Statistics, DeploymentInfo, GetBlockStats, GetBlockchainInfo,
    GetBlockchainInfoError, GetDeploymentInfo, GetDeploymentInfoError, GetMempoolAncestors,
    GetMempoolAncestorsVerbose, GetMempoolDescendants, GetMempoolDescendantsVerbose,
    GetMempoolEntry, GetRawMempool, GetRawMempoolVerbose, MapMempoolEntryError, MempoolEntry,
    MempoolEntryError,
};
use crate::model;
use crate::v23::GetBlockStatsError;

impl GetBlockchainInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&self.chain).map_err(E::Chain)?;
        let best_block_hash =
            self.best_block_hash.parse::<BlockHash>().map_err(E::BestBlockHash)?;
        let time = Some(crate::to_u32(self.time, "time")?);
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
            time,
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

impl GetBlockStats {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockStats, GetBlockStatsError> {
        use GetBlockStatsError as E;

        let average_fee_rate = self.average_fee_rate.and_then(FeeRate::from_sat_per_vb);
        let block_hash =
            self.block_hash.map(|h| h.parse::<BlockHash>()).transpose().map_err(E::BlockHash)?;
        let fee_rate_percentiles = self
            .fee_rate_percentiles
            .map(|arr| arr.iter().map(|vb| FeeRate::from_sat_per_vb(*vb)).collect());
        let max_fee_rate = self.max_fee_rate.and_then(FeeRate::from_sat_per_vb);
        let minimum_fee_rate = self.minimum_fee_rate.and_then(FeeRate::from_sat_per_vb);

        // FIXME: Double check that these values are virtual bytes and not weight units.
        let segwit_total_weight = self.segwit_total_weight.and_then(Weight::from_vb);
        let total_weight = self.total_weight.and_then(Weight::from_vb);

        Ok(model::GetBlockStats {
            average_fee: self.average_fee.map(Amount::from_sat),
            average_fee_rate,
            average_tx_size: self
                .average_tx_size
                .map(|v| crate::to_u32(v, "average_tx_size"))
                .transpose()?,
            block_hash,
            fee_rate_percentiles,
            height: self.height.map(|v| crate::to_u32(v, "height")).transpose()?,
            inputs: self.inputs.map(|v| crate::to_u32(v, "inputs")).transpose()?,
            max_fee: self.max_fee.map(Amount::from_sat),
            max_fee_rate,
            max_tx_size: self.max_tx_size.map(|v| crate::to_u32(v, "max_tx_size")).transpose()?,
            median_fee: self.median_fee.map(Amount::from_sat),
            median_time: self.median_time.map(|v| crate::to_u32(v, "median_time")).transpose()?,
            median_tx_size: self
                .median_tx_size
                .map(|v| crate::to_u32(v, "median_tx_size"))
                .transpose()?,
            minimum_fee: self.minimum_fee.map(Amount::from_sat),
            minimum_fee_rate,
            minimum_tx_size: self
                .minimum_tx_size
                .map(|v| crate::to_u32(v, "minimum_tx_size"))
                .transpose()?,
            outputs: self.outputs.map(|v| crate::to_u32(v, "outputs")).transpose()?,
            subsidy: self.subsidy.map(Amount::from_sat),
            segwit_total_size: self
                .segwit_total_size
                .map(|v| crate::to_u32(v, "segwit_total_size"))
                .transpose()?,
            segwit_total_weight,
            segwit_txs: self.segwit_txs.map(|v| crate::to_u32(v, "segwit_txs")).transpose()?,
            time: self.time.map(|v| crate::to_u32(v, "time")).transpose()?,
            total_out: self.total_out.map(Amount::from_sat),
            total_size: self.total_size.map(|v| crate::to_u32(v, "total_size")).transpose()?,
            total_weight,
            total_fee: self.total_fee.map(Amount::from_sat),
            txs: self.txs.map(|v| crate::to_u32(v, "txs")).transpose()?,
            utxo_increase: self.utxo_increase,
            utxo_size_increase: self.utxo_size_increase,
            utxo_increase_actual: None,      // v25 and later only.
            utxo_size_increase_actual: None, // v25 and later only.
        })
    }
}

impl GetDeploymentInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetDeploymentInfo, GetDeploymentInfoError> {
        let hash = self.hash.parse::<BlockHash>().map_err(GetDeploymentInfoError::BlockHash)?;
        let deployments = self
            .deployments
            .into_iter()
            .map(|(name, dep)| {
                dep.into_model().map(|d| (name, d)).map_err(GetDeploymentInfoError::Deployment)
            })
            .collect::<Result<_, _>>()?;
        Ok(model::GetDeploymentInfo { hash, height: self.height, deployments })
    }
}

impl DeploymentInfo {
    /// Part of `getdeploymentinfo`.
    pub fn into_model(self) -> Result<model::DeploymentInfo, crate::NumericError> {
        Ok(model::DeploymentInfo {
            deployment_type: self.deployment_type,
            height: self.height,
            active: self.active,
            bip9: self.bip9.map(|b| b.into_model()).transpose()?,
        })
    }
}

impl Bip9Info {
    /// Part of `getdeploymentinfo`.
    pub fn into_model(self) -> Result<model::Bip9Info, crate::NumericError> {
        Ok(model::Bip9Info {
            bit: self.bit,
            start_time: self.start_time,
            timeout: self.timeout,
            min_activation_height: self.min_activation_height,
            status: self.status,
            since: self.since,
            status_next: self.status_next,
            statistics: self.statistics.map(|s| s.into_model()).transpose()?,
            signalling: self.signalling,
        })
    }
}

impl Bip9Statistics {
    /// Part of `getdeploymentinfo`.
    pub fn into_model(self) -> Result<model::Bip9Statistics, crate::NumericError> {
        Ok(model::Bip9Statistics {
            period: self.period,
            threshold: self.threshold,
            elapsed: self.elapsed,
            count: self.count,
            possible: self.possible,
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
            unbroadcast: Some(self.unbroadcast),
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
