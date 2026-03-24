// SPDX-License-Identifier: CC0-1.0

use bitcoin::{Amount, Txid, Wtxid};

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
        let wtxid = self.wtxid.parse::<Wtxid>().map_err(E::Wtxid)?;
        let vsize = self.vsize.map(|s| crate::to_u32(s, "vsize")).transpose()?;
        let fees = self
            .fees
            .map(|s| {
                let effective_includes = if s.effective_includes.is_empty() {
                    None
                } else {
                    Some(
                        s.effective_includes
                            .into_iter()
                            .map(|w| w.parse::<Wtxid>().map_err(E::Wtxid))
                            .collect::<Result<_, _>>()?,
                    )
                };

                let effective_fee_rate = s
                    .effective_fee_rate
                    .map(|f| crate::btc_per_kb(f).map_err(E::Feerate))
                    .transpose()?
                    .flatten();

                Ok::<_, MempoolAcceptanceError>(model::MempoolAcceptanceFees {
                    base: Amount::from_btc(s.base).map_err(E::Base)?,
                    effective_fee_rate,
                    effective_includes,
                })
            })
            .transpose()?;

        Ok(model::MempoolAcceptance {
            txid,
            wtxid: Some(wtxid),
            allowed: self.allowed,
            vsize,
            fees,
            reject_reason: self.reject_reason,
            reject_details: self.reject_details,
        })
    }
}
