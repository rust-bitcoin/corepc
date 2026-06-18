// SPDX-License-Identifier: CC0-1.0

//! Auto-generated types for Bitcoin Core `31` - signer.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Holds the RPC return types for this section; the
//! `*Options` request structs live with the call surface in `corepc-client`.

#![allow(non_camel_case_types, non_snake_case, clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

/// Returns a list of external signers from -signer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSigners {
    pub signers: Vec<EnumerateSignersSignersItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSignersSignersItem {
    /// Master key fingerprint
    pub fingerprint: String,
    /// Device name
    pub name: String,
}
