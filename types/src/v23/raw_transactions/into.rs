// SPDX-License-Identifier: CC0-1.0

use std::collections::BTreeMap;

use bitcoin::bip32::{DerivationPath, Fingerprint, KeySource, Xpub};
use bitcoin::hashes::{hash160, ripemd160, sha256, sha256d};
use bitcoin::hex::{self, FromHex as _};
use bitcoin::psbt::{self, raw, PsbtSighashType};
use bitcoin::{Address, Amount};

use super::{
    DecodePsbt, DecodePsbtError, DecodeScript, DecodeScriptError, DecodeScriptSegwit,
    DecodeScriptSegwitError, GlobalXpub, GlobalXpubError, Proprietary, PsbtInput, PsbtInputError,
    PsbtOutput, PsbtOutputError,
};
use crate::model;

impl DecodePsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodePsbt, DecodePsbtError> {
        use DecodePsbtError as E;

        let unsigned_tx = self.tx.to_transaction().map_err(E::Tx)?;
        let version = self.psbt_version;

        let mut xpubs = BTreeMap::default();
        for g in self.global_xpubs {
            let (xpub, key_source) = g.to_key_value_pair().map_err(E::GlobalXpubs)?;
            xpubs.insert(xpub, key_source);
        }

        let proprietary = match self.proprietary {
            Some(props) => {
                let mut map = BTreeMap::default();
                for prop in props {
                    let (key, vec) = prop.to_key_value_pair().map_err(E::Proprietary)?;
                    map.insert(key, vec);
                }
                map
            }
            None => BTreeMap::default(),
        };

        let unknown = match self.unknown {
            Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
            None => BTreeMap::default(),
        };

        let inputs = self
            .inputs
            .into_iter()
            .map(|input| input.into_input())
            .collect::<Result<_, _>>()
            .map_err(E::Inputs)?;
        let outputs = self
            .outputs
            .into_iter()
            .map(|output| output.into_output())
            .collect::<Result<_, _>>()
            .map_err(E::Outputs)?;

        let psbt = bitcoin::Psbt {
            unsigned_tx,
            version,
            xpub: xpubs,
            proprietary,
            unknown,
            inputs,
            outputs,
        };
        let fee = self.fee.map(Amount::from_sat);

        Ok(model::DecodePsbt { psbt, fee })
    }
}

impl GlobalXpub {
    /// Converts this global xpub list element to a map entry suitable to use in `bitcoin::Psbt`.
    pub fn to_key_value_pair(&self) -> Result<(Xpub, KeySource), GlobalXpubError> {
        use GlobalXpubError as E;

        let xpub = self.xpub.parse::<Xpub>().map_err(E::Xpub)?;
        let fp = Fingerprint::from_hex(&self.master_fingerprint).map_err(E::MasterFingerprint)?;
        let path = self.path.parse::<DerivationPath>().map_err(E::Path)?;
        Ok((xpub, (fp, path)))
    }
}

impl Proprietary {
    /// Converts this proprietary list element to a map entry suitable to use in `bitcoin::Psbt`.
    pub fn to_key_value_pair(
        &self,
    ) -> Result<(raw::ProprietaryKey, Vec<u8>), hex::HexToBytesError> {
        // FIXME: Remove cast once rust-bitcoin 0.33 is out.
        //
        // This is changed to a u64 in the upcoming rust-bitcoin
        // release, until then just ignore any additional bits.
        let subtype = self.subtype as u8;

        let prefix = Vec::from_hex(&self.identifier)?;
        let key = Vec::from_hex(&self.key)?;
        let value = Vec::from_hex(&self.value)?;

        Ok((raw::ProprietaryKey { prefix, subtype, key }, value))
    }
}

