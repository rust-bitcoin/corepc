// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::{self, ParseAmountError};
use bitcoin::consensus::encode;
use bitcoin::{address, hex};

use crate::error::write_err;

/// Error when converting a `RawTransaction` type into the model type.
#[derive(Debug)]
pub enum RawTransactionError {
    /// Conversion of on of the transaction inputs failed.
    Inputs(RawTransactionInputError),
    /// Conversion of on of the transaction outputs failed.
    Outputs(RawTransactionOutputError),
}

impl fmt::Display for RawTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RawTransactionError as E;

        match *self {
            E::Inputs(ref e) =>
                write_err!(f, "conversion of on of the transaction inputs failed"; e),
            E::Outputs(ref e) =>
                write_err!(f, "conversion of on of the transaction outputs failed"; e),
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
            E::ScriptSig(ref e) =>
                write_err!(f, "conversion of the input `script_sig` field failed"; e),
            E::Witness(ref e) =>
                write_err!(f, "conversion of one of the `witness` hex strings failed"; e),
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
            E::ScriptPubkey(ref e) =>
                write_err!(f, "conversion of the output `script_pubkey` field failed"; e),
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
    /// Conversion of on of the transaction inputs failed.
    Inputs(RawTransactionInputError),
    /// Conversion of on of the transaction outputs failed.
    Outputs(RawTransactionOutputError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetRawTransactionVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetRawTransactionVerboseError as E;

        match *self {
            E::Inputs(ref e) =>
                write_err!(f, "conversion of on of the transaction inputs failed"; e),
            E::Outputs(ref e) =>
                write_err!(f, "conversion of on of the transaction outputs failed"; e),
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
