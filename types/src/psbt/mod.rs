// SPDX-License-Identifier: CC0-1.0

mod error;

use std::collections::{BTreeMap, HashMap};

use bitcoin::hex::{self, FromHex as _};
use bitcoin::{
    absolute, bip32, ecdsa, psbt, secp256k1, transaction, Amount, OutPoint, PublicKey, ScriptBuf,
    Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};
use serde::{Deserialize, Serialize};

pub use self::error::{
    Bip32DerivError, PartialSignatureError, RawTransactionError, RawTransactionInputError,
    RawTransactionOutputError, WitnessUtxoError,
};
use crate::{ScriptPubkey, ScriptSig};

/// Represents a bitcoin transaction.
///
/// Part of `decoderawtransaction` and `decodepsbt`.
// This JSON data can be encapsulated by a `bitcoin::Transaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RawTransaction {
    /// The transaction id.
    pub txid: String,
    /// The transaction hash (differs from txid for witness transactions).
    pub hash: String,
    /// The transaction size in bytes.
    pub size: u64,
    /// The virtual transaction size (differs from size for witness transactions).
    pub vsize: u64,
    /// The transaction's weight (between vsize*4 - 3 and vsize*4).
    pub weight: u64,
    /// The version number.
    pub version: i32,
    /// The lock time.
    #[serde(rename = "locktime")]
    pub lock_time: u32,
    /// Array of transaction inputs.
    #[serde(rename = "vin")]
    pub inputs: Vec<RawTransactionInput>,
    /// Array of transaction outputs.
    #[serde(rename = "vout")]
    pub outputs: Vec<RawTransactionOutput>,
}

impl RawTransaction {
    /// Converts this raw transaction data into a bitcoin transaction.
    pub fn to_transaction(&self) -> Result<Transaction, RawTransactionError> {
        use RawTransactionError as E;

        let version = transaction::Version::non_standard(self.version);
        let lock_time = absolute::LockTime::from_consensus(self.lock_time);
        let input = self
            .inputs
            .iter()
            .map(|input| input.to_input())
            .collect::<Result<_, _>>()
            .map_err(E::Inputs)?;
        let output = self
            .outputs
            .iter()
            .map(|output| output.to_output())
            .collect::<Result<_, _>>()
            .map_err(E::Outputs)?;

        Ok(Transaction { version, lock_time, input, output })
    }
}

/// Represents a transaction input.
// This JSON data can be encapsulated by a `bitcoin::TxIn`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RawTransactionInput {
    /// The transaction id.
    pub txid: String,
    /// The output number.
    pub vout: u32,
    /// The script.
    #[serde(rename = "scriptSig")]
    pub script_sig: ScriptSig,
    /// Hex-encoded witness data (if any).
    #[serde(rename = "txinwitness")]
    pub txin_witness: Option<Vec<String>>,
    /// The script sequence number.
    pub sequence: u32,
}

impl RawTransactionInput {
    /// Converts this raw transaction input data into a bitcoin [`TxIn`].
    pub fn to_input(&self) -> Result<TxIn, RawTransactionInputError> {
        use RawTransactionInputError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let script_sig = self.script_sig.script_buf().map_err(E::ScriptSig)?;

        let witness = match &self.txin_witness {
            Some(v) => crate::witness_from_hex_slice(v).map_err(E::Witness)?,
            None => Witness::new(),
        };

        Ok(TxIn {
            previous_output: OutPoint { txid, vout: self.vout },
            script_sig,
            sequence: Sequence::from_consensus(self.sequence),
            witness,
        })
    }
}

/// Represents a transaction output.
// This JSON data can be encapsulated by a `bitcoin::TxOut` + index.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RawTransactionOutput {
    /// The value in BTC.
    pub value: f64,
    /// Index number.
    #[serde(rename = "n")]
    pub index: u64,
    /// The script pubkey.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: ScriptPubkey,
}

impl RawTransactionOutput {
    /// Converts this raw UTXO data into a bitcoin `TxOut`.
    pub fn to_output(&self) -> Result<TxOut, RawTransactionOutputError> {
        use RawTransactionOutputError as E;

        let value = Amount::from_btc(self.value).map_err(E::Value)?;
        let script_pubkey = self.script_pubkey.script_buf().map_err(E::ScriptPubkey)?;

        Ok(TxOut { value, script_pubkey })
    }
}

/// Transaction output for witness UTXOs.
// This JSON data can be encapsulated by a `bitcoin::TxOut`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WitnessUtxo {
    /// The value in BTC.
    pub amount: f64,
    /// The scriptPubKey.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: ScriptPubkey,
}

impl WitnessUtxo {
    /// Converts this raw UTXO data into a bitcoin `TxOut`.
    pub fn to_tx_out(&self) -> Result<TxOut, WitnessUtxoError> {
        use WitnessUtxoError as E;

        let value = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let script_pubkey = self.script_pubkey.script_buf().map_err(E::ScriptPubkey)?;

        Ok(TxOut { value, script_pubkey })
    }
}

