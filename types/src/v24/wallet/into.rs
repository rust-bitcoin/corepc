// SPDX-License-Identifier: CC0-1.0

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::{Address, Amount, BlockHash, ScriptBuf, SignedAmount, Transaction, Txid};

use super::{
    GetTransaction, GetTransactionDetail, GetTransactionDetailError, GetTransactionError,
    ListSinceBlock, ListSinceBlockError, ListTransactions, ListUnspent, ListUnspentItem,
    ListUnspentItemError, SendAll, SendAllError, SimulateRawTransaction, TransactionItem,
    TransactionItemError,
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
        let decoded = self
            .decoded
            .map(|tx| encode::deserialize_hex::<Transaction>(&tx).map_err(E::Tx))
            .transpose()?;

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
            self.address.map(|a| a.parse::<Address<_>>().map_err(E::Address)).transpose()?;
        let category = self.category.into_model();
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let vout = crate::to_u32(self.vout, "vout")?;
        let fee = self
            .fee
            .map(|f| SignedAmount::from_btc(f).map_err(E::Fee))
            .transpose()? // optional historically
            .unwrap_or_else(|| SignedAmount::from_sat(0));
        let block_hash =
            self.block_hash.map(|h| h.parse::<BlockHash>().map_err(E::BlockHash)).transpose()?;
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
            mempool_conflicts: None,
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
        let label = self.label.unwrap_or_default();

        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let confirmations = crate::to_u32(self.confirmations, "confirmations")?;
        let redeem_script = self
            .redeem_script
            .map(|hex| ScriptBuf::from_hex(&hex).map_err(E::RedeemScript))
            .transpose()?;

        Ok(model::ListUnspentItem {
            txid,
            vout,
            address,
            label,
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

impl SendAll {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendAll, SendAllError> {
        use SendAllError as E;

        let txid = self.txid.as_ref().map(|s| s.parse()).transpose().map_err(E::Txid)?;

        let hex =
            self.hex.as_ref().map(|h| encode::deserialize_hex(h)).transpose().map_err(E::Hex)?;

        let psbt = self.psbt.as_ref().map(|p| p.parse()).transpose().map_err(E::Psbt)?;

        Ok(model::SendAll { complete: self.complete, txid, hex, psbt })
    }
}

impl SimulateRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SimulateRawTransaction, ParseAmountError> {
        let balance_change = SignedAmount::from_btc(self.balance_change)?;
        Ok(model::SimulateRawTransaction { balance_change })
    }
}
