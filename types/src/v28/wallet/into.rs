// SPDX-License-Identifier: CC0-1.0

use bitcoin::consensus::encode;
use bitcoin::hashes::hash160;
use bitcoin::hex::FromHex;
use bitcoin::key::PublicKey;
use bitcoin::{
    bip32, Address, BlockHash, ScriptBuf, SignedAmount, Transaction, Txid, WitnessProgram,
    WitnessVersion,
};

use super::{
    GetAddressInfo, GetAddressInfoEmbedded, GetAddressInfoEmbeddedError, GetAddressInfoError,
    GetHdKeys, GetHdKeysError, GetTransaction, GetTransactionError, ListSinceBlock,
    ListSinceBlockError, ListTransactions, TransactionItem, TransactionItemError,
};
use crate::model;

impl GetAddressInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfo, GetAddressInfoError> {
        use GetAddressInfoError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program)
        {
            (Some(v), Some(hex)) => {
                if v > u8::MAX as i64 || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version =
                    WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = Vec::from_hex(&hex).map_err(E::WitnessProgramBytes)?;
                let witness_program =
                    WitnessProgram::new(witness_version, &bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            }
            _ => (None, None), // TODO: Think more if catchall is ok.
        };
        let script = self.script.map(|s| s.into_model());
        let redeem_script =
            self.hex.map(|hex| ScriptBuf::from_hex(&hex).map_err(E::Hex)).transpose()?;
        let pubkeys = self
            .pubkeys
            .map(|pubkeys| {
                pubkeys
                    .iter()
                    .map(|s| s.parse::<PublicKey>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(E::Pubkeys)
            })
            .transpose()?;
        let sigs_required =
            self.sigs_required.map(|s| crate::to_u32(s, "sigs_required")).transpose()?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).transpose().map_err(E::Pubkey)?;
        let embedded =
            self.embedded.map(|embedded| embedded.into_model()).transpose().map_err(E::Embedded)?;
        let hd_key_path = self
            .hd_key_path
            .map(|s| s.parse::<bip32::DerivationPath>())
            .transpose()
            .map_err(E::HdKeyPath)?;
        let hd_seed_id =
            self.hd_seed_id.map(|s| s.parse::<hash160::Hash>()).transpose().map_err(E::HdSeedId)?;
        let hd_master_fingerprint = self
            .hd_master_fingerprint
            .map(|s| s.parse::<bip32::Fingerprint>())
            .transpose()
            .map_err(E::HdMasterFingerprint)?;

        Ok(model::GetAddressInfo {
            address,
            script_pubkey,
            is_mine: self.is_mine,
            is_watch_only: self.is_watch_only,
            solvable: Some(self.solvable),
            descriptor: self.descriptor,
            parent_descriptor: self.parent_descriptor,
            is_script: self.is_script,
            is_change: Some(self.is_change),
            is_witness: self.is_witness,
            witness_version,
            witness_program,
            script,
            hex: redeem_script,
            pubkeys,
            sigs_required,
            pubkey,
            embedded,
            is_compressed: self.is_compressed,
            label: None,
            timestamp: self.timestamp,
            hd_key_path,
            hd_seed_id,
            hd_master_fingerprint,
            labels: self.labels,
        })
    }
}

impl GetAddressInfoEmbedded {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfoEmbedded, GetAddressInfoEmbeddedError> {
        use GetAddressInfoEmbeddedError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program)
        {
            (Some(v), Some(hex)) => {
                if v > u8::MAX as i64 || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version =
                    WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = Vec::from_hex(&hex).map_err(E::WitnessProgramBytes)?;
                let witness_program =
                    WitnessProgram::new(witness_version, &bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            }
            _ => (None, None), // TODO: Think more if catchall is ok.
        };
        let script = self.script.map(|s| s.into_model());
        let redeem_script =
            self.hex.map(|hex| ScriptBuf::from_hex(&hex).map_err(E::Hex)).transpose()?;
        let pubkeys = None;
        let sigs_required =
            self.sigs_required.map(|s| crate::to_u32(s, "sigs_required")).transpose()?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).transpose().map_err(E::Pubkey)?;

        Ok(model::GetAddressInfoEmbedded {
            address,
            script_pubkey,
            solvable: self.solvable,
            descriptor: self.descriptor,
            parent_descriptor: self.parent_descriptor,
            is_script: self.is_script,
            is_change: self.is_change,
            is_witness: self.is_witness,
            witness_version,
            witness_program,
            script,
            hex: redeem_script,
            pubkeys,
            sigs_required,
            pubkey,
            is_compressed: self.is_compressed,
            label: None,
            labels: self.labels,
        })
    }
}

impl GetHdKeys {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetHdKeys, GetHdKeysError> {
        let keys = self
            .0
            .into_iter()
            .map(|item| {
                let xpub = item.xpub.parse().map_err(GetHdKeysError::Xpub)?;
                let xpriv = match item.xpriv {
                    Some(xpriv) => Some(xpriv.parse().map_err(GetHdKeysError::Xpriv)?),
                    None => None,
                };
                let descriptors = item
                    .descriptors
                    .into_iter()
                    .map(|desc| model::HdKeyDescriptor {
                        descriptor: desc.descriptor,
                        active: desc.active,
                    })
                    .collect();
                Ok(model::HdKey { xpub, has_private: item.has_private, xpriv, descriptors })
            })
            .collect::<Result<Vec<_>, GetHdKeysError>>()?;
        Ok(model::GetHdKeys(keys))
    }
}

