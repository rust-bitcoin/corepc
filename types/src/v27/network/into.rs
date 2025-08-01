// SPDX-License-Identifier: CC0-1.0

use bitcoin::address::NetworkUnchecked;
use bitcoin::{address, Address};

use super::{GetNodeAddresses, NodeAddress};
use crate::model;

impl GetNodeAddresses {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetNodeAddresses, address::ParseError> {
        let nodes = self.0.into_iter().map(|n| n.into_model()).collect::<Result<_, _>>()?;
        Ok(model::GetNodeAddresses(nodes))
    }
}

impl NodeAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::NodeAddress, address::ParseError> {
        let address = self.address.parse::<Address<NetworkUnchecked>>()?;
        Ok(model::NodeAddress {
            time: self.time,
            services: self.services,
            address,
            port: self.port,
            network: Some(self.network),
        })
    }
}
