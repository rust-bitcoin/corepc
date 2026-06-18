// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method wrappers for Bitcoin Core `31` - signer.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Defines the `*Options` request structs these methods
//! consume; the response types live in the `corepc-types` crate
//! (`types::v31::generated`).

#![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use serde_json::json;
use types::v31::generated::EnumerateSigners;

use crate::client_async::error::Result;
use crate::client_async::Client;

impl Client {
    /// `enumeratesigners` with required arguments only.
    ///
    /// Returns a list of external signers from -signer.
    pub async fn enumerate_signers(&self) -> Result<EnumerateSigners> {
        self.call_raw("enumeratesigners", &[(); 0] as &[()]).await
    }
}
