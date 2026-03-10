// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::hex::DecodeFixedLengthBytesError;
use bitcoin::psbt::PsbtParseError;

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `DescriptorProcessPsbt` type into the model type.
#[derive(Debug)]
pub enum DescriptorProcessPsbtError {
    /// Conversion of the transaction `psbt` field failed.
    Psbt(PsbtParseError),
    /// Conversion of the transaction `hex` field failed.
    Hex(encode::FromHexError),
}

impl fmt::Display for DescriptorProcessPsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Psbt(ref e) => write_err!(f, "conversion of the `psbt` field failed"; e),
            Self::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DescriptorProcessPsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Psbt(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
        }
    }
}

/// Error when converting a `SubmitPackage` type into the model type.
#[derive(Debug)]
pub enum SubmitPackageError {
    /// Conversion of key from `tx_results` map failed.
    TxResultKey(DecodeFixedLengthBytesError),
    /// Conversion of value from `tx_results` map failed.
    TxResultValue(SubmitPackageTxResultError),
    /// Conversion of a list item from `replaced_transactions` field failed.
    ReplaceTransactions(DecodeFixedLengthBytesError),
}

impl fmt::Display for SubmitPackageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::TxResultKey(ref e) =>
                write_err!(f, "conversion of key from `tx_results` map failed"; e),
            Self::TxResultValue(ref e) =>
                write_err!(f, "conversion of value from `tx_results` map failed"; e),
            Self::ReplaceTransactions(ref e) =>
                write_err!(f, "conversion of a list item from `replaced_transactions` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SubmitPackageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::TxResultKey(ref e) => Some(e),
            Self::TxResultValue(ref e) => Some(e),
            Self::ReplaceTransactions(ref e) => Some(e),
        }
    }
}

/// Error when converting a `SubmitPackageTxResult` type into the model type.
#[derive(Debug)]
pub enum SubmitPackageTxResultError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `txid` field failed.
    Txid(DecodeFixedLengthBytesError),
    /// Conversion of the `other_wtxid` field failed.
    OtherWtxid(DecodeFixedLengthBytesError),
    /// Conversion of the `fees` field failed.
    Fees(SubmitPackageTxResultFeesError),
}

impl fmt::Display for SubmitPackageTxResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::OtherWtxid(ref e) =>
                write_err!(f, "conversion of the `other_wtxid` field failed"; e),
            Self::Fees(ref e) => write_err!(f, "conversion of the `fees` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SubmitPackageTxResultError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Txid(ref e) => Some(e),
            Self::OtherWtxid(ref e) => Some(e),
            Self::Fees(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for SubmitPackageTxResultError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `SubmitPackageTxResultFees` type into the model type.
#[derive(Debug)]
pub enum SubmitPackageTxResultFeesError {
    /// Conversion of the `base_fee` field failed.
    BaseFee(ParseAmountError),
    /// Conversion of the `effective_fee_rate` field failed.
    EffectiveFeeRate(ParseAmountError),
    /// Conversion of a list item from `effective_includes` field failed.
    EffectiveIncludes(DecodeFixedLengthBytesError),
}

impl fmt::Display for SubmitPackageTxResultFeesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::BaseFee(ref e) => write_err!(f, "conversion of the `base_fee` field failed"; e),
            Self::EffectiveFeeRate(ref e) =>
                write_err!(f, "conversion of the `effective_fee_rate` field failed"; e),
            Self::EffectiveIncludes(ref e) => write_err!(f, "effective_includes"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SubmitPackageTxResultFeesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::BaseFee(ref e) => Some(e),
            Self::EffectiveFeeRate(ref e) => Some(e),
            Self::EffectiveIncludes(ref e) => Some(e),
        }
    }
}
