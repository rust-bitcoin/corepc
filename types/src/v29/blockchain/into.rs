// SPDX-License-Identifier: CC0-1.0

use alloc::collections::BTreeMap;
use core::str::FromStr;

use bitcoin::consensus::encode;
use bitcoin::hashes::hex::FromHex;
use bitcoin::{
    block, hex, Amount, BlockHash, CompactTarget, ScriptBuf, Target, Txid, Weight, Work,
};

// TODO: Use explicit imports?
use super::*;

impl GetBlockVerboseOne {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockVerboseOne, GetBlockVerboseOneError> {
        use GetBlockVerboseOneError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;
        let stripped_size =
            self.stripped_size.map(|size| crate::to_u32(size, "stripped_size")).transpose()?;
        let weight = Weight::from_wu(self.weight);
        let version = block::Version::from_consensus(self.version);
        let tx = self
            .tx
            .iter()
            .map(|t| t.parse::<Txid>().map_err(E::Hash))
            .collect::<Result<Vec<_>, _>>()?;
        let median_time = self.median_time.map(|t| crate::to_u32(t, "median_time")).transpose()?;
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let target = Some(Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?);
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;
        let previous_block_hash = self
            .previous_block_hash
            .map(|s| s.parse::<BlockHash>())
            .transpose()
            .map_err(E::PreviousBlockHash)?;
        let next_block_hash = self
            .next_block_hash
            .map(|s| s.parse::<BlockHash>())
            .transpose()
            .map_err(E::NextBlockHash)?;

        Ok(model::GetBlockVerboseOne {
            hash,
            confirmations: self.confirmations,
            size: crate::to_u32(self.size, "size")?,
            stripped_size,
            weight,
            height: crate::to_u32(self.height, "height")?,
            version,
            merkle_root: self.merkle_root, // TODO: Use hash, which one depends on segwit or not.
            tx,
            time: crate::to_u32(self.time, "time")?,
            median_time,
            nonce: crate::to_u32(self.nonce, "nonce")?,
            bits,
            target,
            difficulty: self.difficulty,
            chain_work,
            n_tx: crate::to_u32(self.n_tx, "n_tx")?,
            previous_block_hash,
            next_block_hash,
        })
    }
}

impl GetBlockchainInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&self.chain).map_err(E::Chain)?;
        let best_block_hash =
            self.best_block_hash.parse::<BlockHash>().map_err(E::BestBlockHash)?;
        let bits = Some(CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?);
        let target = Some(Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?);
        let time = Some(crate::to_u32(self.time, "time")?);
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;
        let prune_height =
            self.prune_height.map(|h| crate::to_u32(h, "prune_height")).transpose()?;
        let prune_target_size =
            self.prune_target_size.map(|h| crate::to_u32(h, "prune_target_size")).transpose()?;
        let signet_challenge =
            self.signet_challenge.as_ref().map(|s| ScriptBuf::from_hex(s)).transpose()?;

        Ok(model::GetBlockchainInfo {
            chain,
            blocks: crate::to_u32(self.blocks, "blocks")?,
            headers: crate::to_u32(self.headers, "headers")?,
            best_block_hash,
            bits,
            target,
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
            softforks: BTreeMap::new(),
            signet_challenge,
            warnings: self.warnings,
        })
    }
}

impl GetBlockHeader {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockHeader, GetBlockHeaderError> {
        use GetBlockHeaderError as E;

        let v = Vec::from_hex(&self.0).map_err(E::Hex)?;
        let header = encode::deserialize::<block::Header>(&v).map_err(E::Consensus)?;

        Ok(model::GetBlockHeader(header))
    }

    /// Converts json straight to a `bitcoin::BlockHeader`.
    pub fn block_header(self) -> Result<block::Header, GetBlockHeaderError> {
        Ok(self.into_model()?.0)
    }
}

