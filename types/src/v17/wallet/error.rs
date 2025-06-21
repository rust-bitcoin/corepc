// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::psbt::PsbtParseError;
use bitcoin::{address, bip32, hex, key, witness_program, witness_version};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `AddMultisigAddress` type into the model type.
#[derive(Debug)]
pub enum AddMultisigAddressError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
}

impl fmt::Display for AddMultisigAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AddMultisigAddressError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            RedeemScript(ref e) => {
                write_err!(f, "conversion of the `redeem_script` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AddMultisigAddressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use AddMultisigAddressError::*;

        match *self {
            Address(ref e) => Some(e),
            RedeemScript(ref e) => Some(e),
        }
    }
}

/// Error when converting a `BumpFee` type into the model type.
#[derive(Debug)]
pub enum BumpFeeError {
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `original_fee` field failed.
    OriginalFee(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for BumpFeeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BumpFeeError as E;

        match *self {
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::OriginalFee(ref e) => {
                write_err!(f, "conversion of the `original_fee` field failed"; e)
            }
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BumpFeeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use BumpFeeError as E;

        match *self {
            E::Txid(ref e) => Some(e),
            E::OriginalFee(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}

/// Error when converting a `GetAddressInfo` type into the model type.
#[derive(Debug)]
pub enum GetAddressInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
    /// The `witness_version` field's value was too big for a u8.
    WitnessVersionValue(i64),
    /// Conversion of the `witness_version` field failed.
    WitnessVersion(witness_version::TryFromError),
    /// Conversion of the `witness_program` field hex string to bytes failed.
    WitnessProgramBytes(hex::HexToBytesError),
    /// Conversion of the `witness_program` field failed.
    WitnessProgram(witness_program::Error),
    /// Conversion of the `hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the `pubkeys` field failed.
    Pubkeys(key::ParsePublicKeyError),
    /// Conversion of the `pubkey` field failed.
    Pubkey(key::ParsePublicKeyError),
    /// Conversion of the `embedded` field failed.
    Embedded(GetAddressInfoEmbeddedError),
    /// Conversion of the `hd_key_path` field failed.
    HdKeyPath(bip32::Error),
    /// Conversion of the `hd_seed_id` field failed.
    HdSeedId(hex::HexToArrayError),
}

impl fmt::Display for GetAddressInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetAddressInfoError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::ScriptPubkey(ref e) => {
                write_err!(f, "conversion of the `script_pubkey` field failed"; e)
            }
            E::WitnessVersionValue(v) => write!(f, "invalid witness version number: {}", v),
            E::WitnessVersion(ref e) => {
                write_err!(f, "conversion of the `witness_version` field failed"; e)
            }
            E::WitnessProgramBytes(ref e) => {
                write_err!(f, "conversion of the `witness_program` field hex string to bytes failed"; e)
            }
            E::WitnessProgram(ref e) => {
                write_err!(f, "conversion of the `witness_program` field failed"; e)
            }
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Pubkeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            E::Pubkey(ref e) => write_err!(f, "conversion of the `pubkey` failed"; e),
            E::Embedded(ref e) => write_err!(f, "conversion of the `embedded` field failed"; e),
            E::HdKeyPath(ref e) => write_err!(f, "conversion of the `hd_key_path` field failed"; e),
            E::HdSeedId(ref e) => write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetAddressInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetAddressInfoError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::ScriptPubkey(ref e) => Some(e),
            E::WitnessVersionValue(_) => None,
            E::WitnessVersion(ref e) => Some(e),
            E::WitnessProgramBytes(ref e) => Some(e),
            E::WitnessProgram(ref e) => Some(e),
            E::Hex(ref e) => Some(e),
            E::Pubkeys(ref e) => Some(e),
            E::Pubkey(ref e) => Some(e),
            E::Embedded(ref e) => Some(e),
            E::HdKeyPath(ref e) => Some(e),
            E::HdSeedId(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetAddressInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetAddressInfoEmbedded` type into the model type.
#[derive(Debug)]
pub enum GetAddressInfoEmbeddedError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
    /// The `witness_version` field's value was too big for a u8.
    WitnessVersionValue(i64),
    /// Conversion of the `witness_version` field failed.
    WitnessVersion(witness_version::TryFromError),
    /// Conversion of the `witness_program` field hex string to bytes failed.
    WitnessProgramBytes(hex::HexToBytesError),
    /// Conversion of the `witness_program` field failed.
    WitnessProgram(witness_program::Error),
    /// Conversion of the `hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the `pubkeys` field failed.
    Pubkeys(key::ParsePublicKeyError),
    /// Conversion of the `pubkey` field failed.
    Pubkey(key::ParsePublicKeyError),
}

impl fmt::Display for GetAddressInfoEmbeddedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetAddressInfoEmbeddedError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::ScriptPubkey(ref e) => {
                write_err!(f, "conversion of the `script_pubkey` field failed"; e)
            }
            E::WitnessVersionValue(v) => write!(f, "invalid witness version number: {}", v),
            E::WitnessVersion(ref e) => {
                write_err!(f, "conversion of the `witness_version` field failed"; e)
            }
            E::WitnessProgramBytes(ref e) => {
                write_err!(f, "conversion of the `witness_program` field hex string to bytes failed"; e)
            }
            E::WitnessProgram(ref e) => {
                write_err!(f, "conversion of the `witness_program` field failed"; e)
            }
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Pubkeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            E::Pubkey(ref e) => write_err!(f, "conversion of the `pubkey` failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetAddressInfoEmbeddedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetAddressInfoEmbeddedError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::ScriptPubkey(ref e) => Some(e),
            E::WitnessVersionValue(_) => None,
            E::WitnessVersion(ref e) => Some(e),
            E::WitnessProgramBytes(ref e) => Some(e),
            E::WitnessProgram(ref e) => Some(e),
            E::Hex(ref e) => Some(e),
            E::Pubkeys(ref e) => Some(e),
            E::Pubkey(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetAddressInfoEmbeddedError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
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
    /// Conversion of the `wallet_conflicts` field failed.
    WalletConflicts(hex::HexToArrayError),
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
            E::WalletConflicts(ref e) => {
                write_err!(f, "conversion of the `wallet_conflicts` field failed"; e)
            }
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
            E::Tx(ref e) => Some(e),
            E::Details(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTransactionError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting to a `GetTransactionDetail` type to a `concrete` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetTransactionDetailError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for GetTransactionDetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTransactionDetailError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTransactionDetailError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTransactionDetailError as E;

        match *self {
            E::Address(ref e) => Some(e),
            E::Amount(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
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
}

impl fmt::Display for GetWalletInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetWalletInfoError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Balance(ref e) => write_err!(f, "conversion of the `balance` field failed"; e),
            UnconfirmedBalance(ref e) => {
                write_err!(f, "conversion of the `unconfirmed_balance` field failed"; e)
            }
            ImmatureBalance(ref e) => {
                write_err!(f, "conversion of the `immature_balance` field failed"; e)
            }
            PayTxFee(ref e) => write_err!(f, "conversion of the `pay_tx_fee` field failed"; e),
            HdSeedId(ref e) => write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
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
        }
    }
}

impl From<NumericError> for GetWalletInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ListAddressGroupings` type into the model type.
#[derive(Debug)]
pub enum ListAddressGroupingsError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
}

impl fmt::Display for ListAddressGroupingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListAddressGroupingsError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListAddressGroupingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListAddressGroupingsError::*;

        match *self {
            Address(ref e) => Some(e),
            Amount(ref e) => Some(e),
        }
    }
}

/// Error when converting a `ListLockUnspentItem` type into the model type.
#[derive(Debug)]
pub enum ListLockUnspentItemError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `txid` field failed.
    Txid(hex::HexToArrayError),
}

impl fmt::Display for ListLockUnspentItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListLockUnspentItemError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListLockUnspentItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListLockUnspentItemError::*;

        match *self {
            Numeric(ref e) => Some(e),
            Txid(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ListLockUnspentItemError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ListReceivedByAddress` type into the model type.
#[derive(Debug)]
pub enum ListReceivedByAddressError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of txid in the `txids` list (with index) failed.
    Txids(usize, hex::HexToArrayError),
}

impl fmt::Display for ListReceivedByAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListReceivedByAddressError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Txids(index, ref e) => {
                write_err!(f, "conversion of the txid at index {} in the `txids` field failed", index; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListReceivedByAddressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListReceivedByAddressError::*;

        match *self {
            Address(ref e) => Some(e),
            Amount(ref e) => Some(e),
            Txids(_, ref e) => Some(e),
        }
    }
}

/// Error when converting a `ListSinceBlock` type into the model type.
#[derive(Debug)]
pub enum ListSinceBlockError {
    /// Conversion of item in `transactions` list failed.
    Transactions(ListSinceBlockTransactionError),
    /// Conversion of item in `removed` list failed.
    Removed(ListSinceBlockTransactionError),
    /// Conversion of the `last_block` field failed.
    LastBlock(hex::HexToArrayError),
}

impl fmt::Display for ListSinceBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListSinceBlockError::*;

        match *self {
            Transactions(ref e) => {
                write_err!(f, "conversion of the `transactions` field failed"; e)
            }
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
    Address(address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::HexToArrayError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
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
        }
    }
}

impl From<NumericError> for ListSinceBlockTransactionError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ListTransactionsItem` type into the model type.
#[derive(Debug)]
pub enum ListTransactionsItemError {
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
}

impl fmt::Display for ListTransactionsItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListTransactionsItemError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            E::BlockHash(ref e) => write_err!(f, "conversion of the `block_hash` field failed"; e),
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListTransactionsItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListTransactionsItemError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::Amount(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
            E::BlockHash(ref e) => Some(e),
            E::Txid(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ListTransactionsItemError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ListUnspentItem` type into the model type.
#[derive(Debug)]
pub enum ListUnspentItemError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for ListUnspentItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListUnspentItemError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::ScriptPubkey(ref e) => {
                write_err!(f, "conversion of the `script_pubkey` field failed"; e)
            }
            E::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            E::RedeemScript(ref e) => {
                write_err!(f, "conversion of the `redeem_script` field failed"; e)
            }
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListUnspentItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListUnspentItemError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Txid(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::ScriptPubkey(ref e) => Some(e),
            E::Amount(ref e) => Some(e),
            E::RedeemScript(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ListUnspentItemError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `WalletCreateFundedPsbt` type into the model type.
#[derive(Debug)]
pub enum WalletCreateFundedPsbtError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `psbt` field failed.
    Psbt(PsbtParseError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for WalletCreateFundedPsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use WalletCreateFundedPsbtError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Psbt(ref e) => write_err!(f, "conversion of the `psbt` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WalletCreateFundedPsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use WalletCreateFundedPsbtError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Psbt(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for WalletCreateFundedPsbtError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
