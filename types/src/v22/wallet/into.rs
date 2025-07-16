// SPDX-License-Identifier: CC0-1.0

use bitcoin::{address, Address};

use super::WalletDisplayAddress;
use crate::model;

impl WalletDisplayAddress {
    pub fn into_model(self) -> Result<model::WalletDisplayAddress, address::ParseError> {
        let address = self.address.parse::<Address<_>>()?;
        Ok(model::WalletDisplayAddress { address })
    }
}
