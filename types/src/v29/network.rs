// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v28` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.
use serde::{Deserialize, Serialize};

use super::{GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork};
use crate::model;

/// Result of the JSON-RPC method `getnetworkinfo`.
///
/// > getnetworkinfo
///
/// > Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfo {
    /// The server version.
    pub version: usize,
    /// The server subversion string.
    pub subversion: String,
    /// The protocol version.
    #[serde(rename = "protocolversion")]
    pub protocol_version: usize,
    /// The services we offer to the network (hex string).
    #[serde(rename = "localservices")]
    pub local_services: String,
    /// `true` if transaction relay is requested from peers.
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// The time offset.
    #[serde(rename = "timeoffset")]
    pub time_offset: isize,
    /// The total number of connections.
    pub connections: usize,
    /// Whether p2p networking is enabled.
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// Information per network.
    pub networks: Vec<GetNetworkInfoNetwork>,
    /// Minimum relay fee rate for transactions in BTC/kB.
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kB.
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    /// List of local addresses.
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<GetNetworkInfoAddress>,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}

impl GetNetworkInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetNetworkInfo, GetNetworkInfoError> {
        use GetNetworkInfoError as E;

        let relay_fee = crate::btc_per_kb(self.relay_fee).map_err(E::RelayFee)?;
        let incremental_fee = crate::btc_per_kb(self.incremental_fee).map_err(E::IncrementalFee)?;

        Ok(model::GetNetworkInfo {
            version: self.version,
            subversion: self.subversion,
            protocol_version: self.protocol_version,
            local_services: self.local_services,
            local_relay: self.local_relay,
            time_offset: self.time_offset,
            connections: self.connections,
            network_active: self.network_active,
            networks: self.networks.into_iter().map(|n| n.into_model()).collect(),
            relay_fee,
            incremental_fee,
            local_addresses: self.local_addresses.into_iter().map(|a| a.into_model()).collect(),
            warnings: self.warnings,
        })
    }
}
