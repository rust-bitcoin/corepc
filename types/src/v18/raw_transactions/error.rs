// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::hex;

use crate::error::write_err;

/// Error when converting a missing input during `analyzepsbt`.
#[derive(Debug)]
pub enum AnalyzePsbtError {
    /// Conversion of the `inputs` field failed.
    Inputs(AnalyzePsbtInputMissingError),
    /// Conversion of the `estimated_fee_rate` field failed.
    EstimatedFeeRate(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for AnalyzePsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Inputs(ref e) => write_err!(f, "conversion of one of the `inputs` failed"; e),
            Self::EstimatedFeeRate(ref e) =>
                write_err!(f, "conversion of the `estimated_fee_rate` field failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AnalyzePsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Inputs(ref e) => Some(e),
            Self::EstimatedFeeRate(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
        }
    }
}

/// Error when converting a missing input during `analyzepsbt`.
#[derive(Debug)]
pub enum AnalyzePsbtInputMissingError {
    /// Conversion of the `pubkeys` field failed.
    PubKeys(hex::HexToArrayError),
    /// Conversion of the `signatures` field failed.
    Signatures(hex::HexToArrayError),
    /// Conversion of the `RedeemScript` field failed.
    RedeemScript(hex::HexToArrayError),
    /// Conversion of the `witness_script` field failed.
    WitnessScript(hex::HexToArrayError),
}

impl fmt::Display for AnalyzePsbtInputMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::PubKeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            Self::Signatures(ref e) =>
                write_err!(f, "conversion of the `signatures` field failed"; e),
            Self::RedeemScript(ref e) =>
                write_err!(f, "conversion of the `redeem_script` field failed"; e),
            Self::WitnessScript(ref e) =>
                write_err!(f, "conversion of the `witness_script` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AnalyzePsbtInputMissingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::PubKeys(ref e) => Some(e),
            Self::Signatures(ref e) => Some(e),
            Self::RedeemScript(ref e) => Some(e),
            Self::WitnessScript(ref e) => Some(e),
        }
    }
}
