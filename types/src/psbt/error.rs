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
        use RawTransactionError as E;

        match *self {
            E::Inputs(ref e) => {
                write_err!(f, "conversion of one of the transaction inputs failed"; e)
            }
            E::Outputs(ref e) => {
                write_err!(f, "conversion of one of the transaction outputs failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RawTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use RawTransactionError as E;

        match *self {
            E::Inputs(ref e) => Some(e),
            E::Outputs(ref e) => Some(e),
        }
    }
}

/// Error when converting a `RawTransactionInput` type into a `TxIn`.
#[derive(Debug)]
pub enum RawTransactionInputError {
    /// Conversion of the input `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the input `script_sig` field failed.
    ScriptSig(hex::HexToBytesError),
    /// Conversion of one of the `witness` hex strings failed.
    Witness(hex::HexToBytesError),
}

impl fmt::Display for RawTransactionInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RawTransactionInputError as E;

        match *self {
            E::Txid(ref e) => write_err!(f, "conversion of the input `txid` field failed"; e),
            E::ScriptSig(ref e) => {
                write_err!(f, "conversion of the input `script_sig` field failed"; e)
            }
            E::Witness(ref e) => {
                write_err!(f, "conversion of one of the `witness` hex strings failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RawTransactionInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use RawTransactionInputError as E;

        match *self {
            E::Txid(ref e) => Some(e),
            E::ScriptSig(ref e) => Some(e),
            E::Witness(ref e) => Some(e),
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
        use RawTransactionOutputError as E;

        match *self {
            E::Value(ref e) => write_err!(f, "conversion of the output `value` field failed"; e),
            E::ScriptPubkey(ref e) => {
                write_err!(f, "conversion of the output `script_pubkey` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RawTransactionOutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use RawTransactionOutputError as E;

        match *self {
            E::Value(ref e) => Some(e),
            E::ScriptPubkey(ref e) => Some(e),
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
        use WitnessUtxoError as E;

        match *self {
            E::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            E::ScriptPubkey(ref e) => {
                write_err!(f, "conversion of the `script_pubkey` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WitnessUtxoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use WitnessUtxoError as E;

        match *self {
            E::Amount(ref e) => Some(e),
            E::ScriptPubkey(ref e) => Some(e),
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
        use PartialSignatureError as E;

        match *self {
            E::PublicKey(ref e) => {
                write_err!(f, "partial sigs key-value pair parse pubkey failed"; e)
            }
            E::Signature(ref e) => write_err!(f, "partial sigs key-value pair parse sig failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PartialSignatureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use PartialSignatureError as E;

        match *self {
            E::PublicKey(ref e) => Some(e),
            E::Signature(ref e) => Some(e),
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
        use Bip32DerivError as E;

        match *self {
            E::Pubkey(ref e) => write_err!(f, "conversion of the pubkey failed"; e),
            E::MasterFingerprint(ref e) => {
                write_err!(f, "conversion of the `master_fingerprint` field failed"; e)
            }
            E::Path(ref e) => write_err!(f, "conversion of the `path` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Bip32DerivError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Bip32DerivError as E;

        match *self {
            E::Pubkey(ref e) => Some(e),
            E::MasterFingerprint(ref e) => Some(e),
            E::Path(ref e) => Some(e),
        }
    }
}
