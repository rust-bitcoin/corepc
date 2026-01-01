// SPDX-License-Identifier: CC0-1.0

use bitcoin::consensus::encode;
use bitcoin::hashes::hex::FromHex;
use bitcoin::{Transaction, Txid, Wtxid};

use super::{
    GetOrphanTxs, GetOrphanTxsVerboseOne, GetOrphanTxsVerboseOneEntry,
    GetOrphanTxsVerboseOneEntryError, GetOrphanTxsVerboseTwo, GetOrphanTxsVerboseTwoEntry,
    GetOrphanTxsVerboseTwoEntryError,
};
use crate::model;

impl GetOrphanTxs {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::GetOrphanTxs { model::GetOrphanTxs(self.0) }
}

impl GetOrphanTxsVerboseOneEntry {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::GetOrphanTxsVerboseOneEntry, GetOrphanTxsVerboseOneEntryError> {
        use GetOrphanTxsVerboseOneEntryError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wtxid = self.wtxid.parse::<Wtxid>().map_err(E::Wtxid)?;

        Ok(model::GetOrphanTxsVerboseOneEntry {
            txid,
            wtxid,
            bytes: self.bytes,
            vsize: self.vsize,
            weight: self.weight,
            from: self.from,
            entry_time: None,
            expiration_time: None,
        })
    }
}

impl GetOrphanTxsVerboseTwoEntry {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::GetOrphanTxsVerboseTwoEntry, GetOrphanTxsVerboseTwoEntryError> {
        use GetOrphanTxsVerboseTwoEntryError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wtxid = self.wtxid.parse::<Wtxid>().map_err(E::Wtxid)?;
        let v = Vec::from_hex(&self.hex).map_err(E::Hex)?;
        let transaction = encode::deserialize::<Transaction>(&v).map_err(E::Consensus)?;

        Ok(model::GetOrphanTxsVerboseTwoEntry {
            txid,
            wtxid,
            bytes: self.bytes,
            vsize: self.vsize,
            weight: self.weight,
            from: self.from,
            entry_time: None,
            expiration_time: None,
            transaction,
        })
    }
}

impl GetOrphanTxsVerboseOne {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::GetOrphanTxsVerboseOne, GetOrphanTxsVerboseOneEntryError> {
        let v = self.0.into_iter().map(|e| e.into_model()).collect::<Result<Vec<_>, _>>()?;

        Ok(model::GetOrphanTxsVerboseOne(v))
    }
}

impl GetOrphanTxsVerboseTwo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::GetOrphanTxsVerboseTwo, GetOrphanTxsVerboseTwoEntryError> {
        let v = self.0.into_iter().map(|e| e.into_model()).collect::<Result<Vec<_>, _>>()?;

        Ok(model::GetOrphanTxsVerboseTwo(v))
    }
}
