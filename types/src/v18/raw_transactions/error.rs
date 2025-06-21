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
        use AnalyzePsbtError as E;

        match *self {
            E::Inputs(ref e) => write_err!(f, "conversion of one of the `inputs` failed"; e),
            E::EstimatedFeeRate(ref e) => {
                write_err!(f, "conversion of the `estimated_fee_rate` field failed"; e)
            }
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AnalyzePsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use AnalyzePsbtError as E;

        match *self {
            E::Inputs(ref e) => Some(e),
            E::EstimatedFeeRate(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}

/// Error when converting a missing input during `analyzepsbt`.
#[derive(Debug)]
pub enum AnalyzePsbtInputMissingError {
    /// Conversion of the `pubkeys` field failed.
    Pubkeys(hex::HexToArrayError),
    /// Conversion of the `signatures` field failed.
    Signatures(hex::HexToArrayError),
    /// Conversion of the `RedeemScript` field failed.
    RedeemScript(hex::HexToArrayError),
    /// Conversion of the `witness_script` field failed.
    WitnessScript(hex::HexToArrayError),
}

impl fmt::Display for AnalyzePsbtInputMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AnalyzePsbtInputMissingError as E;

        match *self {
            E::Pubkeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            E::Signatures(ref e) => write_err!(f, "conversion of the `signatures` field failed"; e),
            E::RedeemScript(ref e) => {
                write_err!(f, "conversion of the `redeem_script` field failed"; e)
            }
            E::WitnessScript(ref e) => {
                write_err!(f, "conversion of the `witness_script` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AnalyzePsbtInputMissingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use AnalyzePsbtInputMissingError as E;

        match *self {
            E::Pubkeys(ref e) => Some(e),
            E::Signatures(ref e) => Some(e),
            E::RedeemScript(ref e) => Some(e),
            E::WitnessScript(ref e) => Some(e),
        }
    }
}
