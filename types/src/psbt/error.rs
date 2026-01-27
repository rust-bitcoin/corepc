// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::{amount, bip32, ecdsa, hex, key};

use crate::error::write_err;

/// Error when converting a `RawTransaction` type into the model type.
#[derive(Debug)]
pub enum RawTransactionError {
    /// Conversion of one of the transaction inputs failed.
    Inputs(RawTransactionInputError),
    /// Conversion of one of the transaction outputs failed.
    Outputs(RawTransactionOutputError),
}

impl fmt::Display for RawTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Inputs(ref e) =>
                write_err!(f, "conversion of one of the transaction inputs failed"; e),
            Self::Outputs(ref e) =>
                write_err!(f, "conversion of one of the transaction outputs failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RawTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Inputs(ref e) => Some(e),
            Self::Outputs(ref e) => Some(e),
        }
    }
}

/// Error when converting a `RawTransactionInput` type into a `TxIn`.
#[derive(Debug)]
pub enum RawTransactionInputError {
    /// Conversion of the input `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Input lacked both `txid` and `coinbase` data.
    MissingTxid,
    /// Input lacked the `vout` field for a non-coinbase input.
    MissingVout,
    /// Input lacked both `scriptSig` and `coinbase` data.
    MissingScriptSig,
    /// Conversion of the input `script_sig` field failed.
    ScriptSig(hex::HexToBytesError),
    /// Conversion of one of the `witness` hex strings failed.
    Witness(hex::HexToBytesError),
}

impl fmt::Display for RawTransactionInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of the input `txid` field failed"; e),
            Self::MissingTxid =>
                write!(f, "missing both `txid` and `coinbase` fields for the transaction input"),
            Self::MissingVout =>
                write!(f, "missing `vout` field for non-coinbase transaction input"),
            Self::MissingScriptSig =>
                write!(f, "missing both `scriptSig` and `coinbase` data for the transaction input"),
            Self::ScriptSig(ref e) =>
                write_err!(f, "conversion of the input `script_sig` field failed"; e),
            Self::Witness(ref e) =>
                write_err!(f, "conversion of one of the `witness` hex strings failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RawTransactionInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::MissingTxid => None,
            Self::MissingVout => None,
            Self::MissingScriptSig => None,
            Self::ScriptSig(ref e) => Some(e),
            Self::Witness(ref e) => Some(e),
        }
    }
}

/// Error when converting a `RawTransactionOutput` type into a `TxIn`.
#[derive(Debug)]
pub enum RawTransactionOutputError {
    /// Conversion of the output `value` field failed.
    Value(amount::ParseAmountError),
    /// Conversion of the output `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
}

impl fmt::Display for RawTransactionOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Value(ref e) => write_err!(f, "conversion of the output `value` field failed"; e),
            Self::ScriptPubkey(ref e) =>
                write_err!(f, "conversion of the output `script_pubkey` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RawTransactionOutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Value(ref e) => Some(e),
            Self::ScriptPubkey(ref e) => Some(e),
        }
    }
}

/// Error when converting a `WitnessUtxo` type into a `TxOut`.
#[derive(Debug)]
pub enum WitnessUtxoError {
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
}

impl fmt::Display for WitnessUtxoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::ScriptPubkey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WitnessUtxoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Amount(ref e) => Some(e),
            Self::ScriptPubkey(ref e) => Some(e),
        }
    }
}

/// Error when converting one of the partial sigs key-value pairs.
#[derive(Debug)]
pub enum PartialSignatureError {
    /// Error parsing public key.
    PublicKey(key::ParsePublicKeyError),
    /// Error parsing signature.
    Signature(ecdsa::Error),
}

impl fmt::Display for PartialSignatureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::PublicKey(ref e) =>
                write_err!(f, "partial sigs key-value pair parse pubkey failed"; e),
            Self::Signature(ref e) =>
                write_err!(f, "partial sigs key-value pair parse sig failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PartialSignatureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::PublicKey(ref e) => Some(e),
            Self::Signature(ref e) => Some(e),
        }
    }
}

/// Error when converting BIP-32 derivation information.
#[derive(Debug)]
pub enum Bip32DerivError {
    /// Conversion of the pubkey failed.
    Pubkey(key::ParsePublicKeyError),
    /// Conversion of the `master_fingerprint` field failed.
    MasterFingerprint(hex::HexToArrayError),
    /// Conversion of the `path` field failed.
    Path(bip32::Error),
}

impl fmt::Display for Bip32DerivError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Pubkey(ref e) => write_err!(f, "conversion of the pubkey failed"; e),
            Self::MasterFingerprint(ref e) =>
                write_err!(f, "conversion of the `master_fingerprint` field failed"; e),
            Self::Path(ref e) => write_err!(f, "conversion of the `path` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Bip32DerivError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Pubkey(ref e) => Some(e),
            Self::MasterFingerprint(ref e) => Some(e),
            Self::Path(ref e) => Some(e),
        }
    }
}
