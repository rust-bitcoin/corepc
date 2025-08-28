// SPDX-License-Identifier: CC0-1.0

use bitcoin::{hex, Txid};

use super::{
    GetWalletInfo, GetWalletInfoError, GetWalletInfoScanning, PsbtBumpFee, PsbtBumpFeeError, Send,
    SendError, SendMany, SendManyVerbose, UnloadWallet,
};
use crate::model;

impl UnloadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::UnloadWallet {
        model::UnloadWallet { warnings: vec![self.warning] }
    }
}

impl PsbtBumpFee {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::PsbtBumpFee, PsbtBumpFeeError> {
        use PsbtBumpFeeError as E;

        let psbt = self.psbt.parse().map_err(E::Psbt)?;
        let original_fee = bitcoin::Amount::from_btc(self.original_fee).map_err(E::OriginalFee)?;
        let fee = bitcoin::Amount::from_btc(self.fee).map_err(E::Fee)?;
        let errors = self.errors;
        Ok(model::PsbtBumpFee { psbt, original_fee, fee, errors })
    }
}

impl Send {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::Send, SendError> {
        use SendError as E;

        let txid = self.txid.as_ref().map(|s| s.parse()).transpose().map_err(E::Txid)?;

        let hex = self
            .hex
            .as_ref()
            .map(|h| bitcoin::consensus::encode::deserialize_hex(h))
            .transpose()
            .map_err(E::Hex)?;

        let psbt = self.psbt.as_ref().map(|p| p.parse()).transpose().map_err(E::Psbt)?;

        Ok(model::Send { complete: self.complete, txid, hex, psbt })
    }
}

impl SendMany {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendMany, hex::HexToArrayError> {
        let txid = self.0.parse::<Txid>()?;
        Ok(model::SendMany(txid))
    }
}

impl SendManyVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendManyVerbose, hex::HexToArrayError> {
        let txid = self.txid.parse::<Txid>()?;
        Ok(model::SendManyVerbose { txid, fee_reason: self.fee_reason })
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
        let keypool_oldest = crate::to_u32(self.keypool_oldest, "keypool_oldest")?;
        let keypool_size = crate::to_u32(self.keypool_size, "keypool_size")?;
        let keypool_size_hd_internal =
            crate::to_u32(self.keypool_size_hd_internal, "keypool_size_hd_internal")?;
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
            balance,
            unconfirmed_balance,
            immature_balance,
            tx_count,
            keypool_oldest,
            keypool_size,
            keypool_size_hd_internal,
            unlocked_until: self.unlocked_until,
            pay_tx_fee,
            hd_seed_id,
            private_keys_enabled: self.private_keys_enabled,
            avoid_reuse: Some(self.avoid_reuse),
            scanning,
            format: Some(self.format),
            descriptors: Some(self.descriptors),
            external_signer: None,
            blank: None,
            birthtime: None,
            last_processed_block: None,
        })
    }
}
