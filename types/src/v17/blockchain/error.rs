// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::{self, ParseAmountError};
use bitcoin::consensus::encode;
use bitcoin::error::UnprefixedHexError;
use bitcoin::{address, hex, network};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetBlockVerboseOne` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerboseOneError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockVerboseOneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockVerboseOneError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Tx(ref e) => write_err!(f, "conversion of the `tx` field failed"; e),
            Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            ChainWork(ref e) => write_err!(f, "conversion of the `chain_work` field failed"; e),
            PreviousBlockHash(ref e) => {
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e)
            }
            NextBlockHash(ref e) => {
                write_err!(f, "conversion of the `next_block_hash` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockVerboseOneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockVerboseOneError::*;

        match *self {
            Numeric(ref e) => Some(e),
            Hash(ref e) => Some(e),
            Tx(ref e) => Some(e),
            Bits(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
            PreviousBlockHash(ref e) => Some(e),
            NextBlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockVerboseOneError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockchainInfo` type into the model type.
#[derive(Debug)]
pub enum GetBlockchainInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `chain` field failed.
    Chain(network::ParseNetworkError),
    /// Conversion of the `best_block_hash` field failed.
    BestBlockHash(hex::HexToArrayError),
    /// Conversion of the `chain_work` field failed.
    ChainWork(UnprefixedHexError),
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockchainInfoError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Chain(ref e) => write_err!(f, "conversion of the `chain` field failed"; e),
            BestBlockHash(ref e) => {
                write_err!(f, "conversion of the `best_block_hash` field failed"; e)
            }
            ChainWork(ref e) => write_err!(f, "conversion of the `chain_work` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockchainInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockchainInfoError::*;

        match *self {
            Numeric(ref e) => Some(e),
            Chain(ref e) => Some(e),
            BestBlockHash(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockchainInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockHeader` type into the model type.
#[derive(Debug)]
pub enum GetBlockHeaderError {
    /// Conversion of hex data to bytes failed.
    Hex(hex::HexToBytesError),
    /// Consensus decoding of bytes to header failed.
    Consensus(encode::Error),
}

impl fmt::Display for GetBlockHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockHeaderError::*;

        match *self {
            Hex(ref e) => write_err!(f, "conversion of hex data to bytes failed"; e),
            Consensus(ref e) => write_err!(f, "consensus decoding of bytes to header failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockHeaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockHeaderError::*;

        match *self {
            Hex(ref e) => Some(e),
            Consensus(ref e) => Some(e),
        }
    }
}

/// Error when converting a `GetBlockHeader` type into the model type.
#[derive(Debug)]
pub enum GetBlockHeaderVerboseError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of `merkle_root` field failed.
    MerkleRoot(hex::HexToArrayError),
    /// Conversion of `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockHeaderVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockHeaderVerboseError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            MerkleRoot(ref e) => write_err!(f, "conversion of the `merkle_root` field failed"; e),
            Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            ChainWork(ref e) => write_err!(f, "conversion of the `chain_work` field failed"; e),
            PreviousBlockHash(ref e) => {
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e)
            }
            NextBlockHash(ref e) => {
                write_err!(f, "conversion of the `next_block_hash` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockHeaderVerboseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockHeaderVerboseError::*;

        match *self {
            Numeric(ref e) => Some(e),
            Hash(ref e) => Some(e),
            MerkleRoot(ref e) => Some(e),
            Bits(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
            PreviousBlockHash(ref e) => Some(e),
            NextBlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockHeaderVerboseError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockStats` type into the model type.
#[derive(Debug)]
pub enum GetBlockStatsError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockStatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockStatsError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            BlockHash(ref e) => write_err!(f, "conversion of the `block_hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockStatsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockStatsError::*;

        match *self {
            Numeric(ref e) => Some(e),
            BlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockStatsError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ChainTips` type into the model type.
#[derive(Debug)]
pub enum ChainTipsError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `hash` field failed.
    Hash(hex::HexToArrayError),
}

impl fmt::Display for ChainTipsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ChainTipsError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChainTipsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ChainTipsError::*;

        match *self {
            Numeric(ref e) => Some(e),
            Hash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ChainTipsError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetChainTxStats` type into the model type.
#[derive(Debug)]
pub enum GetChainTxStatsError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `window_final_block_hash` field failed.
    WindowFinalBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetChainTxStatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetChainTxStatsError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            WindowFinalBlockHash(ref e) => {
                write_err!(f, "conversion of the `window_final_block_hash` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetChainTxStatsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetChainTxStatsError::*;

        match *self {
            Numeric(ref e) => Some(e),
            WindowFinalBlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetChainTxStatsError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `MapMempoolEntry` into the model type.
#[derive(Debug)]
pub enum MapMempoolEntryError {
    /// Conversion of a `txid` failed.
    Txid(hex::HexToArrayError),
    /// Conversion of a `MempoolEntry` value inside a map failed.
    MempoolEntry(MempoolEntryError),
}

impl fmt::Display for MapMempoolEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MapMempoolEntryError as E;

        match *self {
            E::Txid(ref e) => write_err!(f, "conversion of a `txid` failed"; e),
            E::MempoolEntry(ref e) => write_err!(f, "conversion of an `MempoolEntry` failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MapMempoolEntryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use MapMempoolEntryError as E;

        match *self {
            E::Txid(ref e) => Some(e),
            E::MempoolEntry(ref e) => Some(e),
        }
    }
}

/// Error when converting a `MempoolEntry` type into the model type.
#[derive(Debug)]
pub enum MempoolEntryError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::HexToArrayError),
    /// Conversion of the `MempoolEntryFees` type failed.
    Fees(MempoolEntryFeesError),
    /// Conversion of the `depends` field failed.
    Depends(hex::HexToArrayError),
    /// Conversion of the `spent_by` field failed.
    SpentBy(hex::HexToArrayError),
}

impl From<NumericError> for MempoolEntryError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

impl fmt::Display for MempoolEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MempoolEntryError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
            E::Fees(ref e) => write_err!(f, "conversion of the `fees` field failed"; e),
            E::Depends(ref e) => write_err!(f, "conversion of the `depends` field failed"; e),
            E::SpentBy(ref e) => write_err!(f, "conversion of the `spent_by` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MempoolEntryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use MempoolEntryError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Wtxid(ref e) => Some(e),
            E::Fees(ref e) => Some(e),
            E::Depends(ref e) => Some(e),
            E::SpentBy(ref e) => Some(e),
        }
    }
}

/// Error when converting a `MempoolEntryFees` type into the model type.
#[derive(Debug)]
pub enum MempoolEntryFeesError {
    /// Conversion of the `base` field failed.
    Base(ParseAmountError),
    /// Conversion of the `modified` field failed.
    Modified(ParseAmountError),
    /// Conversion of the `ancestor` field failed.
    Ancestor(ParseAmountError),
    /// Conversion of the `descendant` field failed.
    Descendant(ParseAmountError),
}

impl fmt::Display for MempoolEntryFeesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MempoolEntryFeesError as E;

        match *self {
            E::Base(ref e) => write_err!(f, "conversion of the `base` field failed"; e),
            E::Modified(ref e) => write_err!(f, "conversion of the `modified` field failed"; e),
            E::Ancestor(ref e) => write_err!(f, "conversion of the `ancestor` field failed"; e),
            E::Descendant(ref e) => write_err!(f, "conversion of the `descendant` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MempoolEntryFeesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use MempoolEntryFeesError as E;

        match *self {
            E::Base(ref e) => Some(e),
            E::Modified(ref e) => Some(e),
            E::Ancestor(ref e) => Some(e),
            E::Descendant(ref e) => Some(e),
        }
    }
}

/// Error when converting a `GetMempoolInfo` type into the model type.
#[derive(Debug)]
pub enum GetMempoolInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of a fee rate failed.
    FeeRate(ParseAmountError),
}

impl fmt::Display for GetMempoolInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetMempoolInfoError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::FeeRate(ref e) => write_err!(f, "fee rate"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetMempoolInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetMempoolInfoError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::FeeRate(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetMempoolInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

impl From<ParseAmountError> for GetMempoolInfoError {
    fn from(e: ParseAmountError) -> Self { Self::FeeRate(e) }
}

/// Error when converting a `GetTxOut` type into the model type.
///
/// Note that variants in this type are not named in the usual fashion. The `ScriptBuf` and
/// `Address` variants are named after the functions on [`crate::ScriptPubkey`].
#[derive(Debug)]
pub enum GetTxOutError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `value` field failed.
    Value(amount::ParseAmountError),
    /// Conversion of the `ScriptPubkey` hex to a `ScriptBuf` failed.
    ScriptBuf(hex::HexToBytesError),
    /// Conversion of the `ScriptPubkey` `address` field failed.
    Address(address::ParseError),
}

impl fmt::Display for GetTxOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTxOutError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            BestBlock(ref e) => write_err!(f, "conversion of the `beast_block` field failed"; e),
            Value(ref e) => write_err!(f, "conversion of the `value` field failed"; e),
            ScriptBuf(ref e) => {
                write_err!(f, "conversion of the `ScriptPubkey` hex to a `ScriptBuf` failed"; e)
            }
            Address(ref e) => {
                write_err!(f, "conversion of the `ScriptPubkey` `address` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxOutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTxOutError::*;

        match *self {
            Numeric(ref e) => Some(e),
            BestBlock(ref e) => Some(e),
            Value(ref e) => Some(e),
            ScriptBuf(ref e) => Some(e),
            Address(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTxOutError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetTxOutSetInfo` type into the model type.
#[derive(Debug)]
pub enum GetTxOutSetInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `total_amount` field failed.
    TotalAmount(amount::ParseAmountError),
}

impl fmt::Display for GetTxOutSetInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTxOutSetInfoError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            BestBlock(ref e) => write_err!(f, "conversion of the `best_block` field failed"; e),
            TotalAmount(ref e) => write_err!(f, "conversion of the `total_amount` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxOutSetInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTxOutSetInfoError::*;

        match *self {
            Numeric(ref e) => Some(e),
            BestBlock(ref e) => Some(e),
            TotalAmount(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTxOutSetInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
