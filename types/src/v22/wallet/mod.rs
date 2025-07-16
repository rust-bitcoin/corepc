// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v22` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `listdescriptors`.
///
/// > List descriptors imported into a descriptor-enabled wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ListDescriptors {
    /// Name of wallet this operation was performed on.
    pub wallet_name: String,
    /// Array of descriptor objects.
    pub descriptors: Vec<DescriptorInfo>,
}

/// A descriptor object from `listdescriptors`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DescriptorInfo {
    /// Descriptor string representation.
    #[serde(rename = "desc")]
    pub descriptor: String,
    /// The creation time of the descriptor.
    pub timestamp: u64,
    /// Activeness flag.
    pub active: bool,
    /// Whether this is an internal or external descriptor; defined only for active descriptors.
    pub internal: Option<bool>,
    /// Defined only for ranged descriptors.
    pub range: Option<[u64; 2]>,
    /// The next index to generate addresses from; defined only for ranged descriptors.
    pub next: Option<u64>,
}

/// Result of JSON-RPC method `walletdisplayaddress`.
///
/// > Display address on an external signer for verification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WalletDisplayAddress {
    /// The address as confirmed by the signer
    pub address: String,
}
