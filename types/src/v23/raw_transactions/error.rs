// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::{address, bip32, hex, sighash};

use super::{Bip32DerivError, PartialSignatureError, RawTransactionError, WitnessUtxoError};
use crate::error::write_err;
use crate::v19::DecodeScriptSegwitError;

/// Error when converting a `DecodePsbt` type into the model type.
#[derive(Debug)]
pub enum DecodePsbtError {
    /// Conversion of the `tx` field failed.
    Tx(RawTransactionError),
    /// Conversion of the `global_xpubs` field failed.
    GlobalXpubs(GlobalXpubError),
    /// Conversion of the `proprietary` field failed.
    Proprietary(hex::HexToBytesError),
    /// Conversion of one the map items in the `unknown` field failed.
    Unknown(hex::HexToBytesError),
    /// Conversion of one of the PSBT inputs failed.
    Inputs(PsbtInputError),
    /// Conversion of one of the PSBT outputs failed.
    Outputs(PsbtOutputError),
}

impl fmt::Display for DecodePsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Tx(ref e) => write_err!(f, "conversion of raw transaction data field failed"; e),
            Self::GlobalXpubs(ref e) =>
                write_err!(f, "conversion of one the map items in the `global_xbubs` field failed"; e),
            Self::Proprietary(ref e) =>
                write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e),
            Self::Unknown(ref e) =>
                write_err!(f, "conversion of one the map items in the `unknown` field failed"; e),
            Self::Inputs(ref e) => write_err!(f, "conversion of one of the PSBT inputs failed"; e),
            Self::Outputs(ref e) =>
                write_err!(f, "conversion of one of the PSBT outputs failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodePsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Tx(ref e) => Some(e),
            Self::GlobalXpubs(ref e) => Some(e),
            Self::Proprietary(ref e) => Some(e),
            Self::Unknown(ref e) => Some(e),
            Self::Inputs(ref e) => Some(e),
            Self::Outputs(ref e) => Some(e),
        }
    }
}

/// Error when converting one of the global xpubs failed.
#[derive(Debug)]
pub enum GlobalXpubError {
    /// Conversion of the `xpub` field failed.
    Xpub(bip32::Error),
    /// Conversion of the `master_fingerprint` field failed.
    MasterFingerprint(hex::HexToArrayError),
    /// Conversion of the `path` field failed.
    Path(bip32::Error),
}

impl fmt::Display for GlobalXpubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Xpub(ref e) => write_err!(f, "conversion of the xpub failed"; e),
            Self::MasterFingerprint(ref e) =>
                write_err!(f, "conversion of the `master_fingerprint` field failed"; e),
            Self::Path(ref e) => write_err!(f, "conversion of the `path` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GlobalXpubError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Xpub(ref e) => Some(e),
            Self::MasterFingerprint(ref e) => Some(e),
            Self::Path(ref e) => Some(e),
        }
    }
}

/// Error when converting one of the `DecodePsbt` inputs failed.
#[derive(Debug)]
pub enum PsbtInputError {
    /// Conversion of the `non_witness_utxo` field failed.
    NonWitnessUtxo(RawTransactionError),
    /// Conversion of the `witness_utxo` field failed.
    WitnessUtxo(WitnessUtxoError),
    /// Conversion of the `partial_signatures` field failed.
    PartialSignatures(PartialSignatureError),
    /// Conversion of the `sighash` field failed.
    Sighash(sighash::SighashTypeParseError),
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
    /// Conversion of the `witness_script` field failed.
    WitnessScript(hex::HexToBytesError),
    /// Conversion of the `bip32_derivs` field failed.
    Bip32Derivs(Bip32DerivError),
    /// Conversion of the `final_script_sig` field failed.
    FinalScriptSig(hex::HexToBytesError),
    /// Conversion of the `final_script_witness` field failed.
    FinalScriptWitness(hex::HexToBytesError),
    /// Conversion of the `ripemd160` hash failed.
    Ripemd160(hex::HexToArrayError),
    /// Conversion of the `ripemd160` preimage failed.
    Ripemd160Preimage(hex::HexToBytesError),
    /// Conversion of the `sha256` hash failed.
    Sha256(hex::HexToArrayError),
    /// Conversion of the `sha256` preimage failed.
    Sha256Preimage(hex::HexToBytesError),
    /// Conversion of the `hash160` hash failed.
    Hash160(hex::HexToArrayError),
    /// Conversion of the `hash160` preimage failed.
    Hash160Preimage(hex::HexToBytesError),
    /// Conversion of the `hash256` hash failed.
    Hash256(hex::HexToArrayError),
    /// Conversion of the `hash256` preimage failed.
    Hash256Preimage(hex::HexToBytesError),
    /// Conversion of the `proprietary` field failed.
    Proprietary(hex::HexToBytesError),
    /// Conversion of the `unknown` field failed.
    Unknown(hex::HexToBytesError),
}

