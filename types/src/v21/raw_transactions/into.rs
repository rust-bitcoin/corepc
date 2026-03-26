// SPDX-License-Identifier: CC0-1.0

use bitcoin::{Amount, Txid};

use super::{MempoolAcceptance, MempoolAcceptanceError, TestMempoolAccept, TestMempoolAcceptError};
use crate::model;

impl TestMempoolAccept {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::TestMempoolAccept, TestMempoolAcceptError> {
        let results = self.0.into_iter().map(|r| r.into_model()).collect::<Result<_, _>>()?;

        Ok(model::TestMempoolAccept { results })
    }
}

impl MempoolAcceptance {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolAcceptance, MempoolAcceptanceError> {
        use MempoolAcceptanceError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let vsize = self.vsize.map(|s| crate::to_u32(s, "vsize")).transpose()?;
        let fees = match self.fees {
            Some(s) => {
                let base = Amount::from_btc(s.base).map_err(E::Base)?;
                Some(model::MempoolAcceptanceFees {
                    base,
                    effective_fee_rate: None, // v25 and later only.
                    effective_includes: None, // v25 and later only.
                })
            }
            None => None,
        };

        Ok(model::MempoolAcceptance {
            txid,
            wtxid: None, // v22 and later only.
            allowed: self.allowed,
            vsize,
            fees,
            reject_reason: self.reject_reason,
            reject_details: None, // v29 and later only.
        })
    }
}
