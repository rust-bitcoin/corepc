// SPDX-License-Identifier: CC0-1.0

use bitcoin::{hex, Txid};

use super::{
    PsbtBumpFee, PsbtBumpFeeError, Send, SendError, SendMany, SendManyVerbose, UnloadWallet,
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
