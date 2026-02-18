// SPDX-License-Identifier: CC0-1.0

use bitcoin::consensus::encode;
use bitcoin::{Address, BlockHash, ScriptBuf, SignedAmount, Transaction, Txid};

use super::{
    AddMultisigAddress, AddMultisigAddressError, GetTransaction, GetTransactionError,
    GetWalletInfo, GetWalletInfoError, GetWalletInfoScanning, ListSinceBlock, ListSinceBlockError,
    ListTransactions, TransactionItem, TransactionItemError,
};
use crate::model;

impl AddMultisigAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AddMultisigAddress, AddMultisigAddressError> {
        use AddMultisigAddressError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let redeem_script = ScriptBuf::from_hex(&self.redeem_script).map_err(E::RedeemScript)?;

        Ok(model::AddMultisigAddress {
            address,
            redeem_script,
            descriptor: Some(self.descriptor),
            warnings: self.warnings,
        })
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
            fee, // Option in model
            confirmations: self.confirmations,
            generated: self.generated,
            trusted: self.trusted,
            block_hash,
            block_height,
            block_index,
            block_time: self.block_time,
            txid,
            wtxid: None,
            wallet_conflicts,
            replaced_by_txid,
            replaces_txid,
            mempool_conflicts: None, // v28 and later only.
            to: self.to,
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable: self.bip125_replaceable.into_model(),
            parent_descriptors: None, // v24 and later only.
            details,
            decoded,
            last_processed_block: None, // v26 and later only.
            tx,
        })
    }
}

impl GetWalletInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetWalletInfo, GetWalletInfoError> {
        use GetWalletInfoError as E;

        let wallet_version = crate::to_u32(self.wallet_version, "wallet_version")?;
        let balance = bitcoin::Amount::from_btc(self.balance).map_err(E::Balance)?;
        let unconfirmed_balance =
            bitcoin::Amount::from_btc(self.unconfirmed_balance).map_err(E::UnconfirmedBalance)?;
        let immature_balance =
            bitcoin::Amount::from_btc(self.immature_balance).map_err(E::ImmatureBalance)?;
        let tx_count = crate::to_u32(self.tx_count, "tx_count")?;
        let keypool_oldest =
            self.keypool_oldest.map(|v| crate::to_u32(v, "keypool_oldest")).transpose()?;
        let keypool_size = crate::to_u32(self.keypool_size, "keypool_size")?;
        let keypool_size_hd_internal = self
            .keypool_size_hd_internal
            .map(|v| crate::to_u32(v, "keypool_size_hd_internal"))
            .transpose()?;
        let pay_tx_fee = crate::btc_per_kb(self.pay_tx_fee).map_err(E::PayTxFee)?;
        let hd_seed_id = self.hd_seed_id.map(|s| s.parse()).transpose().map_err(E::HdSeedId)?;

        let scanning = match self.scanning {
            GetWalletInfoScanning::Details { duration, progress } =>
                Some(model::GetWalletInfoScanning::Details { duration, progress }),
            GetWalletInfoScanning::NotScanning(b) =>
                Some(model::GetWalletInfoScanning::NotScanning(b)),
        };

        Ok(model::GetWalletInfo {
            wallet_name: self.wallet_name,
            wallet_version,
            balance: Some(balance),
            unconfirmed_balance: Some(unconfirmed_balance),
            immature_balance: Some(immature_balance),
            tx_count,
            keypool_oldest: Some(keypool_oldest.unwrap_or(0)),
            keypool_size,
            keypool_size_hd_internal: keypool_size_hd_internal.unwrap_or(0),
            unlocked_until: self.unlocked_until,
            pay_tx_fee,
            hd_seed_id,
            private_keys_enabled: self.private_keys_enabled,
            avoid_reuse: Some(self.avoid_reuse),
            scanning,
            format: Some(self.format),
            descriptors: Some(self.descriptors),
            external_signer: Some(self.external_signer),
            blank: None,
            birthtime: None,
            flags: None,
            last_processed_block: None,
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

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
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
            address: Some(address),
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
            wtxid: None,
            wallet_conflicts: Some(wallet_conflicts),
            replaced_by_txid,
            replaces_txid,
            mempool_conflicts: None,
            to: self.to,
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable,
            parent_descriptors: None,
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
