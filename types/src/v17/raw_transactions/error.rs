// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::psbt::PsbtParseError;
use bitcoin::{address, hex, sighash};

use crate::error::write_err;
use crate::psbt::{
    Bip32DerivError, PartialSignatureError, RawTransactionError, RawTransactionInputError,
    RawTransactionOutputError, WitnessUtxoError,
};

/// Error when converting a `DecodePsbt` type into the model type.
#[derive(Debug)]
pub enum DecodePsbtError {
    /// Conversion of the `tx` field failed.
    Tx(RawTransactionError),
    /// Conversion of one the map items in the `unknown` field failed.
    Unknown(hex::HexToBytesError),
    /// Conversion of one of the PSBT inputs failed.
    Inputs(PsbtInputError),
    /// Conversion of one of the PSBT outputs failed.
    Outputs(PsbtOutputError),
}

impl fmt::Display for DecodePsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodePsbtError as E;

        match *self {
            E::Tx(ref e) => write_err!(f, "conversion of raw transaction data field failed"; e),
            E::Unknown(ref e) => {
                write_err!(f, "conversion of one the map items in the `unknown` field failed"; e)
            }
            E::Inputs(ref e) => write_err!(f, "conversion of one of the PSBT inputs failed"; e),
            E::Outputs(ref e) => write_err!(f, "conversion of one of the PSBT outputs failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodePsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DecodePsbtError as E;

        match *self {
            E::Tx(ref e) => Some(e),
            E::Unknown(ref e) => Some(e),
            E::Inputs(ref e) => Some(e),
            E::Outputs(ref e) => Some(e),
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
    /// Conversion of the `unknown` field failed.
    Unknown(hex::HexToBytesError),
}

impl fmt::Display for PsbtInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PsbtInputError as E;

        match *self {
            E::NonWitnessUtxo(ref e) => {
                write_err!(f, "conversion of the `non_witness_utxo` field failed"; e)
            }
            E::WitnessUtxo(ref e) => {
                write_err!(f, "conversion of the `witness_utxo` field failed"; e)
            }
            E::PartialSignatures(ref e) => {
                write_err!(f, "conversion of the `partial_signatures` field failed"; e)
            }
            E::Sighash(ref e) => write_err!(f, "conversion of the `sighash` field failed"; e),
            E::RedeemScript(ref e) => {
                write_err!(f, "conversion of the `redeem_script` field failed"; e)
            }
            E::WitnessScript(ref e) => {
                write_err!(f, "conversion of the `witness_script` field failed"; e)
            }
            E::Bip32Derivs(ref e) => {
                write_err!(f, "conversion of the `bip32_derivs` field failed"; e)
            }
            E::FinalScriptSig(ref e) => {
                write_err!(f, "conversion of the `final_script_sig` field failed"; e)
            }
            E::FinalScriptWitness(ref e) => {
                write_err!(f, "conversion of the `final_script_witness` field failed"; e)
            }
            E::Unknown(ref e) => write_err!(f, "conversion of the `unknown` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PsbtInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use PsbtInputError as E;

        match *self {
            E::NonWitnessUtxo(ref e) => Some(e),
            E::WitnessUtxo(ref e) => Some(e),
            E::PartialSignatures(ref e) => Some(e),
            E::Sighash(ref e) => Some(e),
            E::RedeemScript(ref e) => Some(e),
            E::WitnessScript(ref e) => Some(e),
            E::Bip32Derivs(ref e) => Some(e),
            E::FinalScriptSig(ref e) => Some(e),
            E::FinalScriptWitness(ref e) => Some(e),
            E::Unknown(ref e) => Some(e),
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
    /// Conversion of the `unknown` field failed.
    Unknown(hex::HexToBytesError),
}

impl fmt::Display for PsbtOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PsbtOutputError as E;

        match *self {
            E::RedeemScript(ref e) => {
                write_err!(f, "conversion of the `redeem_script` field failed"; e)
            }
            E::WitnessScript(ref e) => {
                write_err!(f, "conversion of the `witness_script` field failed"; e)
            }
            E::Bip32Derivs(ref e) => {
                write_err!(f, "conversion of the `bip32_derivs` field failed"; e)
            }
            E::Unknown(ref e) => write_err!(f, "conversion of the `unknown` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PsbtOutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use PsbtOutputError as E;

        match *self {
            E::RedeemScript(ref e) => Some(e),
            E::WitnessScript(ref e) => Some(e),
            E::Bip32Derivs(ref e) => Some(e),
            E::Unknown(ref e) => Some(e),
        }
    }
}

/// Error when converting a `DecodeScript` type into the model type.
#[derive(Debug)]
pub enum DecodeScriptError {
    /// Conversion of the transaction `hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the transaction `addresses` field failed.
    Addresses(address::ParseError),
    /// Conversion of the transaction `p2sh` field failed.
    P2sh(address::ParseError),
}

impl fmt::Display for DecodeScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodeScriptError as E;

        match *self {
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Addresses(ref e) => write_err!(f, "conversion of the `addresses` field failed"; e),
            E::P2sh(ref e) => write_err!(f, "conversion of the `p2sh` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeScriptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DecodeScriptError as E;

        match *self {
            E::Hex(ref e) => Some(e),
            E::Addresses(ref e) => Some(e),
            E::P2sh(ref e) => Some(e),
        }
    }
}

/// Error when converting a `FinalizePsbt` type into the model type.
#[derive(Debug)]
pub enum FinalizePsbtError {
    /// Conversion of the transaction `psbt` field failed.
    Psbt(PsbtParseError),
    /// Conversion of the transaction `hex` field failed.
    Hex(encode::FromHexError),
}

impl fmt::Display for FinalizePsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FinalizePsbtError as E;

        match *self {
            E::Psbt(ref e) => write_err!(f, "conversion of the `psbt` field failed"; e),
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FinalizePsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use FinalizePsbtError as E;

        match *self {
            E::Psbt(ref e) => Some(e),
            E::Hex(ref e) => Some(e),
        }
    }
}

/// Error when converting a `FundRawTransaction` type into the model type.
#[derive(Debug)]
pub enum FundRawTransactionError {
    /// Conversion of the transaction `hex` field failed.
    Hex(encode::FromHexError),
    /// Conversion of the transaction `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for FundRawTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FundRawTransactionError as E;

        match *self {
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FundRawTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use FundRawTransactionError as E;

        match *self {
            E::Hex(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}

/// Error when converting a `GetRawTransactionVerbose` type into the model type.
#[derive(Debug)]
pub enum GetRawTransactionVerboseError {
    /// Conversion of one of the transaction inputs failed.
    Inputs(RawTransactionInputError),
    /// Conversion of one of the transaction outputs failed.
    Outputs(RawTransactionOutputError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetRawTransactionVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetRawTransactionVerboseError as E;

        match *self {
            E::Inputs(ref e) => {
                write_err!(f, "conversion of one of the transaction inputs failed"; e)
            }
            E::Outputs(ref e) => {
                write_err!(f, "conversion of one of the transaction outputs failed"; e)
            }
            E::BlockHash(ref e) => write_err!(f, "conversion of the `block_hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetRawTransactionVerboseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetRawTransactionVerboseError as E;

        match *self {
            E::Inputs(ref e) => Some(e),
            E::Outputs(ref e) => Some(e),
            E::BlockHash(ref e) => Some(e),
        }
    }
}

/// Error when converting a `SignRawTransaction` type into the model type.
#[derive(Debug)]
pub enum SignRawTransactionError {
    /// Conversion of the transaction `hex` field failed.
    Hex(encode::FromHexError),
    /// Conversion of the transaction `errors` field failed.
    Errors(SignFailError),
}

impl fmt::Display for SignRawTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SignRawTransactionError as E;

        match *self {
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Errors(ref e) => write_err!(f, "conversion of the `errors` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SignRawTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use SignRawTransactionError as E;

        match *self {
            E::Hex(ref e) => Some(e),
            E::Errors(ref e) => Some(e),
        }
    }
}

/// Error when converting a `SignFailError` type into the model type.
#[derive(Debug)]
pub enum SignFailError {
    /// Conversion of the transaction `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the transaction `script_sig` field failed.
    ScriptSig(hex::HexToBytesError),
}

impl fmt::Display for SignFailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SignFailError as E;

        match *self {
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::ScriptSig(ref e) => write_err!(f, "conversion of the `script_sig` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SignFailError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use SignFailError as E;

        match *self {
            E::Txid(ref e) => Some(e),
            E::ScriptSig(ref e) => Some(e),
        }
    }
}
