// SPDX-License-Identifier: CC0-1.0

use super::{GetWalletInfo, GetWalletInfoError, GetWalletInfoScanning};
use crate::model;

impl GetWalletInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetWalletInfo, GetWalletInfoError> {
        use GetWalletInfoError as E;

        let wallet_version = crate::to_u32(self.wallet_version, "wallet_version")?;
        let tx_count = crate::to_u32(self.tx_count, "tx_count")?;
        let keypool_size = crate::to_u32(self.keypool_size, "keypool_size")?;
        let keypool_size_hd_internal = self
            .keypool_size_hd_internal
            .map(|v| crate::to_u32(v, "keypool_size_hd_internal"))
            .transpose()?
            .unwrap_or(0);
        let last_processed_block = self
            .last_processed_block
            .map(|l| l.into_model())
            .transpose()
            .map_err(E::LastProcessedBlock)?;

        let scanning = match self.scanning {
            GetWalletInfoScanning::Details { duration, progress } =>
                Some(model::GetWalletInfoScanning::Details { duration, progress }),
            GetWalletInfoScanning::NotScanning(b) =>
                Some(model::GetWalletInfoScanning::NotScanning(b)),
        };

        Ok(model::GetWalletInfo {
            wallet_name: self.wallet_name,
            wallet_version,
            format: Some(self.format),
            balance: None,
            unconfirmed_balance: None,
            immature_balance: None,
            tx_count,
            keypool_oldest: None,
            keypool_size,
            keypool_size_hd_internal,
            unlocked_until: self.unlocked_until,
            pay_tx_fee: None,
            hd_seed_id: None,
            private_keys_enabled: self.private_keys_enabled,
            avoid_reuse: Some(self.avoid_reuse),
            scanning,
            descriptors: Some(self.descriptors),
            external_signer: Some(self.external_signer),
            blank: Some(self.blank),
            birthtime: self.birthtime,
            flags: Some(self.flags),
            last_processed_block,
        })
    }
}