impl GetBlockHeaderVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockHeaderVerbose, GetBlockHeaderVerboseError> {
        use GetBlockHeaderVerboseError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;
        let version = block::Version::from_consensus(self.version);
        let merkle_root = self.merkle_root.parse::<TxMerkleNode>().map_err(E::MerkleRoot)?;
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;
        let target = Some(Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?);
        let previous_block_hash = self
            .previous_block_hash
            .map(|s| s.parse::<BlockHash>().map_err(E::PreviousBlockHash))
            .transpose()?;
        let next_block_hash = self
            .next_block_hash
            .map(|s| s.parse::<BlockHash>().map_err(E::NextBlockHash))
            .transpose()?;

        Ok(model::GetBlockHeaderVerbose {
            hash,
            confirmations: self.confirmations,
            height: crate::to_u32(self.height, "height")?,
            version,
            merkle_root,
            time: crate::to_u32(self.time, "time")?,
            median_time: crate::to_u32(self.median_time, "median_time")?,
            nonce: crate::to_u32(self.nonce, "nonce")?,
            bits,
            difficulty: self.difficulty,
            chain_work,
            target,
            n_tx: self.n_tx,
            previous_block_hash,
            next_block_hash,
        })
    }

    /// Converts json straight to a `bitcoin::BlockHeader`.
    pub fn block_header(self) -> Result<block::Header, hex::HexToArrayError> { todo!() }
}

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
                        bits: Some(CompactTarget::from_unprefixed_hex(&s.bits).map_err(E::Bits)?),
                        target: Some(
                            Target::from_unprefixed_hex(s.target.as_ref()).map_err(E::Target)?,
                        ),
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

impl GetDescriptorActivity {
    /// Converts the raw JSON-RPC `GetDescriptorActivity` type into the strongly-typed model version.
    pub fn into_model(self) -> Result<model::GetDescriptorActivity, GetDescriptorActivityError> {
        use GetDescriptorActivityError as E;

        let activities = self
            .activity
            .into_iter()
            .map(|entry| -> Result<model::ActivityEntry, GetDescriptorActivityError> {
                match entry {
                    ActivityEntry::Spend(spend) => {
                        let amount = Amount::from_btc(spend.amount).map_err(E::Amount)?;
                        let block_hash = spend
                            .block_hash
                            .map(|s| BlockHash::from_str(&s))
                            .transpose()
                            .map_err(E::Hash)?;
                        let height =
                            spend.height.map(|h| crate::to_u32(h, "height")).transpose()?;
                        let spend_txid = Txid::from_str(&spend.spend_txid).map_err(E::Hash)?;
                        let prevout_txid = Txid::from_str(&spend.prevout_txid).map_err(E::Hash)?;
                        let prevout_spk = spend.prevout_spk.into_model().map_err(E::PrevoutSpk)?;

                        Ok(model::ActivityEntry::Spend(model::SpendActivity {
                            amount,
                            block_hash,
                            height,
                            spend_txid,
                            spend_vout: spend.spend_vout,
                            prevout_txid,
                            prevout_vout: spend.prevout_vout,
                            prevout_spk,
                        }))
                    }
                    ActivityEntry::Receive(receive) => {
                        let amount = Amount::from_btc(receive.amount).map_err(E::Amount)?;
                        let block_hash = receive
                            .block_hash
                            .map(|s| BlockHash::from_str(&s))
                            .transpose()
                            .map_err(E::Hash)?;
                        let height =
                            receive.height.map(|h| crate::to_u32(h, "height")).transpose()?; // Uses From<NumericError>
                        let txid = Txid::from_str(&receive.txid).map_err(E::Hash)?;
                        let output_spk = receive.output_spk.into_model().map_err(E::OutputSpk)?;

                        Ok(model::ActivityEntry::Receive(model::ReceiveActivity {
                            amount,
                            block_hash,
                            height,
                            txid,
                            vout: receive.vout,
                            output_spk,
                        }))
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::GetDescriptorActivity { activity: activities })
    }
}
