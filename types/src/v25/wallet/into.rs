// SPDX-License-Identifier: CC0-1.0

use super::{CreateWallet, LoadWallet, UnloadWallet};
use crate::model;

impl CreateWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::CreateWallet {
        // As the content of the deprecated `warning` field would be the same as `warnings`, we
        // simply ignore the field, even in case it's set.
        model::CreateWallet { name: self.name, warnings: self.warnings.unwrap_or_default() }
    }

    /// Returns the created wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

impl LoadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::LoadWallet {
        // As the content of the deprecated `warning` field would be the same as `warnings`, we
        // simply ignore the field, even in case it's set.
        model::LoadWallet { name: self.name, warnings: self.warnings.unwrap_or_default() }
    }

    /// Returns the loaded wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

impl UnloadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::UnloadWallet {
        model::UnloadWallet { warnings: self.warnings.unwrap_or_default() }
    }
}