impl PsbtInput {
    /// Converts this PSBT data into a PSBT input.
    pub fn into_input(self) -> Result<psbt::Input, PsbtInputError> {
        use PsbtInputError as E;

        let non_witness_utxo = self
            .non_witness_utxo
            .map(|raw| raw.to_transaction())
            .transpose()
            .map_err(E::NonWitnessUtxo)?;
        let witness_utxo =
            self.witness_utxo.map(|utxo| utxo.to_tx_out()).transpose().map_err(E::WitnessUtxo)?;
        let partial_sigs = match self.partial_signatures {
            Some(map) => crate::psbt::into_partial_signatures(map).map_err(E::PartialSignatures)?,
            None => BTreeMap::default(),
        };
        let sighash_type = self
            .sighash
            .map(|partial| partial.parse::<PsbtSighashType>())
            .transpose()
            .map_err(E::Sighash)?;
        let redeem_script = self
            .redeem_script
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::RedeemScript)?;
        let witness_script = self
            .witness_script
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::WitnessScript)?;
        let bip32_derivation = match self.bip32_derivs {
            Some(derivs) =>
                crate::psbt::vec_into_bip32_derivation(derivs).map_err(E::Bip32Derivs)?,
            None => BTreeMap::default(),
        };
        let final_script_sig = self
            .final_script_sig
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::FinalScriptSig)?;
        let final_script_witness = self
            .final_script_witness
            .map(|v| crate::witness_from_hex_slice(&v))
            .transpose()
            .map_err(E::FinalScriptWitness)?;

        let ripemd160_preimages = match self.ripemd160_preimages {
            Some(map) => {
                let mut preimages = BTreeMap::default();
                for (hash, preimage) in map.iter() {
                    let hash = hash.parse::<ripemd160::Hash>().map_err(E::Ripemd160)?;
                    let preimage = Vec::from_hex(preimage).map_err(E::Ripemd160Preimage)?;
                    preimages.insert(hash, preimage);
                }
                preimages
            }
            None => BTreeMap::default(),
        };
        let sha256_preimages = match self.sha256_preimages {
            Some(map) => {
                let mut preimages = BTreeMap::default();
                for (hash, preimage) in map.iter() {
                    let hash = hash.parse::<sha256::Hash>().map_err(E::Sha256)?;
                    let preimage = Vec::from_hex(preimage).map_err(E::Sha256Preimage)?;
                    preimages.insert(hash, preimage);
                }
                preimages
            }
            None => BTreeMap::default(),
        };
        let hash160_preimages = match self.hash160_preimages {
            Some(map) => {
                let mut preimages = BTreeMap::default();
                for (hash, preimage) in map.iter() {
                    let hash = hash.parse::<hash160::Hash>().map_err(E::Hash160)?;
                    let preimage = Vec::from_hex(preimage).map_err(E::Hash160Preimage)?;
                    preimages.insert(hash, preimage);
                }
                preimages
            }
            None => BTreeMap::default(),
        };
        let hash256_preimages = match self.hash256_preimages {
            Some(map) => {
                let mut preimages = BTreeMap::default();
                for (hash, preimage) in map.iter() {
                    let hash = hash.parse::<sha256d::Hash>().map_err(E::Hash256)?;
                    let preimage = Vec::from_hex(preimage).map_err(E::Hash256Preimage)?;
                    preimages.insert(hash, preimage);
                }
                preimages
            }
            None => BTreeMap::default(),
        };

        let proprietary = match self.proprietary {
            Some(props) => {
                let mut map = BTreeMap::default();
                for prop in props {
                    let (key, vec) = prop.to_key_value_pair().map_err(E::Proprietary)?;
                    map.insert(key, vec);
                }
                map
            }
            None => BTreeMap::default(),
        };

        let unknown = match self.unknown {
            Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
            None => BTreeMap::default(),
        };

        // These fields do not appear until Core v24.
        let tap_key_sig = None;
        let tap_script_sigs = BTreeMap::default();
        let tap_scripts = BTreeMap::default();
        let tap_key_origins = BTreeMap::default();
        let tap_internal_key = None;
        let tap_merkle_root = None;

        Ok(psbt::Input {
            non_witness_utxo,
            witness_utxo,
            partial_sigs,
            sighash_type,
            redeem_script,
            witness_script,
            bip32_derivation,
            final_script_sig,
            final_script_witness,
            ripemd160_preimages,
            sha256_preimages,
            hash160_preimages,
            hash256_preimages,
            tap_key_sig,
            tap_script_sigs,
            tap_scripts,
            tap_key_origins,
            tap_internal_key,
            tap_merkle_root,
            proprietary,
            unknown,
        })
    }
}

impl PsbtOutput {
    /// Converts this PSBT data into a PSBT output.
    pub fn into_output(self) -> Result<psbt::Output, PsbtOutputError> {
        use PsbtOutputError as E;

        let redeem_script = self
            .redeem_script
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::RedeemScript)?;
        let witness_script = self
            .witness_script
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::WitnessScript)?;
        let bip32_derivation = match self.bip32_derivs {
            Some(derivs) =>
                crate::psbt::vec_into_bip32_derivation(derivs).map_err(E::Bip32Derivs)?,
            None => BTreeMap::default(),
        };

        let proprietary = match self.proprietary {
            Some(props) => {
                let mut map = BTreeMap::default();
                for prop in props {
                    let (key, vec) = prop.to_key_value_pair().map_err(E::Proprietary)?;
                    map.insert(key, vec);
                }
                map
            }
            None => BTreeMap::default(),
        };

        let unknown = match self.unknown {
            Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
            None => BTreeMap::default(),
        };

        // These fields do not appear until Core v24.
        let tap_internal_key = None;
        let tap_tree = None;
        let tap_key_origins = BTreeMap::default();

        Ok(psbt::Output {
            redeem_script,
            witness_script,
            bip32_derivation,
            tap_internal_key,
            tap_tree,
            tap_key_origins,
            proprietary,
            unknown,
        })
    }
}

impl DecodeScript {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeScript, DecodeScriptError> {
        use DecodeScriptError as E;

        let address = match self.address {
            Some(addr) => Some(addr.parse::<Address<_>>().map_err(E::Address)?),
            None => None,
        };
        let addresses = match self.addresses {
            Some(addresses) => addresses
                .iter()
                .map(|s| s.parse::<Address<_>>())
                .collect::<Result<_, _>>()
                .map_err(E::Addresses)?,
            None => vec![],
        };
        let p2sh = self.p2sh.map(|s| s.parse::<Address<_>>()).transpose().map_err(E::P2sh)?;

        Ok(model::DecodeScript {
            script_pubkey: None,
            type_: self.type_,
            descriptor: self.descriptor,
            address,
            required_signatures: self.required_signatures,
            addresses,
            p2sh,
            segwit: self.segwit.map(|s| s.into_model()).transpose().map_err(E::Segwit)?,
        })
    }
}
impl DecodeScriptSegwit {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeScriptSegwit, DecodeScriptSegwitError> {
        use DecodeScriptSegwitError as E;

        let address = match self.address {
            Some(addr) => Some(addr.parse::<Address<_>>().map_err(E::Address)?),
            None => None,
        };
        let addresses = match self.addresses {
            Some(addrs) => addrs
                .into_iter()
                .map(|s| s.parse::<Address<_>>())
                .collect::<Result<_, _>>()
                .map_err(E::Addresses)?,
            None => vec![],
        };
        let required_signatures = self.required_signatures;
        let p2sh_segwit = self.p2sh_segwit;

        Ok(model::DecodeScriptSegwit {
            asm: self.asm,
            hex: self.hex,
            descriptor: self.descriptor,
            type_: self.type_,
            address,
            required_signatures,
            addresses,
            p2sh_segwit,
        })
    }
}
