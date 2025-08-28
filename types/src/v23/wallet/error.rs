// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::hex;

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
    /// Conversion of the `wallet_conflicts` field failed.
    WalletConflicts(hex::HexToArrayError),
    /// Conversion of the `replaced_by_txid` field failed.
    ReplacedByTxid(hex::HexToArrayError),
    /// Conversion of the `replaces_txid` field failed.
    ReplacesTxid(hex::HexToArrayError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the `details` field failed.
    Details(GetTransactionDetailError),
}

/// Error when converting a `ListSinceBlock` type into the model type.
#[derive(Debug)]
pub enum ListSinceBlockError {
    /// Conversion of the `transactions` field failed.
    Transactions(TransactionItemError),
    /// Conversion of the `removed` field failed.
    Removed(TransactionItemError),
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
    BlockHash(hex::HexToArrayError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `wallet_conflicts` field failed.
    WalletConflicts(hex::HexToArrayError),
    /// Conversion of the `replaced_by_txid` field failed.
    ReplacedByTxid(hex::HexToArrayError),
    /// Conversion of the `replaces_txid` field failed.
    ReplacesTxid(hex::HexToArrayError),
}

impl fmt::Display for TransactionItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TransactionItemError as E;
        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            E::BlockHash(ref e) => write_err!(f, "conversion of the `block_hash` field failed"; e),
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
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
impl std::error::Error for TransactionItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use TransactionItemError as E;
        match *self {
            E::Numeric(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::Amount(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
            E::BlockHash(ref e) => Some(e),
            E::Txid(ref e) => Some(e),
            E::WalletConflicts(ref e) => Some(e),
            E::ReplacedByTxid(ref e) => Some(e),
            E::ReplacesTxid(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for TransactionItemError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
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
            E::WalletConflicts(ref e) =>
                write_err!(f, "conversion of the `wallet_conflicts` field failed"; e),
            E::ReplacedByTxid(ref e) =>
                write_err!(f, "conversion of the `replaced_by_txid` field failed"; e),
            E::ReplacesTxid(ref e) =>
                write_err!(f, "conversion of the `replaces_txid` field failed"; e),
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
            E::WalletConflicts(ref e) => Some(e),
            E::ReplacedByTxid(ref e) => Some(e),
            E::ReplacesTxid(ref e) => Some(e),
            E::Tx(ref e) => Some(e),
            E::Details(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTransactionError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