impl GetTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransaction, GetTransactionError> {
        use GetTransactionError as E;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let fee = self.fee.map(|fee| SignedAmount::from_btc(fee).map_err(E::Fee)).transpose()?;
        let block_hash =
            self.block_hash.map(|s| s.parse::<BlockHash>().map_err(E::BlockHash)).transpose()?;
        let block_index =
            self.block_index.map(|idx| crate::to_u32(idx, "block_index")).transpose()?;
        let block_height =
            self.block_height.map(|h| crate::to_u32(h, "block_height")).transpose()?;
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wtxid = self.wtxid.map(|s| s.parse::<Txid>().map_err(E::Wtxid)).transpose()?;
        let wallet_conflicts = self
            .wallet_conflicts
            .into_iter()
            .map(|s| s.parse::<Txid>().map_err(E::WalletConflicts))
            .collect::<Result<Vec<_>, _>>()?;
        let replaced_by_txid = self
            .replaced_by_txid
            .map(|s| s.parse::<Txid>().map_err(E::ReplacedByTxid))
            .transpose()?;
        let replaces_txid =
            self.replaces_txid.map(|s| s.parse::<Txid>().map_err(E::ReplacesTxid)).transpose()?;
        let tx = encode::deserialize_hex::<Transaction>(&self.hex).map_err(E::Tx)?;
        let details = self
            .details
            .into_iter()
            .map(|d| d.into_model().map_err(E::Details))
            .collect::<Result<Vec<_>, _>>()?;
        let decoded = self
            .decoded
            .map(|tx| encode::deserialize_hex::<Transaction>(&tx).map_err(E::Tx))
            .transpose()?;
        let last_processed_block = self
            .last_processed_block
            .map(|l| l.into_model())
            .transpose()
            .map_err(E::LastProcessedBlock)?;

        Ok(model::GetTransaction {
            amount,
            fee,
            confirmations: self.confirmations,
            generated: self.generated,
            trusted: self.trusted,
            block_hash,
            block_height,
            block_index,
            block_time: self.block_time,
            txid,
            wtxid,
            wallet_conflicts,
            replaced_by_txid,
            replaces_txid,
            mempool_conflicts: None, // v28 and later only.
            to: self.to,
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable: self.bip125_replaceable.into_model(),
            parent_descriptors: self.parent_descriptors,
            details,
            decoded,
            last_processed_block,
            tx,
        })
    }
}

impl ListSinceBlock {
    pub fn into_model(self) -> Result<model::ListSinceBlock, ListSinceBlockError> {
        use ListSinceBlockError as E;

        let transactions = self
            .transactions
            .into_iter()
            .map(|t| t.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Transactions)?;
        let removed = self
            .removed
            .into_iter()
            .map(|t| t.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Removed)?;
        let last_block = self.last_block.parse::<BlockHash>().map_err(E::LastBlock)?;

        Ok(model::ListSinceBlock { transactions, removed, last_block })
    }
}

impl TransactionItem {
    pub fn into_model(self) -> Result<model::TransactionItem, TransactionItemError> {
        use TransactionItemError as E;

        let address =
            self.address.map(|s| s.parse::<Address<_>>().map_err(E::Address)).transpose()?;
        let category = self.category.into_model();
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let vout = crate::to_u32(self.vout, "vout")?;
        let fee = self
            .fee
            .map(|f| SignedAmount::from_btc(f).map_err(E::Fee))
            .transpose()? // optional historically
            .unwrap_or_else(|| SignedAmount::from_sat(0));
        let block_hash =
            self.block_hash.map(|s| s.parse::<BlockHash>().map_err(E::BlockHash)).transpose()?;
        let block_height =
            self.block_height.map(|h| crate::to_u32(h, "block_height")).transpose()?;
        let block_index = self.block_index.map(|h| crate::to_u32(h, "block_index")).transpose()?;
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wtxid = self.wtxid.parse::<Txid>().map_err(E::Wtxid)?;
        let wallet_conflicts = self
            .wallet_conflicts
            .into_iter()
            .map(|s| s.parse::<Txid>().map_err(E::WalletConflicts))
            .collect::<Result<Vec<_>, _>>()?;
        let replaced_by_txid = self
            .replaced_by_txid
            .map(|s| s.parse::<Txid>().map_err(E::ReplacedByTxid))
            .transpose()?;
        let replaces_txid =
            self.replaces_txid.map(|s| s.parse::<Txid>().map_err(E::ReplacesTxid)).transpose()?;
        let mempool_conflicts = self
            .mempool_conflicts
            .map(|v| v.into_iter().filter_map(|s| s.parse::<Txid>().ok()).collect::<Vec<_>>());
        let bip125_replaceable = self.bip125_replaceable.into_model();

        Ok(model::TransactionItem {
            involves_watch_only: self.involves_watch_only,
            address,
            category,
            amount,
            vout,
            fee,
            confirmations: self.confirmations,
            generated: self.generated,
            trusted: self.trusted,
            block_hash,
            block_height,
            block_index,
            block_time: self.block_time,
            txid: Some(txid),
            wtxid: Some(wtxid),
            wallet_conflicts: Some(wallet_conflicts),
            replaced_by_txid,
            replaces_txid,
            mempool_conflicts,
            to: self.to,
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable,
            parent_descriptors: self.parent_descriptors,
            abandoned: self.abandoned,
            label: self.label,
        })
    }
}

impl ListTransactions {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListTransactions, TransactionItemError> {
        let transactions =
            self.0.into_iter().map(|tx| tx.into_model()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::ListTransactions(transactions))
    }
}
