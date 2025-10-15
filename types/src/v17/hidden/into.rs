// SPDX-License-Identifier: CC0-1.0

use bitcoin::BlockHash;

use super::{
    WaitForBlock, WaitForBlockError, WaitForBlockHeight, WaitForBlockHeightError, WaitForNewBlock,
    WaitForNewBlockError,
};
use crate::model;

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
