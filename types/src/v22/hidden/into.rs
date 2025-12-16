// SPDX-License-Identifier: CC0-1.0

use super::AddConnection;
use crate::model;

impl AddConnection {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::AddConnection {
        model::AddConnection { address: self.address, connection_type: self.connection_type }
    }
}
