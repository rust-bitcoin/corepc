// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v25` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `createwallet`.
///
/// > createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )
///
/// > Creates and loads a new wallet.
///
/// > Arguments:
/// > 1. wallet_name             (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
/// > 2. disable_private_keys    (boolean, optional, default=false) Disable the possibility of private keys (only watchonlys are possible in this mode).
/// > 3. blank                   (boolean, optional, default=false) Create a blank wallet. A blank wallet has no keys or HD seed. One can be set using sethdseed.
/// > 4. passphrase              (string, optional) Encrypt the wallet with this passphrase.
/// > 5. avoid_reuse             (boolean, optional, default=false) Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
/// > 6. descriptors             (boolean, optional, default=true) Create a native descriptor wallet. The wallet will use descriptors internally to handle address creation. Setting to "false" will create a legacy wallet; however, the legacy wallet type is being deprecated and support for creating and opening legacy wallets will be removed in the future.
/// > 7. load_on_startup         (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
/// > 8. external_signer         (boolean, optional, default=false) Use an external signer such as a hardware wallet. Requires -signer to be configured. Wallet creation will fail if keys cannot be fetched. Requires disable_private_keys and descriptors set to true.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWallet {
    /// The wallet name if created successfully.
    ///
    /// If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating the wallet. Multiple messages will be delimited by newlines.
    ///
    /// DEPRECATED, returned only if config option -deprecatedrpc=walletwarningfield is passed. As
    /// the content would still be the same as `warnings`, we simply ignore the field.
    pub warning: Option<String>,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Result of JSON-RPC method `listdescriptors`.
///
/// > List descriptors imported into a descriptor-enabled wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListDescriptors {
    /// Name of wallet this operation was performed on.
    pub wallet_name: String,
    /// Array of descriptor objects.
    pub descriptors: Vec<DescriptorInfo>,
}

/// A descriptor object. Part of `listdescriptors`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
    /// Same as `next_index` field. Kept for compatibility reason.
    pub next: Option<u64>,
    /// The next index to generate addresses from; defined only for ranged descriptors.
    pub next_index: Option<u64>,
}

/// Result of the JSON-RPC method `loadwallet`.
///
/// > loadwallet "filename" ( load_on_startup )
///
/// > Loads a wallet from a wallet file or directory.
/// > Note that all wallet command-line options used when starting bitcoind will be
/// > applied to the new wallet.
///
/// > Arguments:
/// > 1. filename           (string, required) The wallet directory or .dat file.
/// > 2. load_on_startup    (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet. Multiple messages will be delimited by newlines.
    ///
    /// DEPRECATED, returned only if config option -deprecatedrpc=walletwarningfield is passed. As
    /// the content would still be the same as `warnings`, we simply ignore the field.
    pub warning: Option<String>,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `unloadwallet`.
///
/// > unloadwallet ( "wallet_name" load_on_startup )
/// >
/// > Unloads the wallet referenced by the request endpoint otherwise unloads the wallet specified in the argument.
/// > Specifying the wallet name on a wallet endpoint is invalid.
/// >
/// > Arguments:
/// > 1. wallet_name        (string, optional, default=the wallet name from the RPC endpoint) The name of the wallet to unload. If provided both here and in the RPC endpoint, the two must be identical.
/// > 2. load_on_startup    (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UnloadWallet {
    /// Warning messages, if any, related to unloading the wallet. Multiple messages will be delimited by newlines.
    ///
    /// DEPRECATED, returned only if config option -deprecatedrpc=walletwarningfield is passed. As
    /// the content would still be the same as `warnings`, we simply ignore the field.
    pub warning: Option<String>,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}
