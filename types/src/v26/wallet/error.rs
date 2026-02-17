// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::{hex, psbt};

use super::GetTransactionDetailError;
use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetBalances` type into the model type.
#[derive(Debug)]
pub enum GetBalancesError {
    /// Conversion of the `mine` field failed.
    Mine(ParseAmountError),
    /// Conversion of the `watchonly` field failed.
    WatchOnly(ParseAmountError),
    /// Conversion of the `last_processed_block` field failed.
    LastProcessedBlock(LastProcessedBlockError),
}

impl fmt::Display for GetBalancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Mine(ref e) => write_err!(f, "conversion of the `mine` field failed"; e),
            Self::WatchOnly(ref e) =>
                write_err!(f, "conversion of the `watchonly` field failed"; e),
            Self::LastProcessedBlock(ref e) =>
                write_err!(f, "conversion of the `last_processed_block` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBalancesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Mine(ref e) => Some(e),
            Self::WatchOnly(ref e) => Some(e),
            Self::LastProcessedBlock(ref e) => Some(e),
        }
    }
}

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
    /// Conversion of the `last_processed_block` field failed.
    LastProcessedBlock(LastProcessedBlockError),
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
            Self::LastProcessedBlock(ref e) =>
                write_err!(f, "conversion of the `last_processed_block` field failed"; e),
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
            Self::LastProcessedBlock(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTransactionError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetWalletInfo` type into the model type.
#[derive(Debug)]
pub enum GetWalletInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `balance` field failed.
    Balance(ParseAmountError),
    /// Conversion of the `unconfirmed_balance` field failed.
    UnconfirmedBalance(ParseAmountError),
    /// Conversion of the `immature_balance` field failed.
    ImmatureBalance(ParseAmountError),
    /// Conversion of the `pay_tx_fee` field failed.
    PayTxFee(ParseAmountError),
    /// Conversion of the `hd_seed_id` field failed.
    HdSeedId(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `last_processed_block` field failed.
    LastProcessedBlock(LastProcessedBlockError),
}

impl fmt::Display for GetWalletInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Balance(ref e) => write_err!(f, "conversion of the `balance` field failed"; e),
            Self::UnconfirmedBalance(ref e) =>
                write_err!(f, "conversion of the `unconfirmed_balance` field failed"; e),
            Self::ImmatureBalance(ref e) =>
                write_err!(f, "conversion of the `immature_balance` field failed"; e),
            Self::PayTxFee(ref e) =>
                write_err!(f, "conversion of the `pay_tx_fee` field failed"; e),
            Self::HdSeedId(ref e) =>
                write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
            Self::LastProcessedBlock(ref e) =>
                write_err!(f, "conversion of the `last_processed_block` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetWalletInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Balance(ref e) => Some(e),
            Self::UnconfirmedBalance(ref e) => Some(e),
            Self::ImmatureBalance(ref e) => Some(e),
            Self::PayTxFee(ref e) => Some(e),
            Self::HdSeedId(ref e) => Some(e),
            Self::LastProcessedBlock(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetWalletInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `LastProcessedBlock` type into the model type.
#[derive(Debug)]
pub enum LastProcessedBlockError {
    /// Conversion of the `hash` field failed.
    Hash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `height` field failed.
    Height(NumericError),
}

impl fmt::Display for LastProcessedBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::Height(ref e) => write_err!(f, "conversion of the `height` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LastProcessedBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Hash(ref e) => Some(e),
            Self::Height(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for LastProcessedBlockError {
    fn from(e: NumericError) -> Self { Self::Height(e) }
}

/// Error when converting a `WalletProcessPsbt` type into the model type.
#[derive(Debug)]
pub enum WalletProcessPsbtError {
    /// Conversion of the `psbt` field failed.
    Psbt(psbt::PsbtParseError),
    /// Conversion of the `hex` field failed.
    Hex(encode::FromHexError),
}

impl fmt::Display for WalletProcessPsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Psbt(ref e) => write!(f, "psbt parse error: {}", e),
            Self::Hex(ref e) => write!(f, "hex decode error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WalletProcessPsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Psbt(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
        }
    }
}
