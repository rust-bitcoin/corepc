// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::hex;

use crate::error::write_err;
use crate::NumericError;

#[derive(Debug)]
pub enum GetDeploymentInfoError {
    /// Conversion of the `hash field failed.
    BlockHash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `deployments` field failed.
    Deployment(NumericError),
}

impl fmt::Display for GetDeploymentInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::BlockHash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::Deployment(ref e) =>
                write_err!(f, "conversion of the `deployments` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetDeploymentInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::BlockHash(ref e) => Some(e),
            Self::Deployment(ref e) => Some(e),
        }
    }
}
