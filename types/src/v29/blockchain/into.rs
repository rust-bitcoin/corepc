// SPDX-License-Identifier: CC0-1.0

use core::str::FromStr;

use bitcoin::address::NetworkUnchecked;
use bitcoin::consensus::encode;
use bitcoin::hashes::hex::FromHex;
use bitcoin::{
    block, hex, Address, Amount, BlockHash, CompactTarget, ScriptBuf, Target, Txid, Weight, Work,
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
        let weight = Weight::from_wu(self.weight); // FIXME: Confirm this uses weight units.
        let version = block::Version::from_consensus(self.version);
        let tx = self
            .tx
            .iter()
            .map(|t| t.parse::<Txid>().map_err(E::Hash))
            .collect::<Result<Vec<_>, _>>()?;
        let median_time = self.median_time.map(|t| crate::to_u32(t, "median_time")).transpose()?;
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let _target = Some(Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?);
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
            // target,
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
        let _bits = Some(CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?);
        let _target = Some(Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?);
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;
        let prune_height =
            self.prune_height.map(|h| crate::to_u32(h, "prune_height")).transpose()?;
        let prune_target_size =
            self.prune_target_size.map(|h| crate::to_u32(h, "prune_target_size")).transpose()?;
        let softforks = BTreeMap::new(); // TODO: Handle softforks stuff.
        let _signet_challenge = Some(ScriptBuf::from_hex(&self.signet_challenge)?);

        Ok(model::GetBlockchainInfo {
            chain,
            blocks: crate::to_u32(self.blocks, "blocks")?,
            headers: crate::to_u32(self.headers, "headers")?,
            best_block_hash,
            // bits,
            // target,
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
            // signet_challenge,
            warnings: vec![self.warnings],
        })
    }
}

impl Bip9SoftforkStatus {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::Bip9SoftforkStatus {
        use model::Bip9SoftforkStatus::*;

        match self {
            Self::Defined => Defined,
            Self::Started => Started,
            Self::LockedIn => LockedIn,
            Self::Active => Active,
            Self::Failed => Failed,
        }
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
        let chain_work = Work::from_unprefixed_hex(&self.bits).map_err(E::ChainWork)?;
        let _target = Some(Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?);
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
            // target,
            n_tx: self.n_tx,
            previous_block_hash,
            next_block_hash,
        })
    }

    /// Converts json straight to a `bitcoin::BlockHeader`.
    pub fn block_header(self) -> Result<block::Header, hex::HexToArrayError> { todo!() }
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
                        let prevout_spk = convert_script_pubkey(spend.prevout_spk)
                            .map_err(|e| E::ActivityEntry(Box::new(e)))?;

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
                        let output_spk = convert_script_pubkey(receive.output_spk)
                            .map_err(|e| E::ActivityEntry(Box::new(e)))?;

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

// Helper function to convert the raw ScriptPubkey to the model ScriptPubkey
fn convert_script_pubkey(
    spk: ScriptPubkey,
) -> Result<crate::model::ScriptPubkey, GetDescriptorActivityError> {
    use GetDescriptorActivityError as E;

    let script_buf = ScriptBuf::from_hex(&spk.hex).map_err(E::Script)?;
    let address = spk
        .address
        .map(|s| Address::<NetworkUnchecked>::from_str(&s))
        .transpose()
        .map_err(E::Address)?;

    Ok(model::ScriptPubkey { asm: spk.asm, hex: script_buf, type_: spk.type_, address })
}
