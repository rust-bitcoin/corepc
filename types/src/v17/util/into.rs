// SPDX-License-Identifier: CC0-1.0

use bitcoin::hex::FromHex;
use bitcoin::{amount, sign_message, Address, ScriptBuf, WitnessProgram, WitnessVersion};

use super::{
    CreateMultisig, CreateMultisigError, EstimateSmartFee, SignMessageWithPrivKey, ValidateAddress,
    ValidateAddressError,
};
use crate::model;

impl CreateMultisig {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::CreateMultisig, CreateMultisigError> {
        use CreateMultisigError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let redeem_script = ScriptBuf::from_hex(&self.redeem_script).map_err(E::RedeemScript)?;

        Ok(model::CreateMultisig { address, redeem_script, descriptor: None, warnings: None })
    }
}

impl EstimateSmartFee {
    pub fn into_model(self) -> Result<model::EstimateSmartFee, amount::ParseAmountError> {
        let fee_rate = match self.fee_rate {
            Some(f) => crate::btc_per_kb(f)?,
            None => None,
        };
        Ok(model::EstimateSmartFee { fee_rate, errors: self.errors, blocks: self.blocks })
    }
}

impl SignMessageWithPrivKey {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::SignMessageWithPrivKey, sign_message::MessageSignatureError> {
        // The RPC returns a base64-encoded Bitcoin message signature.
        let sig = self.0.parse::<sign_message::MessageSignature>()?;
        Ok(model::SignMessageWithPrivKey(sig))
    }
}

impl ValidateAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ValidateAddress, ValidateAddressError> {
        use ValidateAddressError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubKey)?;
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

        Ok(model::ValidateAddress {
            is_valid: self.is_valid,
            address,
            script_pubkey,
            is_script: self.is_script,
            is_witness: self.is_witness,
            witness_version,
            witness_program,
        })
    }
}
