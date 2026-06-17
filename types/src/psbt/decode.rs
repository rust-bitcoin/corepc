// SPDX-License-Identifier: CC0-1.0

//! Assembles a whole `bitcoin::Psbt` from a decoded `decodepsbt` response.
//!
//! `decodepsbt` returns the PSBT already decomposed into its parts (the unsigned transaction, per
//! input/output maps, global xpubs, ...) rather than the base64 wire form, so a `bitcoin::Psbt`
//! has to be rebuilt from those parts. [`RawPsbt`] is the shared, version-nonspecific shape of that
//! decode; [`RawPsbt::into_psbt`] performs the assembly, reusing the sibling `crate::psbt` helpers
//! (`RawTransaction`, `WitnessUtxo`, `PsbtScript`, `Bip32Deriv`, ...). It is the PSBT counterpart of
//! [`RawTransaction::to_transaction`], so a generated `into_model` can bridge its decoded value
//! through JSON into `RawPsbt` and call this, exactly as `decoderawtransaction` does for a `Transaction`.
//!
//! MuSig2 fields are intentionally ignored: `bitcoin::Psbt` does not model them (and neither does
//! the hand-written conversion). Unknown keys deserialize away by default (no `deny_unknown_fields`).

use std::collections::{BTreeMap, HashMap};

use bitcoin::bip32::{DerivationPath, Fingerprint, KeySource};
use bitcoin::hashes::{hash160, ripemd160, sha256, sha256d};
use bitcoin::hex::{self, FromHex as _};
use bitcoin::psbt::{self, raw, PsbtSighashType};
use bitcoin::taproot::{
    self, ControlBlock, LeafVersion, TapLeafHash, TapNodeHash, TapTree, TaprootBuilder,
};
use bitcoin::{secp256k1, ScriptBuf, TapSighashType, XOnlyPublicKey};
use serde::{Deserialize, Serialize};

use super::{
    Bip32Deriv, FinalScript, PsbtScript, RawTransaction, RawTransactionError, WitnessUtxo,
    WitnessUtxoError,
};

/// The decoded shape of a `decodepsbt` response, ready to be assembled into a `bitcoin::Psbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RawPsbt {
    /// The decoded network-serialized unsigned transaction.
    pub tx: RawTransaction,
    /// The global xpubs.
    #[serde(default)]
    pub global_xpubs: Vec<GlobalXpub>,
    /// The PSBT version number.
    pub psbt_version: u32,
    /// The global proprietary map.
    pub proprietary: Option<Vec<Proprietary>>,
    /// The unknown global fields.
    pub unknown: Option<HashMap<String, String>>,
    /// Array of transaction inputs.
    pub inputs: Vec<RawPsbtInput>,
    /// Array of transaction outputs.
    pub outputs: Vec<RawPsbtOutput>,
}

/// A `decodepsbt` input map.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RawPsbtInput {
    pub non_witness_utxo: Option<RawTransaction>,
    pub witness_utxo: Option<WitnessUtxo>,
    pub partial_signatures: Option<HashMap<String, String>>,
    pub sighash: Option<String>,
    pub redeem_script: Option<PsbtScript>,
    pub witness_script: Option<PsbtScript>,
    pub bip32_derivs: Option<Vec<Bip32Deriv>>,
    #[serde(rename = "final_scriptsig")]
    pub final_script_sig: Option<FinalScript>,
    #[serde(rename = "final_scriptwitness")]
    pub final_script_witness: Option<Vec<String>>,
    pub ripemd160_preimages: Option<HashMap<String, String>>,
    pub sha256_preimages: Option<HashMap<String, String>>,
    pub hash160_preimages: Option<HashMap<String, String>>,
    pub hash256_preimages: Option<HashMap<String, String>>,
    pub taproot_key_path_sig: Option<String>,
    pub taproot_script_path_sigs: Option<Vec<TaprootScriptPathSig>>,
    pub taproot_scripts: Option<Vec<TaprootScript>>,
    pub taproot_bip32_derivs: Option<Vec<TaprootBip32Deriv>>,
    pub taproot_internal_key: Option<String>,
    pub taproot_merkle_root: Option<String>,
    pub proprietary: Option<Vec<Proprietary>>,
    pub unknown: Option<HashMap<String, String>>,
}

