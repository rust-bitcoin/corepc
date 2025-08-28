// SPDX-License-Identifier: CC0-1.0

//! Errors for wallet types newly (re)defined in v20.

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::{address, hex};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `ListSinceBlock` type into the model type.
#[derive(Debug)]
pub enum ListSinceBlockError {
    /// Conversion of item in `transactions` list failed.
    Transactions(TransactionItemError),
    /// Conversion of item in `removed` list failed.
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
///
/// Note: Additional fields introduced in v20 (e.g. `generated`, `trusted`, `block_height`,
/// `wallet_conflicts`, `involvesWatchonly`) are currently not modelled and therefore are
/// intentionally ignored during conversion; as such they have no dedicated error variants.
#[derive(Debug)]
pub enum TransactionItemError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::HexToArrayError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of an item in the `wallet_conflicts` list failed.
    WalletConflicts(hex::HexToArrayError),
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
                write_err!(f, "conversion of an item in the `wallet_conflicts` list failed"; e),
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
        }
    }
}

impl From<NumericError> for TransactionItemError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
