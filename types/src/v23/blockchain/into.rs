// SPDX-License-Identifier: CC0-1.0

use alloc::collections::BTreeMap;

use bitcoin::{BlockHash, Network, Txid, Work, Wtxid};

use super::{
    Bip9Info, Bip9Statistics, DeploymentInfo, GetBlockchainInfo, GetBlockchainInfoError,
    GetDeploymentInfo, GetDeploymentInfoError, GetMempoolEntry, MempoolEntry, MempoolEntryError,
};
use crate::model;

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
    /// Returned as part of `getdeploymentinfo`.
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
    /// Returned as part of `getdeploymentinfo`.
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
    /// Returned as part of `getdeploymentinfo`.
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
