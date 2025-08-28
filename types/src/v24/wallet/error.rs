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
    BlockHash(hex::HexToArrayError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::HexToArrayError),
    /// Conversion of the `wallet_conflicts` field failed.
    WalletConflicts(hex::HexToArrayError),
    /// Conversion of the `replaced_by_txid` field failed.
    ReplacedByTxid(hex::HexToArrayError),
    /// Conversion of the `replaces_txid` field failed.
    ReplacesTxid(hex::HexToArrayError),
    /// Conversion of the `mempool_conflicts` field failed.
    MempoolConflicts(hex::HexToArrayError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the `details` field failed.
    Details(GetTransactionDetailError),
}

impl fmt::Display for GetTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTransactionError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            E::BlockHash(ref e) => write_err!(f, "conversion of the `block_hash` field failed"; e),
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
            E::WalletConflicts(ref e) =>
                write_err!(f, "conversion of the `wallet_conflicts` field failed"; e),
            E::ReplacedByTxid(ref e) =>
                write_err!(f, "conversion of the `replaced_by_txid` field failed"; e),
            E::ReplacesTxid(ref e) =>
                write_err!(f, "conversion of the `replaces_txid` field failed"; e),
            E::MempoolConflicts(ref e) =>
                write_err!(f, "conversion of the `mempool_conflicts` field failed"; e),
            E::Tx(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Details(ref e) => write_err!(f, "conversion of the `details` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTransactionError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Amount(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
            E::BlockHash(ref e) => Some(e),
            E::Txid(ref e) => Some(e),
            E::Wtxid(ref e) => Some(e),
            E::WalletConflicts(ref e) => Some(e),
            E::ReplacedByTxid(ref e) => Some(e),
            E::ReplacesTxid(ref e) => Some(e),
            E::MempoolConflicts(ref e) => Some(e),
            E::Tx(ref e) => Some(e),
            E::Details(ref e) => Some(e),
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
    Transactions(ListSinceBlockTransactionError),
    /// Conversion of the `removed` field failed.
    Removed(ListSinceBlockTransactionError),
    /// Conversion of the `last_block` field failed.
    LastBlock(hex::HexToArrayError),
}

impl fmt::Display for ListSinceBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListSinceBlockError::*;
        match *self {
            Transactions(ref e) =>
                write_err!(f, "conversion of the `transactions` field failed"; e),
            Removed(ref e) => write_err!(f, "conversion of the `removed` field failed"; e),
            LastBlock(ref e) => write_err!(f, "conversion of the `last_block` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListSinceBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListSinceBlockError::*;
        match *self {
            Transactions(ref e) => Some(e),
            Removed(ref e) => Some(e),
            LastBlock(ref e) => Some(e),
        }
    }
}

/// Error when converting a `ListSinceBlockTransaction` type into the model type.
#[derive(Debug)]
pub enum ListSinceBlockTransactionError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `address` field failed.
    Address(bitcoin::address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::HexToArrayError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::HexToArrayError),
    /// Conversion of the `wallet_conflicts` field failed.
    WalletConflicts(hex::HexToArrayError),
    /// Conversion of the `replaced_by_txid` field failed.
    ReplacedByTxid(hex::HexToArrayError),
    /// Conversion of the `replaces_txid` field failed.
    ReplacesTxid(hex::HexToArrayError),
}

impl fmt::Display for ListSinceBlockTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListSinceBlockTransactionError as E;
        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            E::BlockHash(ref e) => write_err!(f, "conversion of the `block_hash` field failed"; e),
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
            E::WalletConflicts(ref e) =>
                write_err!(f, "conversion of the `wallet_conflicts` field failed"; e),
            E::ReplacedByTxid(ref e) =>
                write_err!(f, "conversion of the `replaced_by_txid` field failed"; e),
            E::ReplacesTxid(ref e) =>
                write_err!(f, "conversion of the `replaces_txid` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListSinceBlockTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListSinceBlockTransactionError as E;
        match *self {
            E::Numeric(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::Amount(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
            E::BlockHash(ref e) => Some(e),
            E::Txid(ref e) => Some(e),
            E::Wtxid(ref e) => Some(e),
            E::WalletConflicts(ref e) => Some(e),
            E::ReplacedByTxid(ref e) => Some(e),
            E::ReplacesTxid(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ListSinceBlockTransactionError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `SendAll` type into the model type.
#[derive(Debug)]
pub enum SendAllError {
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `hex` field failed.
    Hex(encode::FromHexError),
    /// Conversion of the `psbt` field failed.
    Psbt(PsbtParseError),
}

impl fmt::Display for SendAllError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SendAllError as E;

        match *self {
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Psbt(ref e) => write_err!(f, "conversion of the `psbt` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SendAllError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use SendAllError as E;

        match *self {
            E::Txid(ref e) => Some(e),
            E::Hex(ref e) => Some(e),
            E::Psbt(ref e) => Some(e),
        }
    }
}
