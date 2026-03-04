// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::hex;
use bitcoin::psbt::PsbtParseError;

use super::GetTransactionDetailError;
use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetTransaction` type into the model type.
#[derive(Debug)]
pub enum GetTransactionError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `txid` field failed.
    Txid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `wallet_conflicts` field failed.
    WalletConflicts(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `replaced_by_txid` field failed.
    ReplacedByTxid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `replaces_txid` field failed.
    ReplacesTxid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `mempool_conflicts` field failed.
    MempoolConflicts(hex::DecodeFixedLengthBytesError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the `details` field failed.
    Details(GetTransactionDetailError),
}

impl fmt::Display for GetTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            Self::BlockHash(ref e) =>
                write_err!(f, "conversion of the `block_hash` field failed"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
            Self::WalletConflicts(ref e) =>
                write_err!(f, "conversion of the `wallet_conflicts` field failed"; e),
            Self::ReplacedByTxid(ref e) =>
                write_err!(f, "conversion of the `replaced_by_txid` field failed"; e),
            Self::ReplacesTxid(ref e) =>
                write_err!(f, "conversion of the `replaces_txid` field failed"; e),
            Self::MempoolConflicts(ref e) =>
                write_err!(f, "conversion of the `mempool_conflicts` field failed"; e),
            Self::Tx(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            Self::Details(ref e) => write_err!(f, "conversion of the `details` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Amount(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
            Self::BlockHash(ref e) => Some(e),
            Self::Txid(ref e) => Some(e),
            Self::Wtxid(ref e) => Some(e),
            Self::WalletConflicts(ref e) => Some(e),
            Self::ReplacedByTxid(ref e) => Some(e),
            Self::ReplacesTxid(ref e) => Some(e),
            Self::MempoolConflicts(ref e) => Some(e),
            Self::Tx(ref e) => Some(e),
            Self::Details(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTransactionError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ListSinceBlock` type into the model type.
#[derive(Debug)]
pub enum ListSinceBlockError {
    /// Conversion of the `transactions` field failed.
    Transactions(TransactionItemError),
    /// Conversion of the `removed` field failed.
    Removed(TransactionItemError),
    /// Conversion of the `last_block` field failed.
    LastBlock(hex::DecodeFixedLengthBytesError),
}

impl fmt::Display for ListSinceBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Transactions(ref e) =>
                write_err!(f, "conversion of the `transactions` field failed"; e),
            Self::Removed(ref e) => write_err!(f, "conversion of the `removed` field failed"; e),
            Self::LastBlock(ref e) =>
                write_err!(f, "conversion of the `last_block` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListSinceBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Transactions(ref e) => Some(e),
            Self::Removed(ref e) => Some(e),
            Self::LastBlock(ref e) => Some(e),
        }
    }
}

/// Error when converting a `TransactionItem` type into the model type.
#[derive(Debug)]
pub enum TransactionItemError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `address` field failed.
    Address(bitcoin::address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `txid` field failed.
    Txid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `wallet_conflicts` field failed.
    WalletConflicts(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `replaced_by_txid` field failed.
    ReplacedByTxid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `replaces_txid` field failed.
    ReplacesTxid(hex::DecodeFixedLengthBytesError),
}

impl fmt::Display for TransactionItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            Self::BlockHash(ref e) =>
                write_err!(f, "conversion of the `block_hash` field failed"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
            Self::WalletConflicts(ref e) =>
                write_err!(f, "conversion of the `wallet_conflicts` field failed"; e),
            Self::ReplacedByTxid(ref e) =>
                write_err!(f, "conversion of the `replaced_by_txid` field failed"; e),
            Self::ReplacesTxid(ref e) =>
                write_err!(f, "conversion of the `replaces_txid` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TransactionItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Address(ref e) => Some(e),
            Self::Amount(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
            Self::BlockHash(ref e) => Some(e),
            Self::Txid(ref e) => Some(e),
            Self::Wtxid(ref e) => Some(e),
            Self::WalletConflicts(ref e) => Some(e),
            Self::ReplacedByTxid(ref e) => Some(e),
            Self::ReplacesTxid(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for TransactionItemError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `SendAll` type into the model type.
#[derive(Debug)]
pub enum SendAllError {
    /// Conversion of the `txid` field failed.
    Txid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `hex` field failed.
    Hex(encode::FromHexError),
    /// Conversion of the `psbt` field failed.
    Psbt(PsbtParseError),
}

impl fmt::Display for SendAllError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            Self::Psbt(ref e) => write_err!(f, "conversion of the `psbt` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SendAllError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
            Self::Psbt(ref e) => Some(e),
        }
    }
}