/// A script part of a PSBT input or output.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtScript {
    /// The asm.
    pub asm: String,
    /// The hex.
    pub hex: String,
    /// The type, eg 'pubkeyhash'.
    #[serde(rename = "type")]
    pub type_: String,
}

impl PsbtScript {
    /// Parses the hex into a `ScriptBuf`.
    pub fn script_buf(&self) -> Result<ScriptBuf, hex::HexToBytesError> {
        ScriptBuf::from_hex(&self.hex)
    }
}

/// BIP-32 derivation information.
// WARNING: The bip32_derivs in PSBT input list seems to change shape
// a bunch of times over the versions. By v23 it looks like this.
//
// This JSON data can be encapsulated as a map item in bitcoin::psbt::{Input, Output}
// bip32_derivation: BTreeMap<secp256k1::PublicKey, KeySource>,
// KeySource = (Fingerprint, DerivationPath);
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Bip32Deriv {
    /// The public key this path corresponds to.
    pub pubkey: String,
    /// The fingerprint of the master key.
    pub master_fingerprint: String,
    /// The path.
    pub path: String,
}

/// The key source data for a BIP-32 derivation.
// In v0.17 the BIP-32 derivation for inputs is a map of pubkey to this type.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct InputKeySource {
    /// The fingerprint of the master key.
    pub master_fingerprint: String,
    /// The path.
    pub path: String,
}

/// Final script data.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FinalScript {
    /// The asm.
    pub asm: String,
    /// The hex.
    pub hex: String,
}

/// Converts a map of unknown key-value pairs.
pub fn into_unknown(
    hash_map: HashMap<String, String>,
) -> Result<BTreeMap<psbt::raw::Key, Vec<u8>>, hex::HexToBytesError> {
    let mut map = BTreeMap::default();
    for (k, v) in hash_map.iter() {
        // FIXME: This is best guess, I (Tobin) don't actually know what
        // is in the hex string returned by Core.
        let key = Vec::from_hex(k)?;
        let value = Vec::from_hex(v)?;

        // rust-bitcoin separates out the key type.
        let mut p = key.as_slice();
        let _ = crate::compact_size_decode(&mut p); // Moves p past the keylen integer.
        let type_value = crate::compact_size_decode(&mut p);

        // In the next release of rust-bitcoin this is changed to a u64.
        // Yes this looses data - c'est la vie.
        let type_value: u8 = type_value as u8;

        let key = psbt::raw::Key { type_value, key };
        map.insert(key, value);
    }
    Ok(map)
}

/// Converts a map of partial signature key-value pairs.
pub fn into_partial_signatures(
    hash_map: HashMap<String, String>,
) -> Result<BTreeMap<PublicKey, ecdsa::Signature>, PartialSignatureError> {
    use PartialSignatureError as E;

    let mut map = BTreeMap::default();
    for (k, v) in hash_map.iter() {
        let pubkey = k.parse::<PublicKey>().map_err(E::PublicKey)?;
        let signature = v.parse::<ecdsa::Signature>().map_err(E::Signature)?;
        map.insert(pubkey, signature);
    }
    Ok(map)
}

/// Converts a hash map of BIP-32 derivation data into a map suitable for use with `psbt::Psbt`.
pub fn map_into_bip32_derivation(
    hash_map: HashMap<String, InputKeySource>,
) -> Result<BTreeMap<secp256k1::PublicKey, bip32::KeySource>, Bip32DerivError> {
    use bip32::{DerivationPath, Fingerprint};
    use Bip32DerivError as E;

    let mut map = BTreeMap::default();
    for (k, v) in hash_map.iter() {
        let pubkey = k.parse::<PublicKey>().map_err(E::Pubkey)?;
        let fingerprint =
            Fingerprint::from_hex(&v.master_fingerprint).map_err(E::MasterFingerprint)?;
        let path = v.path.parse::<DerivationPath>().map_err(E::Path)?;

        map.insert(pubkey.inner, (fingerprint, path));
    }
    Ok(map)
}

/// Converts a vector map of BIP-32 derivation data into a map suitable for use with `psbt::Psbt`.
pub fn vec_into_bip32_derivation(
    v: Vec<Bip32Deriv>,
) -> Result<BTreeMap<secp256k1::PublicKey, bip32::KeySource>, Bip32DerivError> {
    use bip32::{DerivationPath, Fingerprint};
    use Bip32DerivError as E;

    let mut map = BTreeMap::default();
    for deriv in v.iter() {
        let pubkey = deriv.pubkey.parse::<PublicKey>().map_err(E::Pubkey)?;
        let fingerprint =
            Fingerprint::from_hex(&deriv.master_fingerprint).map_err(E::MasterFingerprint)?;
        let path = deriv.path.parse::<DerivationPath>().map_err(E::Path)?;

        map.insert(pubkey.inner, (fingerprint, path));
    }
    Ok(map)
}
