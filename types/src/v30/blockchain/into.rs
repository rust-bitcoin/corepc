// SPDX-License-Identifier: CC0-1.0

use super::{GetMempoolInfo, GetMempoolInfoError};
use crate::model;

impl GetMempoolInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolInfo, GetMempoolInfoError> {
        let size = crate::to_u32(self.size, "size")?;
        let bytes = crate::to_u32(self.bytes, "bytes")?;
        let usage = crate::to_u32(self.usage, "usage")?;
        let max_mempool = crate::to_u32(self.max_mempool, "max_mempool")?;
        let mempool_min_fee = crate::btc_per_kb(self.mempool_min_fee)?;
        let min_relay_tx_fee = crate::btc_per_kb(self.min_relay_tx_fee)?;
        let incremental_relay_fee = crate::btc_per_kb(self.incremental_relay_fee)?;
        let unbroadcast_count = Some(crate::to_u32(self.unbroadcast_count, "unbroadcast_count")?);

        Ok(model::GetMempoolInfo {
            loaded: Some(self.loaded),
            size,
            bytes,
            usage,
            total_fee: Some(self.total_fee),
            max_mempool,
            mempool_min_fee,
            min_relay_tx_fee,
            incremental_relay_fee: Some(incremental_relay_fee),
            unbroadcast_count,
            full_rbf: Some(self.full_rbf),
            permit_bare_multisig: Some(self.permit_bare_multisig),
            max_data_carrier_size: Some(self.max_data_carrier_size),
        })
    }
}
