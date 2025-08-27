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
        use GetBalancesError as E;

        match *self {
            E::Mine(ref e) => write_err!(f, "conversion of the `mine` field failed"; e),
            E::WatchOnly(ref e) => write_err!(f, "conversion of the `watchonly` field failed"; e),
            E::LastProcessedBlock(ref e) =>
                write_err!(f, "conversion of the `last_processed_block` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBalancesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBalancesError as E;

        match *self {
            E::Mine(ref e) => Some(e),
            E::WatchOnly(ref e) => Some(e),
            E::LastProcessedBlock(ref e) => Some(e),
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
    /// Conversion of the `last_processed_block` field failed.
    LastProcessedBlock(LastProcessedBlockError),
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
            E::LastProcessedBlock(ref e) =>
                write_err!(f, "conversion of the `last_processed_block` field failed"; e),
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
            E::LastProcessedBlock(ref e) => Some(e),
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
    HdSeedId(hex::HexToArrayError),
    /// Conversion of the `last_processed_block` field failed.
    LastProcessedBlock(LastProcessedBlockError),
}

impl fmt::Display for GetWalletInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetWalletInfoError::*;
        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Balance(ref e) => write_err!(f, "conversion of the `balance` field failed"; e),
            UnconfirmedBalance(ref e) =>
                write_err!(f, "conversion of the `unconfirmed_balance` field failed"; e),
            ImmatureBalance(ref e) =>
                write_err!(f, "conversion of the `immature_balance` field failed"; e),
            PayTxFee(ref e) => write_err!(f, "conversion of the `pay_tx_fee` field failed"; e),
            HdSeedId(ref e) => write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
            LastProcessedBlock(ref e) =>
                write_err!(f, "conversion of the `last_processed_block` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetWalletInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetWalletInfoError::*;
        match *self {
            Numeric(ref e) => Some(e),
            Balance(ref e) => Some(e),
            UnconfirmedBalance(ref e) => Some(e),
            ImmatureBalance(ref e) => Some(e),
            PayTxFee(ref e) => Some(e),
            HdSeedId(ref e) => Some(e),
            LastProcessedBlock(ref e) => Some(e),
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
    Hash(hex::HexToArrayError),
    /// Conversion of the `height` field failed.
    Height(NumericError),
}

impl fmt::Display for LastProcessedBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LastProcessedBlockError::*;

        match *self {
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Height(ref e) => write_err!(f, "conversion of the `height` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LastProcessedBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use LastProcessedBlockError::*;

        match *self {
            Hash(ref e) => Some(e),
            Height(ref e) => Some(e),
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
        match self {
            WalletProcessPsbtError::Psbt(e) => write!(f, "psbt parse error: {}", e),
            WalletProcessPsbtError::Hex(e) => write!(f, "hex decode error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WalletProcessPsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WalletProcessPsbtError::Psbt(e) => Some(e),
            WalletProcessPsbtError::Hex(e) => Some(e),
        }
    }
}
