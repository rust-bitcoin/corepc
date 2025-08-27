// SPDX-License-Identifier: CC0-1.0

use bitcoin::amount::ParseAmountError;
use bitcoin::hashes::hash160;
use bitcoin::hex::FromHex;
use bitcoin::key::PublicKey;
use bitcoin::{
    bip32, Address, Amount, ScriptBuf, SignedAmount, Txid, WitnessProgram, WitnessVersion,
};

use super::{
    GetAddressInfo, GetAddressInfoEmbedded, GetAddressInfoEmbeddedError, GetAddressInfoError,
    GetReceivedByLabel, GetWalletInfo, GetWalletInfoError, ListReceivedByAddress,
    ListReceivedByAddressError, ListReceivedByAddressItem, ListReceivedByLabel,
    ListReceivedByLabelError, ListReceivedByLabelItem, ListUnspent, ListUnspentItem,
    ListUnspentItemError,
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
        let labels = self.labels.into_iter().map(|label| label.name).collect();
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
            parent_descriptor: None, // v22 and above only.
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
            label: Some(self.label),
            timestamp: self.timestamp,
            hd_key_path,
            hd_seed_id,
            hd_master_fingerprint,
            labels,
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
        let labels = self.labels.map(|labels| labels.into_iter().map(|label| label.name).collect());

        Ok(model::GetAddressInfoEmbedded {
            address,
            script_pubkey,
            solvable: self.solvable,
            descriptor: self.descriptor,
            parent_descriptor: None, // v22 and above only.
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
            label: self.label,
            labels,
        })
    }
}

impl GetReceivedByLabel {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetReceivedByLabel, ParseAmountError> {
        let amount = bitcoin::Amount::from_btc(self.0)?;
        Ok(model::GetReceivedByLabel(amount))
    }
}

impl GetWalletInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetWalletInfo, GetWalletInfoError> {
        use GetWalletInfoError as E;

        let wallet_version = crate::to_u32(self.wallet_version, "wallet_version")?;
        let balance = Amount::from_btc(self.balance).map_err(E::Balance)?;
        let unconfirmed_balance =
            Amount::from_btc(self.unconfirmed_balance).map_err(E::UnconfirmedBalance)?;
        let immature_balance =
            Amount::from_btc(self.immature_balance).map_err(E::ImmatureBalance)?;
        let tx_count = crate::to_u32(self.tx_count, "tx_count")?;
        let keypool_oldest = crate::to_u32(self.keypool_oldest, "keypoo_oldest")?;
        let keypool_size = crate::to_u32(self.keypool_size, "keypoo_size")?;
        let keypool_size_hd_internal =
            crate::to_u32(self.keypool_size_hd_internal, "keypoo_size_hd_internal")?;
        let pay_tx_fee = crate::btc_per_kb(self.pay_tx_fee).map_err(E::PayTxFee)?;
        let hd_seed_id =
            self.hd_seed_id.map(|s| s.parse::<hash160::Hash>()).transpose().map_err(E::HdSeedId)?;

        Ok(model::GetWalletInfo {
            wallet_name: self.wallet_name,
            wallet_version,
            format: None,
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
            avoid_reuse: None,
            scanning: None,
            descriptors: None,
            external_signer: None,
            blank: None,
            birthtime: None,
            last_processed_block: None,
        })
    }
}

impl ListReceivedByAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListReceivedByAddress, ListReceivedByAddressError> {
        let balances = self
            .0
            .into_iter()
            .map(|balance| balance.into_model())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(model::ListReceivedByAddress(balances))
    }
}

impl ListReceivedByAddressItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::ListReceivedByAddressItem, ListReceivedByAddressError> {
        use ListReceivedByAddressError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let txids = self
            .txids
            .iter()
            .enumerate()
            .map(|(i, txid)| {
                txid.parse::<Txid>().map_err(|e| ListReceivedByAddressError::Txids(i, e))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::ListReceivedByAddressItem {
            involves_watch_only: self.involves_watch_only,
            address,
            amount,
            confirmations: self.confirmations,
            label: self.label,
            txids,
        })
    }
}

impl ListReceivedByLabel {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListReceivedByLabel, ListReceivedByLabelError> {
        self.0
            .into_iter()
            .map(|item| item.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map(model::ListReceivedByLabel)
    }
}

impl ListReceivedByLabelItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListReceivedByLabelItem, ListReceivedByLabelError> {
        use ListReceivedByLabelError as E;

        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let confirmations = crate::to_u32(self.confirmations, "confirmations")?;

        Ok(model::ListReceivedByLabelItem {
            involves_watch_only: self.involves_watch_only,
            amount,
            confirmations,
            label: self.label,
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
            parent_descriptors: None, // v24 and later only.
        })
    }
}
