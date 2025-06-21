// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::consensus::encode;
use bitcoin::error::UnprefixedHexError;
use bitcoin::hex::HexToBytesError;
use bitcoin::{address, amount, hex, network};

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
        use GetBlockVerboseOneError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Tx(ref e) => write_err!(f, "conversion of the `tx` field failed"; e),
            Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
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
            Target(ref e) => Some(e),
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
        use GetBlockchainInfoError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Chain(ref e) => write_err!(f, "conversion of the `chain` field failed"; e),
            BestBlockHash(ref e) => {
                write_err!(f, "conversion of the `best_block_hash` field failed"; e)
            }
            Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            ChainWork(ref e) => write_err!(f, "conversion of the `chain_work` field failed"; e),
            SignetChallenge(ref e) => {
                write_err!(f, "conversion of the `signet_challenge` field failed"; e)
            }
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
            Bits(ref e) => Some(e),
            Target(ref e) => Some(e),
            BestBlockHash(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
            SignetChallenge(ref e) => Some(e),
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
        use GetBlockHeaderVerboseError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            MerkleRoot(ref e) => write_err!(f, "conversion of the `merkle_root` field failed"; e),
            Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
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
}

impl fmt::Display for GetDescriptorActivityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetDescriptorActivityError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric conversion failed"; e),
            Hash(ref e) => write_err!(f, "conversion of a hash string failed"; e),
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Script(ref e) => write_err!(f, "conversion of the script `hex` field failed"; e),
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            ActivityEntry(ref e) => write_err!(f, "conversion of an activity entry failed"; e),
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
