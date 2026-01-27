// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::{amount, hex};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `ChainState` type into the model type.
#[derive(Debug)]
pub enum GetChainStatesError {
    /// Conversion of the `best_block_hash` field failed.
    BestBlockHash(hex::HexToArrayError),
    /// Conversion of the `snapshot_block_hash` field failed.
    SnapshotBlockHash(hex::HexToArrayError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for GetChainStatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::BestBlockHash(ref e) =>
                write_err!(f, "conversion of the `best_block_hash` field failed"; e),
            Self::SnapshotBlockHash(ref e) =>
                write_err!(f, "conversion of the `snapshot_block_hash` field failed"; e),
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetChainStatesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::BestBlockHash(ref e) => Some(e),
            Self::SnapshotBlockHash(ref e) => Some(e),
            Self::Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetChainStatesError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `DumpTxOutSet` type into the model type.
#[derive(Debug)]
pub enum DumpTxOutSetError {
    /// Conversion of the `coins_written` field to Amount failed.
    CoinsWritten(amount::ParseAmountError),
    /// Conversion of the `base_hash` field failed.
    BaseHash(hex::HexToArrayError),
    /// Conversion of the `txoutset_hash` field failed.
    TxOutSetHash(hex::HexToArrayError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for DumpTxOutSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::CoinsWritten(ref e) =>
                write_err!(f, "conversion of the `coins_written` field failed"; e),
            Self::BaseHash(ref e) => write_err!(f, "conversion of the `base_hash` field failed"; e),
            Self::TxOutSetHash(ref e) =>
                write_err!(f, "conversion of the `txoutset_hash` field failed"; e),
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DumpTxOutSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::CoinsWritten(ref e) => Some(e),
            Self::BaseHash(ref e) => Some(e),
            Self::TxOutSetHash(ref e) => Some(e),
            Self::Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for DumpTxOutSetError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetTxOut` type into the model type.
#[derive(Debug)]
pub enum GetTxOutSetInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `total_amount` field failed.
    TotalAmount(amount::ParseAmountError),
    /// Conversion of the `prevout_spent` field failed.
    PrevoutSpent(amount::ParseAmountError),
    /// Conversion of the `coinbase` field failed.
    Coinbase(amount::ParseAmountError),
    /// Conversion of the `new_outputs_ex_coinbase` field failed.
    NewOutputsExCoinbase(amount::ParseAmountError),
    /// Conversion of the `unspendable` field failed.
    Unspendable(amount::ParseAmountError),
    /// Conversion of the `unspendables.genesis_block` field failed.
    UnspendablesGenesisBlock(amount::ParseAmountError),
    /// Conversion of the `unspendables.bip30` field failed.
    UnspendablesBip30(amount::ParseAmountError),
    /// Conversion of the `unspendables.scripts` field failed.
    UnspendablesScripts(amount::ParseAmountError),
    /// Conversion of the `unspendables.unclaimed_rewards` field failed.
    UnspendablesUnclaimedRewards(amount::ParseAmountError),
    /// Conversion of the `total_unspendable_amount` field failed.
    TotalUnspendableAmount(amount::ParseAmountError),
}

impl fmt::Display for GetTxOutSetInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::BestBlock(ref e) =>
                write_err!(f, "conversion of the `best_block` field failed"; e),
            Self::TotalAmount(ref e) =>
                write_err!(f, "conversion of the `total_amount` field failed"; e),
            Self::PrevoutSpent(ref e) =>
                write_err!(f, "conversion of the `prevout_spent` field failed"; e),
            Self::Coinbase(ref e) => write_err!(f, "conversion of the `coinbase` field failed"; e),
            Self::NewOutputsExCoinbase(ref e) =>
                write_err!(f, "conversion of the `new_outputs_ex_coinbase` field failed"; e),
            Self::Unspendable(ref e) =>
                write_err!(f, "conversion of the `unspendable` field failed"; e),
            Self::UnspendablesGenesisBlock(ref e) =>
                write_err!(f, "conversion of the `unspendables.genesis_block` field failed"; e),
            Self::UnspendablesBip30(ref e) =>
                write_err!(f, "conversion of the `unspendables.bip30` field failed"; e),
            Self::UnspendablesScripts(ref e) =>
                write_err!(f, "conversion of the `unspendables.scripts` field failed"; e),
            Self::UnspendablesUnclaimedRewards(ref e) =>
                write_err!(f, "conversion of the `unspendables.unclaimed_rewards` field failed"; e),
            Self::TotalUnspendableAmount(ref e) =>
                write_err!(f, "conversion of the `total_unspendable_amount` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxOutSetInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::BestBlock(ref e) => Some(e),
            Self::TotalAmount(ref e) => Some(e),
            Self::PrevoutSpent(ref e) => Some(e),
            Self::Coinbase(ref e) => Some(e),
            Self::NewOutputsExCoinbase(ref e) => Some(e),
            Self::Unspendable(ref e) => Some(e),
            Self::UnspendablesGenesisBlock(ref e) => Some(e),
            Self::UnspendablesBip30(ref e) => Some(e),
            Self::UnspendablesScripts(ref e) => Some(e),
            Self::UnspendablesUnclaimedRewards(ref e) => Some(e),
            Self::TotalUnspendableAmount(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTxOutSetInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `LoadTxOutSet` type into the model type.
#[derive(Debug)]
pub enum LoadTxOutSetError {
    /// Conversion of the `coins_loaded` field to Amount failed.
    CoinsLoaded(amount::ParseAmountError),
    /// Conversion of the `tip_hash` field failed.
    TipHash(hex::HexToArrayError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for LoadTxOutSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::CoinsLoaded(ref e) =>
                write_err!(f, "conversion of the `coins_loaded` field failed"; e),
            Self::TipHash(ref e) => write_err!(f, "conversion of the `tip_hash` field failed"; e),
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LoadTxOutSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::CoinsLoaded(ref e) => Some(e),
            Self::TipHash(ref e) => Some(e),
            Self::Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for LoadTxOutSetError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
