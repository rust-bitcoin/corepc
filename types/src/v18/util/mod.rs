// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - util.
//!
//! Types for methods found under the `== Util ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `deriveaddresses`.
///
/// > deriveaddresses "descriptor" ( range )
/// >
/// > Derives one or more addresses corresponding to an output descriptor.
/// > Returns an array of derived addresses.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DeriveAddresses(pub Vec<String>);

/// Result of JSON-RPC method `getdescriptorinfo`.
///
/// > getdescriptorinfo "descriptor"
/// >
/// > Analyses a descriptor.
/// > Returns information about the descriptor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorInfo {
    /// The descriptor in canonical form, without private keys.
    pub descriptor: String,
    /// Whether the descriptor is ranged.
    #[serde(rename = "isrange")]
    pub is_range: bool,
    /// Whether the descriptor is solvable.
    #[serde(rename = "issolvable")]
    pub is_solvable: bool,
    /// Whether the input descriptor contained at least one private key.
    #[serde(rename = "hasprivatekeys")]
    pub has_private_keys: bool,
}
