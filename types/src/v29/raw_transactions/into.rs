// SPDX-License-Identifier: CC0-1.0

use bitcoin::{Amount, Txid, Wtxid};

// TODO: Use explicit imports?
use super::*;

impl SubmitPackage {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SubmitPackage, SubmitPackageError> {
        use SubmitPackageError as E;

        let mut tx_results = BTreeMap::new();
        for (k, v) in self.tx_results {
            let wtxid = k.parse::<Wtxid>().map_err(E::TxResultKey)?;
            let result = v.into_model().map_err(E::TxResultValue)?;
            tx_results.insert(wtxid, result);
        }

        let replaced_transactions = self
            .replaced_transactions
            .iter()
            .map(|tx| tx.parse::<Txid>().map_err(E::ReplaceTransactions))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::SubmitPackage {
            package_msg: self.package_msg,
            tx_results,
            replaced_transactions,
        })
    }
}

impl SubmitPackageTxResult {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SubmitPackageTxResult, SubmitPackageTxResultError> {
        use SubmitPackageTxResultError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let other_wtxid =
            self.other_wtxid.map(|s| s.parse::<Wtxid>().map_err(E::OtherWtxid)).transpose()?;
        let vsize = self.vsize.map(|vsize| crate::to_u32(vsize, "vsize")).transpose()?;
        let fees = self.fees.map(|fees| fees.into_model().map_err(E::Fees)).transpose()?;

        Ok(model::SubmitPackageTxResult { txid, other_wtxid, vsize, fees, error: self.error })
    }
}

impl SubmitPackageTxResultFees {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::SubmitPackageTxResultFees, SubmitPackageTxResultFeesError> {
        use SubmitPackageTxResultFeesError as E;

        let base_fee = Amount::from_btc(self.base_fee).map_err(E::BaseFee)?;
        let effective_fee_rate = self
            .effective_fee_rate
            .map(|f| crate::btc_per_kb(f).map_err(E::EffectiveFeeRate))
            .transpose()?
            .flatten();
        let effective_includes = self
            .effective_includes
            .iter()
            .map(|s| s.parse::<Wtxid>().map_err(E::EffectiveIncludes))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::SubmitPackageTxResultFees { base_fee, effective_fee_rate, effective_includes })
    }
}