/// A `decodepsbt` output map.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RawPsbtOutput {
    pub redeem_script: Option<PsbtScript>,
    pub witness_script: Option<PsbtScript>,
    pub bip32_derivs: Option<Vec<Bip32Deriv>>,
    pub taproot_internal_key: Option<String>,
    pub taproot_tree: Option<Vec<TaprootLeaf>>,
    pub taproot_bip32_derivs: Option<Vec<TaprootBip32Deriv>>,
    pub proprietary: Option<Vec<Proprietary>>,
    pub unknown: Option<HashMap<String, String>>,
}

/// A global xpub list element.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GlobalXpub {
    pub xpub: String,
    pub master_fingerprint: String,
    pub path: String,
}

/// A proprietary map list element.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Proprietary {
    pub identifier: String,
    pub subtype: i64,
    pub key: String,
    pub value: String,
}

/// A Taproot script path signature list element.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TaprootScriptPathSig {
    pub pubkey: String,
    pub leaf_hash: String,
    pub sig: String,
}

/// A Taproot leaf script with its control blocks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TaprootScript {
    pub script: String,
    #[serde(rename = "leaf_ver")]
    pub leaf_version: u32,
    pub control_blocks: Vec<String>,
}

/// A Taproot BIP-32 derivation list element.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TaprootBip32Deriv {
    pub pubkey: String,
    pub master_fingerprint: String,
    pub path: String,
    pub leaf_hashes: Vec<String>,
}

/// A leaf of the Taproot tree (output map).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TaprootLeaf {
    pub depth: u32,
    #[serde(rename = "leaf_ver")]
    pub leaf_version: u32,
    pub script: String,
}

impl RawPsbt {
    /// Assembles the `bitcoin::Psbt` from the decoded parts.
    pub fn into_psbt(self) -> Result<psbt::Psbt, RawPsbtError> {
        use RawPsbtError as E;

        let unsigned_tx = self.tx.to_transaction().map_err(E::Tx)?;

        let mut xpub = BTreeMap::default();
        for g in self.global_xpubs {
            let key = g.xpub.parse().map_err(E::Xpub)?;
            let fp = Fingerprint::from_hex(&g.master_fingerprint).map_err(E::Fingerprint)?;
            let path = g.path.parse::<DerivationPath>().map_err(E::Path)?;
            xpub.insert(key, (fp, path));
        }

        let proprietary = proprietary_map(self.proprietary)?;
        let unknown = match self.unknown {
            Some(map) => super::into_unknown(map).map_err(E::Unknown)?,
            None => BTreeMap::default(),
        };

        let inputs =
            self.inputs.into_iter().map(RawPsbtInput::into_input).collect::<Result<_, _>>()?;
        let outputs =
            self.outputs.into_iter().map(RawPsbtOutput::into_output).collect::<Result<_, _>>()?;

        Ok(psbt::Psbt {
            unsigned_tx,
            version: self.psbt_version,
            xpub,
            proprietary,
            unknown,
            inputs,
            outputs,
        })
    }
}

