// SPDX-License-Identifier: CC0-1.0

use bitcoin::hashes::hash160;
use bitcoin::key::PublicKey;
use bitcoin::{
    address, bip32, Address, RedeemScriptBuf, ScriptPubKeyBuf, WitnessProgram, WitnessVersion,
};

use super::{
    GetAddressInfo, GetAddressInfoEmbedded, GetAddressInfoEmbeddedError, GetAddressInfoError,
    WalletDisplayAddress,
};
use crate::model;

impl GetAddressInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfo, GetAddressInfoError> {
        use GetAddressInfoError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptPubKeyBuf::from_hex_no_length_prefix(&self.script_pubkey)
            .map_err(E::ScriptPubKey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program)
        {
            (Some(v), Some(hex)) => {
                if v > u8::MAX as i64 || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version =
                    WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = bitcoin::hex::decode_to_vec(&hex).map_err(E::WitnessProgramBytes)?;
                let witness_program =
                    WitnessProgram::new(witness_version, &bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            }
            _ => (None, None), // TODO: Think more if catchall is ok.
        };
        let script = self.script.map(|s| s.into_model());
        let redeem_script = self
            .hex
            .map(|hex| RedeemScriptBuf::from_hex_no_length_prefix(&hex).map_err(E::Hex))
            .transpose()?;
        let pubkeys = self
            .pubkeys
            .map(|pubkeys| {
                pubkeys
                    .iter()
                    .map(|s| s.parse::<PublicKey>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(E::PubKeys)
            })
            .transpose()?;
        let sigs_required =
            self.sigs_required.map(|s| crate::to_u32(s, "sigs_required")).transpose()?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).transpose().map_err(E::PubKey)?;
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
            is_script: Some(self.is_script),
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
        let script_pubkey = ScriptPubKeyBuf::from_hex_no_length_prefix(&self.script_pubkey)
            .map_err(E::ScriptPubKey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program)
        {
            (Some(v), Some(hex)) => {
                if v > u8::MAX as i64 || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version =
                    WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = bitcoin::hex::decode_to_vec(&hex).map_err(E::WitnessProgramBytes)?;
                let witness_program =
                    WitnessProgram::new(witness_version, &bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            }
            _ => (None, None), // TODO: Think more if catchall is ok.
        };
        let script = self.script.map(|s| s.into_model());
        let redeem_script = self
            .hex
            .map(|hex| RedeemScriptBuf::from_hex_no_length_prefix(&hex).map_err(E::Hex))
            .transpose()?;
        let pubkeys = None;
        let sigs_required =
            self.sigs_required.map(|s| crate::to_u32(s, "sigs_required")).transpose()?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).transpose().map_err(E::PubKey)?;

        Ok(model::GetAddressInfoEmbedded {
            address,
            script_pubkey,
            solvable: self.solvable,
            descriptor: self.descriptor,
            parent_descriptor: self.parent_descriptor,
            is_script: Some(self.is_script),
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

impl WalletDisplayAddress {
    pub fn into_model(self) -> Result<model::WalletDisplayAddress, address::ParseError> {
        let address = self.address.parse::<Address<_>>()?;
        Ok(model::WalletDisplayAddress { address })
    }
}
