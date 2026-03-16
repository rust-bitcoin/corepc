// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::hex;

use crate::error::write_err;

#[derive(Debug)]
pub enum GetTxSpendingPrevoutError {
    /// Conversion of the `outpoint` field failed.
    Txid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `spending_txid` field failed.
    SpendingTxid(hex::DecodeFixedLengthBytesError),
}

impl fmt::Display for GetTxSpendingPrevoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of the `outpoint` field failed"; e),
            Self::SpendingTxid(ref e) =>
                write_err!(f, "conversion of the `spending_txid` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxSpendingPrevoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::SpendingTxid(ref e) => Some(e),
        }
    }
}
