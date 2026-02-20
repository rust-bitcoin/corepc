// SPDX-License-Identifier: CC0-1.0

use bitcoin::consensus::encode;
use bitcoin::{BlockHash, Psbt, SignedAmount, Transaction, Txid};

use super::{
    CreateWallet, GetBalances, GetBalancesError, GetTransaction, GetTransactionError,
    GetWalletInfo, GetWalletInfoError, GetWalletInfoScanning, LastProcessedBlock,
    LastProcessedBlockError, LoadWallet, UnloadWallet, WalletProcessPsbt, WalletProcessPsbtError,
};
use crate::model;

impl CreateWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::CreateWallet {
        model::CreateWallet { name: self.name, warnings: self.warnings.unwrap_or_default() }
    }

    /// Returns the created wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

impl GetBalances {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalances, GetBalancesError> {
        use GetBalancesError as E;

        let mine = self.mine.into_model().map_err(E::Mine)?;
        let watch_only = self
            .watch_only
            .map(|watch_only| watch_only.into_model())
            .transpose()
            .map_err(E::WatchOnly)?;
        let last_processed_block = self
            .last_processed_block
            .map(|l| l.into_model())
            .transpose()
            .map_err(E::LastProcessedBlock)?;

        Ok(model::GetBalances { mine, watch_only, last_processed_block })
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
        let last_processed_block = self
            .last_processed_block
            .map(|l| l.into_model())
            .transpose()
            .map_err(E::LastProcessedBlock)?;

        let scanning = match self.scanning {
            GetWalletInfoScanning::Details { duration, progress } =>
                Some(model::GetWalletInfoScanning::Details { duration, progress }),
            GetWalletInfoScanning::NotScanning(b) =>
                Some(model::GetWalletInfoScanning::NotScanning(b)),
        };

        Ok(model::GetWalletInfo {
            wallet_name: self.wallet_name,
            wallet_version,
            format: Some(self.format),
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
            descriptors: Some(self.descriptors),
            external_signer: Some(self.external_signer),
            blank: Some(self.blank),
            birthtime: self.birthtime,
            flags: None,
            last_processed_block,
        })
    }
}

impl LastProcessedBlock {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::LastProcessedBlock, LastProcessedBlockError> {
        let hash = self.hash.parse::<BlockHash>().map_err(LastProcessedBlockError::Hash)?;
        let height = crate::to_u32(self.height, "height")?;
        Ok(model::LastProcessedBlock { height, hash })
    }
}

impl LoadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::LoadWallet {
        model::LoadWallet { name: self.name, warnings: self.warnings.unwrap_or_default() }
    }

    /// Returns the loaded wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

impl UnloadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::UnloadWallet {
        model::UnloadWallet { warnings: self.warnings.unwrap_or_default() }
    }
}

impl WalletProcessPsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::WalletProcessPsbt, WalletProcessPsbtError> {
        use WalletProcessPsbtError as E;

        let psbt = self.psbt.parse::<Psbt>().map_err(E::Psbt)?;
        let hex =
            self.hex.as_ref().map(|h| encode::deserialize_hex(h)).transpose().map_err(E::Hex)?;
        Ok(model::WalletProcessPsbt { psbt, complete: self.complete, hex })
    }
}
