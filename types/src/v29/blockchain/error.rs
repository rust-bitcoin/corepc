// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::consensus::encode;
use bitcoin::error::UnprefixedHexError;
use bitcoin::hex::HexToBytesError;
use bitcoin::{address, amount, hex, network};

use crate::error::write_err;
use crate::psbt::{RawTransactionInputError, RawTransactionOutputError};
use crate::v17::GetRawTransactionVerboseError;
use crate::{NumericError, ScriptPubkeyError};

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
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockVerboseOneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::Tx(ref e) => write_err!(f, "conversion of the `tx` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockVerboseOneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::Tx(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::NextBlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockVerboseOneError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockVerboseTwo` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerboseTwoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
    /// Conversion of a transaction entry failed.
    Transaction(GetRawTransactionVerboseError),
    /// Conversion of the transaction `fee` field failed.
    Fee(amount::ParseAmountError),
}

impl fmt::Display for GetBlockVerboseTwoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
            Self::Transaction(ref e) =>
                write_err!(f, "conversion of a transaction entry failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockVerboseTwoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::NextBlockHash(ref e) => Some(e),
            Self::Transaction(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockVerboseTwoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockVerboseThree` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerboseThreeError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
    /// Conversion of one of the transaction inputs failed.
    Inputs(RawTransactionInputError),
    /// Conversion of one of the transaction outputs failed.
    Outputs(RawTransactionOutputError),
    /// Conversion of the transaction `block_hash` field failed.
    TransactionBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `fee` field failed.
    Fee(amount::ParseAmountError),
    /// Conversion of a prevout height failed.
    PrevoutHeight(NumericError),
    /// Conversion of a prevout value failed.
    PrevoutValue(amount::ParseAmountError),
    /// Conversion of a prevout script_pubkey failed.
    PrevoutScriptPubkey(ScriptPubkeyError),
}

impl fmt::Display for GetBlockVerboseThreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
            Self::Inputs(ref e) =>
                write_err!(f, "conversion of one of the transaction inputs failed"; e),
            Self::Outputs(ref e) =>
                write_err!(f, "conversion of one of the transaction outputs failed"; e),
            Self::TransactionBlockHash(ref e) =>
                write_err!(f, "conversion of the `block_hash` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            Self::PrevoutHeight(ref e) =>
                write_err!(f, "conversion of a prevout `height` field failed"; e),
            Self::PrevoutValue(ref e) =>
                write_err!(f, "conversion of a prevout `value` field failed"; e),
            Self::PrevoutScriptPubkey(ref e) =>
                write_err!(f, "conversion of a prevout `script_pubkey` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockVerboseThreeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::NextBlockHash(ref e) => Some(e),
            Self::Inputs(ref e) => Some(e),
            Self::Outputs(ref e) => Some(e),
            Self::TransactionBlockHash(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
            Self::PrevoutHeight(ref e) => Some(e),
            Self::PrevoutValue(ref e) => Some(e),
            Self::PrevoutScriptPubkey(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockVerboseThreeError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockchainInfo` type into the model type.
#[derive(Debug)]
pub enum GetBlockchainInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `chain` field failed.
    Chain(network::ParseNetworkError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of the `best_block_hash` field failed.
    BestBlockHash(hex::HexToArrayError),
    /// Conversion of the `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the `script` field failed.
    SignetChallenge(hex::HexToBytesError),
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Chain(ref e) => write_err!(f, "conversion of the `chain` field failed"; e),
            Self::BestBlockHash(ref e) =>
                write_err!(f, "conversion of the `best_block_hash` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::SignetChallenge(ref e) =>
                write_err!(f, "conversion of the `signet_challenge` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockchainInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Chain(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::BestBlockHash(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::SignetChallenge(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockchainInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

impl From<HexToBytesError> for GetBlockchainInfoError {
    fn from(e: HexToBytesError) -> Self { Self::SignetChallenge(e) }
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
        match *self {
            Self::Hex(ref e) => write_err!(f, "conversion of hex data to bytes failed"; e),
            Self::Consensus(ref e) =>
                write_err!(f, "consensus decoding of bytes to header failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockHeaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Hex(ref e) => Some(e),
            Self::Consensus(ref e) => Some(e),
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
    /// Conversion of `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockHeaderVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::MerkleRoot(ref e) =>
                write_err!(f, "conversion of the `merkle_root` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
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
            Target(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
            PreviousBlockHash(ref e) => Some(e),
            NextBlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockHeaderVerboseError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ChainState` type into the model type.
#[derive(Debug)]
pub enum GetChainStatesError {
    /// Conversion of the `best_block_hash` field failed.
    BestBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
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
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::SnapshotBlockHash(ref e) =>
                write_err!(f, "conversion of the `snapshot_block_hash` field failed"; e),
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetChainStatesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetChainStatesError::*;

        match *self {
            BestBlockHash(ref e) => Some(e),
            Bits(ref e) => Some(e),
            Target(ref e) => Some(e),
            SnapshotBlockHash(ref e) => Some(e),
            Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetChainStatesError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetDescriptorActivity` type into the model type.
#[derive(Debug)]
pub enum GetDescriptorActivityError {
    /// Conversion of numeric type (e.g., height) to expected type failed.
    Numeric(NumericError),
    /// Conversion of a hash string (BlockHash, Txid) failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the `amount` field (f64 BTC) failed.
    Amount(amount::ParseAmountError),
    /// Conversion of script hex to ScriptBuf failed.
    Script(hex::HexToBytesError),
    /// Conversion of address string to Address failed.
    Address(address::ParseError),
    /// An error occurred during processing of an individual activity entry.
    /// We wrap the inner error to provide context. This might not be strictly necessary
    /// if the inner errors are distinct enough, but can be helpful.
    ActivityEntry(Box<GetDescriptorActivityError>), // Use Box to avoid recursive type size issues
    /// Conversion of the `prevout_spk` field failed.
    PrevoutSpk(ScriptPubkeyError),
    /// Conversion of the `output_spk` field failed.
    OutputSpk(ScriptPubkeyError),
}

impl fmt::Display for GetDescriptorActivityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric conversion failed"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of a hash string failed"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::Script(ref e) => write_err!(f, "conversion of the script `hex` field failed"; e),
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::ActivityEntry(ref e) =>
                write_err!(f, "conversion of an activity entry failed"; e),
            Self::PrevoutSpk(ref e) =>
                write_err!(f, "conversion of the `prevout_spk` field failed"; e),
            Self::OutputSpk(ref e) =>
                write_err!(f, "conversion of the `output_spk` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetDescriptorActivityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetDescriptorActivityError::*;

        match *self {
            Numeric(ref e) => Some(e),
            Hash(ref e) => Some(e),
            Amount(ref e) => Some(e),
            Script(ref e) => Some(e),
            Address(ref e) => Some(e),
            ActivityEntry(ref e) => Some(&**e), // Deref the Box to get the inner error
            PrevoutSpk(ref e) => Some(e),
            OutputSpk(ref e) => Some(e),
        }
    }
}

// Implement From for inner errors to allow easy wrapping with `?`
impl From<NumericError> for GetDescriptorActivityError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

impl From<hex::HexToArrayError> for GetDescriptorActivityError {
    fn from(e: hex::HexToArrayError) -> Self { Self::Hash(e) }
}

impl From<amount::ParseAmountError> for GetDescriptorActivityError {
    fn from(e: amount::ParseAmountError) -> Self { Self::Amount(e) }
}

impl From<hex::HexToBytesError> for GetDescriptorActivityError {
    fn from(e: hex::HexToBytesError) -> Self { Self::Script(e) }
}

impl From<address::ParseError> for GetDescriptorActivityError {
    fn from(e: address::ParseError) -> Self { Self::Address(e) }
}
