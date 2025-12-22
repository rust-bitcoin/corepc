// SPDX-License-Identifier: CC0-1.0

use std::collections::BTreeMap;

use bitcoin::bip32::{DerivationPath, Fingerprint, KeySource, Xpub};
use bitcoin::hashes::{hash160, ripemd160, sha256, sha256d};
use bitcoin::hex::{self, FromHex as _};
use bitcoin::psbt::{self, raw, PsbtSighashType};
use bitcoin::taproot::{
    ControlBlock, LeafVersion, TapLeafHash, TapNodeHash, TapTree, TaprootBuilder,
};
use bitcoin::{Amount, ScriptBuf, XOnlyPublicKey};

use super::{
    taproot, ControlBlocksError, DecodePsbt, DecodePsbtError, GlobalXpub, GlobalXpubError,
    Proprietary, PsbtInput, PsbtInputError, PsbtOutput, PsbtOutputError, TaprootBip32Deriv,
    TaprootBip32DerivsError, TaprootLeaf, TaprootLeafError, TaprootScript, TaprootScriptError,
    TaprootScriptPathSig, TaprootScriptPathSigError,
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
        let fee = self.fee.map(Amount::from_btc).transpose().map_err(E::Fee)?;

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

        let tap_key_sig = self
            .taproot_key_path_sig
            .map(|s| taproot::signature_from_str(&s))
            .transpose()
            .map_err(E::TaprootKeyPathSig)?;
        let tap_script_sigs = match self.taproot_script_path_sigs {
            Some(vec) => {
                let mut map = BTreeMap::default();
                for elem in vec.iter() {
                    let ((pubkey, hash), sig) =
                        elem.to_key_value_pair().map_err(E::TaprootScriptPathSigs)?;
                    map.insert((pubkey, hash), sig);
                }
                map
            }
            None => BTreeMap::default(),
        };
        let tap_scripts = match self.taproot_scripts {
            Some(vec) => {
                let mut map = BTreeMap::default();
                for elem in vec.iter() {
                    let (control_block, (script, key_source)) =
                        elem.to_key_value_pair().map_err(E::TaprootScripts)?;
                    map.insert(control_block, (script, key_source));
                }
                map
            }
            None => BTreeMap::default(),
        };
        let tap_key_origins = match self.taproot_bip32_derivs {
            Some(vec) => {
                let mut map = BTreeMap::default();
                for elem in vec.iter() {
                    let (pubkey, (leaves, key_source)) =
                        elem.to_key_value_pair().map_err(E::TaprootBip32Derivs)?;
                    map.insert(pubkey, (leaves, key_source));
                }
                map
            }
            None => BTreeMap::default(),
        };
        let tap_internal_key = self
            .taproot_internal_key
            .map(|key| key.parse::<XOnlyPublicKey>())
            .transpose()
            .map_err(E::TaprootInternalKey)?;
        let tap_merkle_root = self
            .taproot_merkle_root
            .map(|root| root.parse::<TapNodeHash>())
            .transpose()
            .map_err(E::TaprootMerkleRoot)?;

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

        let tap_internal_key = self
            .taproot_internal_key
            .map(|key| key.parse::<XOnlyPublicKey>())
            .transpose()
            .map_err(E::TaprootInternalKey)?;

        let tap_tree =
            self.taproot_tree.map(build_taproot_tree).transpose().map_err(E::TaprootTree)?;
        let tap_key_origins = match self.taproot_bip32_derivs {
            Some(vec) => {
                let mut map = BTreeMap::default();
                for elem in vec.iter() {
                    let (pubkey, (leaves, key_source)) =
                        elem.to_key_value_pair().map_err(E::TaprootBip32Derivs)?;
                    map.insert(pubkey, (leaves, key_source));
                }
                map
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

impl TaprootScriptPathSig {
    /// Converts list element to a map entry suitable to use in `bitcoin::psbt::Input`.
    pub fn to_key_value_pair(
        &self,
    ) -> Result<((XOnlyPublicKey, TapLeafHash), taproot::Signature), TaprootScriptPathSigError>
    {
        use TaprootScriptPathSigError as E;

        let pubkey = self.pubkey.parse::<XOnlyPublicKey>().map_err(E::Pubkey)?;
        let hash = self.leaf_hash.parse::<TapLeafHash>().map_err(E::LeafHash)?;
        let sig = super::taproot::signature_from_str(&self.sig).map_err(E::Sig)?;

        Ok(((pubkey, hash), sig))
    }
}

impl TaprootScript {
    /// Converts list element to a map entry suitable to use in `bitcoin::psbt::Input`.
    pub fn to_key_value_pair(
        &self,
    ) -> Result<(ControlBlock, (ScriptBuf, LeafVersion)), TaprootScriptError> {
        use TaprootScriptError as E;

        let script = ScriptBuf::from_hex(&self.script).map_err(E::Script)?;

        let leaf_version = self.leaf_version as u8; // FIXME: Is this cast ok?
        let version = LeafVersion::from_consensus(leaf_version).map_err(E::LeafVer)?;

        let control_block = control_block(&self.control_blocks).map_err(E::ControlBlocks)?;

        Ok((control_block, (script, version)))
    }
}

// FIXME: I (Tobin) cannot work out why Core returns a vector of control blocks. From my
// reading of rust-bitcoin code and also BIP-341 there is exactly one control block per script?
fn control_block(control_blocks: &[String]) -> Result<ControlBlock, ControlBlocksError> {
    use ControlBlocksError as E;

    match control_blocks.len() {
        // FIXME: How can this be empty, there would be nothing to key the `tap_scripts` map by?
        0 => Err(E::Missing),
        1 => {
            let bytes = Vec::from_hex(&control_blocks[0]).map_err(E::Parse)?;
            Ok(ControlBlock::decode(&bytes).map_err(E::Decode)?)
        }
        n => Err(E::Multiple(n)),
    }
}

impl TaprootBip32Deriv {
    /// Converts list element to a map entry suitable to use in `bitcoin::psbt::Input`.
    pub fn to_key_value_pair(
        &self,
    ) -> Result<(XOnlyPublicKey, (Vec<TapLeafHash>, KeySource)), TaprootBip32DerivsError> {
        use TaprootBip32DerivsError as E;

        let pubkey = self.pubkey.parse::<XOnlyPublicKey>().map_err(E::Pubkey)?;
        let fp = Fingerprint::from_hex(&self.master_fingerprint).map_err(E::MasterFingerprint)?;
        let path = self.path.parse::<DerivationPath>().map_err(E::Path)?;
        let hashes = self
            .leaf_hashes
            .iter()
            .map(|leaf| leaf.parse::<TapLeafHash>())
            .collect::<Result<_, _>>()
            .map_err(E::LeafHashes)?;

        Ok((pubkey, (hashes, (fp, path))))
    }
}

fn build_taproot_tree(leaves: Vec<TaprootLeaf>) -> Result<TapTree, TaprootLeafError> {
    use TaprootLeafError as E;

    let mut builder = TaprootBuilder::with_capacity(leaves.len());

    for leaf in leaves.iter() {
        // Cast ok because depth can never exceed 128.
        let depth = leaf.depth as u8;

        let leaf_version = leaf.leaf_version as u8; // FIXME: Is this cast ok?
        let version = LeafVersion::from_consensus(leaf_version).map_err(E::LeafVer)?;

        let script = ScriptBuf::from_hex(&leaf.script).map_err(E::Script)?;

        builder = builder.add_leaf_with_ver(depth, script, version).map_err(E::TaprootBuilder)?;
    }
    let tree = builder.try_into_taptree().map_err(E::IncompleteBuilder)?;
    Ok(tree)
}