impl RawPsbtInput {
    fn into_input(self) -> Result<psbt::Input, RawPsbtError> {
        use RawPsbtError as E;

        let non_witness_utxo =
            self.non_witness_utxo.map(|raw| raw.to_transaction()).transpose().map_err(E::Tx)?;
        let witness_utxo =
            self.witness_utxo.map(|u| u.to_tx_out()).transpose().map_err(E::WitnessUtxo)?;
        let partial_sigs = match self.partial_signatures {
            Some(map) => super::into_partial_signatures(map).map_err(E::PartialSignatures)?,
            None => BTreeMap::default(),
        };
        let sighash_type =
            self.sighash.map(|s| s.parse::<PsbtSighashType>()).transpose().map_err(E::Sighash)?;
        let redeem_script =
            self.redeem_script.map(|s| s.script_buf()).transpose().map_err(E::Script)?;
        let witness_script =
            self.witness_script.map(|s| s.script_buf()).transpose().map_err(E::Script)?;
        let bip32_derivation = match self.bip32_derivs {
            Some(d) => super::vec_into_bip32_derivation(d).map_err(E::Bip32Derivs)?,
            None => BTreeMap::default(),
        };
        let final_script_sig = self
            .final_script_sig
            .map(|s| ScriptBuf::from_hex(&s.hex))
            .transpose()
            .map_err(E::Script)?;
        let final_script_witness = self
            .final_script_witness
            .map(|v| crate::witness_from_hex_slice(&v))
            .transpose()
            .map_err(E::Script)?;

        let ripemd160_preimages =
            preimages(self.ripemd160_preimages, |h| h.parse::<ripemd160::Hash>().map_err(E::Hash))?;
        let sha256_preimages =
            preimages(self.sha256_preimages, |h| h.parse::<sha256::Hash>().map_err(E::Hash))?;
        let hash160_preimages =
            preimages(self.hash160_preimages, |h| h.parse::<hash160::Hash>().map_err(E::Hash))?;
        let hash256_preimages =
            preimages(self.hash256_preimages, |h| h.parse::<sha256d::Hash>().map_err(E::Hash))?;

        let tap_key_sig = self.taproot_key_path_sig.map(|s| signature_from_str(&s)).transpose()?;
        let mut tap_script_sigs = BTreeMap::default();
        for s in self.taproot_script_path_sigs.unwrap_or_default() {
            let pubkey = s.pubkey.parse::<XOnlyPublicKey>().map_err(E::XOnly)?;
            let hash = s.leaf_hash.parse::<TapLeafHash>().map_err(E::Hash)?;
            tap_script_sigs.insert((pubkey, hash), signature_from_str(&s.sig)?);
        }
        let mut tap_scripts = BTreeMap::default();
        for s in self.taproot_scripts.unwrap_or_default() {
            let script = ScriptBuf::from_hex(&s.script).map_err(E::Script)?;
            let version =
                LeafVersion::from_consensus(s.leaf_version as u8).map_err(E::LeafVersion)?;
            tap_scripts.insert(control_block(&s.control_blocks)?, (script, version));
        }
        let tap_key_origins = tap_key_origins(self.taproot_bip32_derivs)?;
        let tap_internal_key = self
            .taproot_internal_key
            .map(|k| k.parse::<XOnlyPublicKey>())
            .transpose()
            .map_err(E::XOnly)?;
        let tap_merkle_root = self
            .taproot_merkle_root
            .map(|r| r.parse::<TapNodeHash>())
            .transpose()
            .map_err(E::Hash)?;

        let proprietary = proprietary_map(self.proprietary)?;
        let unknown = match self.unknown {
            Some(map) => super::into_unknown(map).map_err(E::Unknown)?,
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

impl RawPsbtOutput {
    fn into_output(self) -> Result<psbt::Output, RawPsbtError> {
        use RawPsbtError as E;

        let redeem_script =
            self.redeem_script.map(|s| s.script_buf()).transpose().map_err(E::Script)?;
        let witness_script =
            self.witness_script.map(|s| s.script_buf()).transpose().map_err(E::Script)?;
        let bip32_derivation = match self.bip32_derivs {
            Some(d) => super::vec_into_bip32_derivation(d).map_err(E::Bip32Derivs)?,
            None => BTreeMap::default(),
        };
        let tap_internal_key = self
            .taproot_internal_key
            .map(|k| k.parse::<XOnlyPublicKey>())
            .transpose()
            .map_err(E::XOnly)?;
        let tap_tree = self.taproot_tree.map(build_taproot_tree).transpose()?;
        let tap_key_origins = tap_key_origins(self.taproot_bip32_derivs)?;

        let proprietary = proprietary_map(self.proprietary)?;
        let unknown = match self.unknown {
            Some(map) => super::into_unknown(map).map_err(E::Unknown)?,
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

/// Parse a preimage hash map keyed by `parse_hash` (the value is hex-encoded preimage bytes).
fn preimages<H: Ord>(
    map: Option<HashMap<String, String>>,
    parse_hash: impl Fn(&str) -> Result<H, RawPsbtError>,
) -> Result<BTreeMap<H, Vec<u8>>, RawPsbtError> {
    let mut out = BTreeMap::default();
    for (hash, preimage) in map.unwrap_or_default() {
        let hash = parse_hash(&hash)?;
        let bytes = Vec::from_hex(&preimage).map_err(RawPsbtError::Script)?;
        out.insert(hash, bytes);
    }
    Ok(out)
}

/// Build the `tap_key_origins` map shared by PSBT inputs and outputs.
#[allow(clippy::type_complexity)]
fn tap_key_origins(
    derivs: Option<Vec<TaprootBip32Deriv>>,
) -> Result<BTreeMap<XOnlyPublicKey, (Vec<TapLeafHash>, KeySource)>, RawPsbtError> {
    use RawPsbtError as E;
    let mut map = BTreeMap::default();
    for d in derivs.unwrap_or_default() {
        let pubkey = d.pubkey.parse::<XOnlyPublicKey>().map_err(E::XOnly)?;
        let fp = Fingerprint::from_hex(&d.master_fingerprint).map_err(E::Fingerprint)?;
        let path = d.path.parse::<DerivationPath>().map_err(E::Path)?;
        let hashes = d
            .leaf_hashes
            .iter()
            .map(|h| h.parse::<TapLeafHash>())
            .collect::<Result<_, _>>()
            .map_err(E::Hash)?;
        map.insert(pubkey, (hashes, (fp, path)));
    }
    Ok(map)
}

/// Convert the proprietary list into the `bitcoin::Psbt` proprietary map.
fn proprietary_map(
    props: Option<Vec<Proprietary>>,
) -> Result<BTreeMap<raw::ProprietaryKey, Vec<u8>>, RawPsbtError> {
    use RawPsbtError as E;
    let mut map = BTreeMap::default();
    for p in props.unwrap_or_default() {
        let prefix = Vec::from_hex(&p.identifier).map_err(E::Script)?;
        let key = Vec::from_hex(&p.key).map_err(E::Script)?;
        let value = Vec::from_hex(&p.value).map_err(E::Script)?;
        // FIXME: `subtype` widens to `u64` in a future rust-bitcoin; truncate for now (as upstream does).
        map.insert(raw::ProprietaryKey { prefix, subtype: p.subtype as u8, key }, value);
    }
    Ok(map)
}

/// Decode the single control block Core returns per Taproot script (a one-element array).
fn control_block(control_blocks: &[String]) -> Result<ControlBlock, RawPsbtError> {
    match control_blocks {
        [one] => {
            let bytes = Vec::from_hex(one).map_err(RawPsbtError::Script)?;
            ControlBlock::decode(&bytes).map_err(RawPsbtError::Taproot)
        }
        _ => Err(RawPsbtError::ControlBlocks(control_blocks.len())),
    }
}

/// Build a `TapTree` from the depth-first list of leaves.
fn build_taproot_tree(leaves: Vec<TaprootLeaf>) -> Result<TapTree, RawPsbtError> {
    use RawPsbtError as E;
    let mut builder = TaprootBuilder::with_capacity(leaves.len());
    for leaf in leaves {
        let version =
            LeafVersion::from_consensus(leaf.leaf_version as u8).map_err(E::LeafVersion)?;
        let script = ScriptBuf::from_hex(&leaf.script).map_err(E::Script)?;
        builder = builder
            .add_leaf_with_ver(leaf.depth as u8, script, version)
            .map_err(E::TaprootBuilder)?;
    }
    builder.try_into_taptree().map_err(|_| E::IncompleteTaprootTree)
}

/// Decode a hex Taproot signature (Schnorr signature followed by a sighash byte).
fn signature_from_str(sig: &str) -> Result<taproot::Signature, RawPsbtError> {
    use RawPsbtError as E;
    let bytes = Vec::from_hex(sig).map_err(E::Script)?;
    let (sighash_byte, signature) = bytes.split_last().ok_or(E::EmptyTaprootSignature)?;
    Ok(taproot::Signature {
        signature: secp256k1::schnorr::Signature::from_slice(signature).map_err(E::Secp256k1)?,
        sighash_type: TapSighashType::from_consensus_u8(*sighash_byte)
            .map_err(E::TapSighashType)?,
    })
}

/// Error assembling a `bitcoin::Psbt` from a decoded `decodepsbt` response.
#[derive(Debug)]
pub enum RawPsbtError {
    /// Conversion of an (unsigned or non-witness-utxo) transaction failed.
    Tx(RawTransactionError),
    /// Conversion of a witness UTXO failed.
    WitnessUtxo(WitnessUtxoError),
    /// Parsing of partial signatures failed.
    PartialSignatures(super::PartialSignatureError),
    /// Parsing of a BIP-32 derivation failed.
    Bip32Derivs(super::Bip32DerivError),
    /// Parsing the `unknown` key-value map failed.
    Unknown(hex::HexToBytesError),
    /// Parsing an xpub failed.
    Xpub(bitcoin::bip32::Error),
    /// Parsing a derivation path failed.
    Path(bitcoin::bip32::Error),
    /// Parsing a master fingerprint failed.
    Fingerprint(hex::HexToArrayError),
    /// Parsing the sighash type failed.
    Sighash(bitcoin::sighash::SighashTypeParseError),
    /// Decoding a hex-encoded script or byte string failed.
    Script(hex::HexToBytesError),
    /// Parsing a hash failed.
    Hash(hex::HexToArrayError),
    /// Parsing an x-only public key failed.
    XOnly(secp256k1::Error),
    /// A secp256k1 error while decoding a Schnorr signature.
    Secp256k1(secp256k1::Error),
    /// A non-standard Taproot sighash type byte.
    TapSighashType(bitcoin::sighash::InvalidSighashTypeError),
    /// A Taproot signature was empty.
    EmptyTaprootSignature,
    /// An invalid leaf version byte.
    LeafVersion(bitcoin::taproot::TaprootError),
    /// A Taproot script did not carry exactly one control block.
    ControlBlocks(usize),
    /// Decoding a Taproot control block failed.
    Taproot(bitcoin::taproot::TaprootError),
    /// Adding a leaf to the Taproot tree builder failed.
    TaprootBuilder(bitcoin::taproot::TaprootBuilderError),
    /// The Taproot tree builder was left incomplete.
    IncompleteTaprootTree,
}

impl core::fmt::Display for RawPsbtError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use RawPsbtError as E;
        match self {
            E::Tx(e) => write!(f, "transaction: {e}"),
            E::WitnessUtxo(e) => write!(f, "witness utxo: {e}"),
            E::PartialSignatures(e) => write!(f, "partial signatures: {e}"),
            E::Bip32Derivs(e) => write!(f, "bip32 derivation: {e}"),
            E::Unknown(e) => write!(f, "unknown map: {e}"),
            E::Xpub(e) => write!(f, "xpub: {e}"),
            E::Path(e) => write!(f, "derivation path: {e}"),
            E::Fingerprint(e) => write!(f, "master fingerprint: {e}"),
            E::Sighash(e) => write!(f, "sighash type: {e}"),
            E::Script(e) => write!(f, "hex script/bytes: {e}"),
            E::Hash(e) => write!(f, "hash: {e}"),
            E::XOnly(e) => write!(f, "x-only public key: {e}"),
            E::Secp256k1(e) => write!(f, "schnorr signature: {e}"),
            E::TapSighashType(e) => write!(f, "taproot sighash type: {e}"),
            E::EmptyTaprootSignature => write!(f, "empty taproot signature"),
            E::LeafVersion(e) => write!(f, "taproot leaf version: {e}"),
            E::ControlBlocks(n) => write!(f, "expected exactly one control block, got {n}"),
            E::Taproot(e) => write!(f, "taproot control block: {e}"),
            E::TaprootBuilder(e) => write!(f, "taproot tree builder: {e}"),
            E::IncompleteTaprootTree => write!(f, "incomplete taproot tree"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RawPsbtError {}
