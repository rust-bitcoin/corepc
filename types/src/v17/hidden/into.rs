// SPDX-License-Identifier: CC0-1.0

use bitcoin::BlockHash;

use super::{
    EstimateRawFee, EstimateRawFeeError, RawFeeDetail, RawFeeRange, WaitForBlock,
    WaitForBlockError, WaitForBlockHeight, WaitForBlockHeightError, WaitForNewBlock,
    WaitForNewBlockError,
};
use crate::model;

impl EstimateRawFee {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::EstimateRawFee, EstimateRawFeeError> {
        let short = self.short.map(|d| d.into_model()).transpose()?;
        let medium = self.medium.map(|d| d.into_model()).transpose()?;
        let long = self.long.into_model()?;

        Ok(model::EstimateRawFee { short, medium, long })
    }
}

impl RawFeeDetail {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::RawFeeDetail, EstimateRawFeeError> {
        use EstimateRawFeeError as E;

        let fee_rate =
            self.fee_rate.map(crate::btc_per_kb).transpose().map_err(E::FeeRate)?.flatten();
        let pass = self.pass.map(|p| p.into_model()).transpose()?;
        let fail = self.fail.map(|p| p.into_model()).transpose()?;

        Ok(model::RawFeeDetail {
            fee_rate,
            decay: self.decay,
            scale: self.scale,
            pass,
            fail,
            errors: self.errors,
        })
    }
}

impl RawFeeRange {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::RawFeeRange, EstimateRawFeeError> {
        let start_range = crate::btc_per_kb(self.start_range).ok().flatten();
        let end_range = crate::btc_per_kb(self.end_range).ok().flatten();

        Ok(model::RawFeeRange {
            start_range,
            end_range,
            within_target: self.within_target,
            total_confirmed: self.total_confirmed,
            in_mempool: self.in_mempool,
            left_mempool: self.left_mempool,
        })
    }
}

impl WaitForBlock {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::WaitForBlock, WaitForBlockError> {
        use WaitForBlockError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;

        Ok(model::WaitForBlock { hash, height: crate::to_u32(self.height, "height")? })
    }
}

impl WaitForBlockHeight {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::WaitForBlockHeight, WaitForBlockHeightError> {
        use WaitForBlockHeightError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;

        Ok(model::WaitForBlockHeight { hash, height: crate::to_u32(self.height, "height")? })
    }
}

impl WaitForNewBlock {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::WaitForNewBlock, WaitForNewBlockError> {
        use WaitForNewBlockError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;

        Ok(model::WaitForNewBlock { hash, height: crate::to_u32(self.height, "height")? })
    }
}
