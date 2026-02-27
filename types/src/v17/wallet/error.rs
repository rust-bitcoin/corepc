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
        match *self {
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::RedeemScript(ref e) =>
                write_err!(f, "conversion of the `redeem_script` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AddMultisigAddressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Address(ref e) => Some(e),
            Self::RedeemScript(ref e) => Some(e),
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
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::OriginalFee(ref e) =>
                write_err!(f, "conversion of the `original_fee` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BumpFeeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::OriginalFee(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
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
    ScriptPubKey(hex::HexToBytesError),
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
    PubKeys(key::ParsePublicKeyError),
    /// Conversion of the `pubkey` field failed.
    PubKey(key::ParsePublicKeyError),
    /// Conversion of the `embedded` field failed.
    Embedded(GetAddressInfoEmbeddedError),
    /// Conversion of the `hd_key_path` field failed.
    HdKeyPath(bip32::Error),
    /// Conversion of the `hd_seed_id` field failed.
    HdSeedId(hex::HexToArrayError),
}

impl fmt::Display for GetAddressInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::ScriptPubKey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            Self::WitnessVersionValue(v) => write!(f, "invalid witness version number: {}", v),
            Self::WitnessVersion(ref e) =>
                write_err!(f, "conversion of the `witness_version` field failed"; e),
            Self::WitnessProgramBytes(ref e) =>
                write_err!(f, "conversion of the `witness_program` field hex string to bytes failed"; e),
            Self::WitnessProgram(ref e) =>
                write_err!(f, "conversion of the `witness_program` field failed"; e),
            Self::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            Self::PubKeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            Self::PubKey(ref e) => write_err!(f, "conversion of the `pubkey` failed"; e),
            Self::Embedded(ref e) => write_err!(f, "conversion of the `embedded` field failed"; e),
            Self::HdKeyPath(ref e) =>
                write_err!(f, "conversion of the `hd_key_path` field failed"; e),
            Self::HdSeedId(ref e) =>
                write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetAddressInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Address(ref e) => Some(e),
            Self::ScriptPubKey(ref e) => Some(e),
            Self::WitnessVersionValue(_) => None,
            Self::WitnessVersion(ref e) => Some(e),
            Self::WitnessProgramBytes(ref e) => Some(e),
            Self::WitnessProgram(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
            Self::PubKeys(ref e) => Some(e),
            Self::PubKey(ref e) => Some(e),
            Self::Embedded(ref e) => Some(e),
            Self::HdKeyPath(ref e) => Some(e),
            Self::HdSeedId(ref e) => Some(e),
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
    ScriptPubKey(hex::HexToBytesError),
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
    PubKeys(key::ParsePublicKeyError),
    /// Conversion of the `pubkey` field failed.
    PubKey(key::ParsePublicKeyError),
}

impl fmt::Display for GetAddressInfoEmbeddedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::ScriptPubKey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            Self::WitnessVersionValue(v) => write!(f, "invalid witness version number: {}", v),
            Self::WitnessVersion(ref e) =>
                write_err!(f, "conversion of the `witness_version` field failed"; e),
            Self::WitnessProgramBytes(ref e) =>
                write_err!(f, "conversion of the `witness_program` field hex string to bytes failed"; e),
            Self::WitnessProgram(ref e) =>
                write_err!(f, "conversion of the `witness_program` field failed"; e),
            Self::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            Self::PubKeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            Self::PubKey(ref e) => write_err!(f, "conversion of the `pubkey` failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetAddressInfoEmbeddedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Address(ref e) => Some(e),
            Self::ScriptPubKey(ref e) => Some(e),
            Self::WitnessVersionValue(_) => None,
            Self::WitnessVersion(ref e) => Some(e),
            Self::WitnessProgramBytes(ref e) => Some(e),
            Self::WitnessProgram(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
            Self::PubKeys(ref e) => Some(e),
            Self::PubKey(ref e) => Some(e),
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
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            Self::BlockHash(ref e) =>
                write_err!(f, "conversion of the `block_hash` field failed"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::WalletConflicts(ref e) =>
                write_err!(f, "conversion of the `wallet_conflicts` field failed"; e),
            Self::Tx(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            Self::Details(ref e) => write_err!(f, "conversion of the `details` field failed"; e),
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
            Self::WalletConflicts(ref e) => Some(e),
            Self::Tx(ref e) => Some(e),
            Self::Details(ref e) => Some(e),
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
        match *self {
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTransactionDetailError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Address(ref e) => Some(e),
            Self::Amount(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
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
        match *self {
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListAddressGroupingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Address(ref e) => Some(e),
            Self::Amount(ref e) => Some(e),
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
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListLockUnspentItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Txid(ref e) => Some(e),
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
        match *self {
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::Txids(index, ref e) =>
                write_err!(f, "conversion of the txid at index {} in the `txids` field failed", index; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListReceivedByAddressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Address(ref e) => Some(e),
            Self::Amount(ref e) => Some(e),
            Self::Txids(_, ref e) => Some(e),
        }
    }
}

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
        match *self {
            Self::Transactions(ref e) =>
                write_err!(f, "conversion of the `transactions` field failed"; e),
            Self::Removed(ref e) => write_err!(f, "conversion of the `removed` field failed"; e),
            Self::LastBlock(ref e) =>
                write_err!(f, "conversion of the `last_block` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListSinceBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Transactions(ref e) => Some(e),
            Self::Removed(ref e) => Some(e),
            Self::LastBlock(ref e) => Some(e),
        }
    }
}

/// Error when converting a `TransactionItem` type into the model type.
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
}

impl fmt::Display for TransactionItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            Self::BlockHash(ref e) =>
                write_err!(f, "conversion of the `block_hash` field failed"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TransactionItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Address(ref e) => Some(e),
            Self::Amount(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
            Self::BlockHash(ref e) => Some(e),
            Self::Txid(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for TransactionItemError {
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
    ScriptPubKey(hex::HexToBytesError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for ListUnspentItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::ScriptPubKey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::RedeemScript(ref e) =>
                write_err!(f, "conversion of the `redeem_script` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListUnspentItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Txid(ref e) => Some(e),
            Self::Address(ref e) => Some(e),
            Self::ScriptPubKey(ref e) => Some(e),
            Self::Amount(ref e) => Some(e),
            Self::RedeemScript(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
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
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Psbt(ref e) => write_err!(f, "conversion of the `psbt` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WalletCreateFundedPsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Psbt(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for WalletCreateFundedPsbtError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