impl fmt::Display for PsbtInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NonWitnessUtxo(ref e) =>
                write_err!(f, "conversion of the `non_witness_utxo` field failed"; e),
            Self::WitnessUtxo(ref e) =>
                write_err!(f, "conversion of the `witness_utxo` field failed"; e),
            Self::PartialSignatures(ref e) =>
                write_err!(f, "conversion of the `partial_signatures` field failed"; e),
            Self::Sighash(ref e) => write_err!(f, "conversion of the `sighash` field failed"; e),
            Self::RedeemScript(ref e) =>
                write_err!(f, "conversion of the `redeem_script` field failed"; e),
            Self::WitnessScript(ref e) =>
                write_err!(f, "conversion of the `witness_script` field failed"; e),
            Self::Bip32Derivs(ref e) =>
                write_err!(f, "conversion of the `bip32_derivs` field failed"; e),
            Self::FinalScriptSig(ref e) =>
                write_err!(f, "conversion of the `final_script_sig` field failed"; e),
            Self::FinalScriptWitness(ref e) =>
                write_err!(f, "conversion of the `final_script_witness` field failed"; e),
            Self::Ripemd160(ref e) => write_err!(f, "conversion of the `ripemd160` hash failed"; e),
            Self::Ripemd160Preimage(ref e) =>
                write_err!(f, "conversion of the `ripemd160` preimage failed"; e),
            Self::Sha256(ref e) => write_err!(f, "conversion of the `sha256` hash failed"; e),
            Self::Sha256Preimage(ref e) =>
                write_err!(f, "conversion of the `sha256` preimage failed"; e),
            Self::Hash160(ref e) => write_err!(f, "conversion of the `hash160` hash failed"; e),
            Self::Hash160Preimage(ref e) =>
                write_err!(f, "conversion of the `hash160` preimage failed"; e),
            Self::Hash256(ref e) => write_err!(f, "conversion of the `hash256` hash failed"; e),
            Self::Hash256Preimage(ref e) =>
                write_err!(f, "conversion of the `hash256` preimage failed"; e),
            Self::Proprietary(ref e) =>
                write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e),
            Self::Unknown(ref e) => write_err!(f, "conversion of the `unknown` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PsbtInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::NonWitnessUtxo(ref e) => Some(e),
            Self::WitnessUtxo(ref e) => Some(e),
            Self::PartialSignatures(ref e) => Some(e),
            Self::Sighash(ref e) => Some(e),
            Self::RedeemScript(ref e) => Some(e),
            Self::WitnessScript(ref e) => Some(e),
            Self::Bip32Derivs(ref e) => Some(e),
            Self::FinalScriptSig(ref e) => Some(e),
            Self::FinalScriptWitness(ref e) => Some(e),
            Self::Ripemd160(ref e) => Some(e),
            Self::Ripemd160Preimage(ref e) => Some(e),
            Self::Sha256(ref e) => Some(e),
            Self::Sha256Preimage(ref e) => Some(e),
            Self::Hash160(ref e) => Some(e),
            Self::Hash160Preimage(ref e) => Some(e),
            Self::Hash256(ref e) => Some(e),
            Self::Hash256Preimage(ref e) => Some(e),
            Self::Proprietary(ref e) => Some(e),
            Self::Unknown(ref e) => Some(e),
        }
    }
}

/// Error when converting one of the `DecodePsbt` outputs failed.
#[derive(Debug)]
pub enum PsbtOutputError {
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
    /// Conversion of the `witness_script` field failed.
    WitnessScript(hex::HexToBytesError),
    /// Conversion of the `bip32_derivs` field failed.
    Bip32Derivs(Bip32DerivError),
    /// Conversion of the `proprietary` field failed.
    Proprietary(hex::HexToBytesError),
    /// Conversion of the `unknown` field failed.
    Unknown(hex::HexToBytesError),
}

impl fmt::Display for PsbtOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::RedeemScript(ref e) =>
                write_err!(f, "conversion of the `redeem_script` field failed"; e),
            Self::WitnessScript(ref e) =>
                write_err!(f, "conversion of the `witness_script` field failed"; e),
            Self::Bip32Derivs(ref e) =>
                write_err!(f, "conversion of the `bip32_derivs` field failed"; e),
            Self::Proprietary(ref e) =>
                write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e),
            Self::Unknown(ref e) => write_err!(f, "conversion of the `unknown` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PsbtOutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::RedeemScript(ref e) => Some(e),
            Self::WitnessScript(ref e) => Some(e),
            Self::Bip32Derivs(ref e) => Some(e),
            Self::Proprietary(ref e) => Some(e),
            Self::Unknown(ref e) => Some(e),
        }
    }
}

/// Error when converting a `DecodeScript` type into the model type.
#[derive(Debug)]
pub enum DecodeScriptError {
    /// Conversion of the transaction `hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the transaction `address` field failed.
    Address(address::ParseError),
    /// Conversion of the transaction `addresses` field failed.
    Addresses(address::ParseError),
    /// Conversion of the transaction `p2sh` field failed.
    P2sh(address::ParseError),
    /// Conversion of the transaction `segwit` field failed.
    Segwit(DecodeScriptSegwitError),
}

impl fmt::Display for DecodeScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodeScriptError as E;
        match *self {
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::Addresses(ref e) => write_err!(f, "conversion of the `addresses` field failed"; e),
            E::P2sh(ref e) => write_err!(f, "conversion of the `p2sh` field failed"; e),
            E::Segwit(ref e) => write_err!(f, "conversion of the `segwit` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeScriptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DecodeScriptError as E;
        match *self {
            E::Hex(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::Addresses(ref e) => Some(e),
            E::P2sh(ref e) => Some(e),
            E::Segwit(ref e) => Some(e),
        }
    }
}
