// SPDX-License-Identifier: CC0-1.0

use bitcoin::hashes::{hash160, sha256};
use bitcoin::psbt::{Psbt, PsbtParseError};
use bitcoin::Amount;

use super::{
    AnalyzePsbt, AnalyzePsbtError, AnalyzePsbtInput, AnalyzePsbtInputMissing,
    AnalyzePsbtInputMissingError, JoinPsbts, UtxoUpdatePsbt,
};
use crate::model;

impl AnalyzePsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AnalyzePsbt, AnalyzePsbtError> {
        use AnalyzePsbtError as E;

        let inputs = self
            .inputs
            .into_iter()
            .map(|input| input.into_model())
            .collect::<Result<_, _>>()
            .map_err(E::Inputs)?;
        let estimated_fee_rate = self
            .estimated_fee_rate
            .map(crate::btc_per_kb)
            .transpose()
            .map_err(E::EstimatedFeeRate)?;
        let fee = self.fee.map(Amount::from_btc).transpose().map_err(E::Fee)?;

        Ok(model::AnalyzePsbt {
            inputs,
            estimated_vsize: self.estimated_vsize,
            estimated_fee_rate,
            fee,
            next: self.next,
        })
    }
}

impl AnalyzePsbtInput {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AnalyzePsbtInput, AnalyzePsbtInputMissingError> {
        let missing = self.missing.map(|m| m.into_model()).transpose()?;

        Ok(model::AnalyzePsbtInput {
            has_utxo: self.has_utxo,
            is_final: self.is_final,
            missing,
            next: self.next,
        })
    }
}

impl AnalyzePsbtInputMissing {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::AnalyzePsbtInputMissing, AnalyzePsbtInputMissingError> {
        use AnalyzePsbtInputMissingError as E;

        let pubkeys = match self.pubkeys {
            Some(v) => v
                .iter()
                .map(|s| s.parse::<hash160::Hash>())
                .collect::<Result<_, _>>()
                .map_err(E::PubKeys)?,
            None => vec![],
        };
        let signatures = match self.signatures {
            Some(v) => v
                .iter()
                .map(|s| s.parse::<hash160::Hash>())
                .collect::<Result<_, _>>()
                .map_err(E::Signatures)?,
            None => vec![],
        };
        let redeem_script = self
            .redeem_script
            .map(|s| s.parse::<hash160::Hash>())
            .transpose()
            .map_err(E::RedeemScript)?;
        let witness_script = self
            .witness_script
            .map(|s| s.parse::<sha256::Hash>())
            .transpose()
            .map_err(E::WitnessScript)?;

        Ok(model::AnalyzePsbtInputMissing { pubkeys, signatures, redeem_script, witness_script })
    }
}

impl JoinPsbts {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::JoinPsbts, PsbtParseError> {
        let psbt = self.0.parse::<Psbt>()?;
        Ok(model::JoinPsbts(psbt))
    }

    /// Converts json straight to a `bitcoin::Psbt`.
    pub fn psbt(self) -> Result<Psbt, PsbtParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl UtxoUpdatePsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::UtxoUpdatePsbt, PsbtParseError> {
        let psbt = self.0.parse::<Psbt>()?;
        Ok(model::UtxoUpdatePsbt(psbt))
    }

    /// Converts json straight to a `bitcoin::Psbt`.
    pub fn psbt(self) -> Result<Psbt, PsbtParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}
