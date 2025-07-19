// SPDX-License-Identifier: CC0-1.0

use bitcoin::consensus::encode;
use bitcoin::{Address, BlockHash, ScriptBuf, SignedAmount, Transaction, Txid};

use super::{
    GetTransaction, GetTransactionDetail, GetTransactionDetailError, GetTransactionError,
    ListUnspent, ListUnspentItem, ListUnspentItemError,
};
use crate::model;

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
            decoded: self.decoded,
            last_processed_block: None, // v26 and later only.
            tx,
        })
    }
}

impl GetTransactionDetail {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransactionDetail, GetTransactionDetailError> {
        use GetTransactionDetailError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let fee = self.fee.map(|fee| SignedAmount::from_btc(fee).map_err(E::Fee)).transpose()?;

        Ok(model::GetTransactionDetail {
            involves_watch_only: self.involves_watch_only,
            account: self.account,
            address,
            category: self.category.into_model(),
            amount,
            label: self.label,
            vout: self.vout,
            fee,
            abandoned: self.abandoned,
            parent_descriptors: self.parent_descriptors,
        })
    }
}

impl ListUnspent {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListUnspent, ListUnspentItemError> {
        self.0
            .into_iter()
            .map(|item| item.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map(model::ListUnspent)
    }
}

impl ListUnspentItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListUnspentItem, ListUnspentItemError> {
        use ListUnspentItemError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let vout = crate::to_u32(self.vout, "vout")?;
        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let confirmations = crate::to_u32(self.confirmations, "confirmations")?;
        let redeem_script = self
            .redeem_script
            .map(|hex| ScriptBuf::from_hex(&hex).map_err(E::RedeemScript))
            .transpose()?;

        Ok(model::ListUnspentItem {
            txid,
            vout,
            address,
            label: self.label,
            script_pubkey,
            amount,
            confirmations,
            redeem_script,
            spendable: self.spendable,
            solvable: self.solvable,
            descriptor: self.descriptor,
            safe: self.safe,
            parent_descriptors: self.parent_descriptors,
        })
    }
}
