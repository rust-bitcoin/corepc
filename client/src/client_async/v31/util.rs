// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method wrappers for Bitcoin Core `31` - util.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Defines the `*Options` request structs these methods
//! consume; the response types live in the `corepc-types` crate
//! (`types::v31::generated`).

#![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use serde_json::json;
use types::v31::generated::{
    CreateMultisig, DeriveAddresses, EstimateSmartFee, GetDescriptorInfo, GetIndexInfo,
    SignMessageWithPrivKey, ValidateAddress, VerifyMessage,
};

use crate::client_async::error::Result;
use crate::client_async::Client;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DeriveAddressesRange {
    Number(f64),
    List(Vec<serde_json::Value>),
}

/// Optional parameters for the `createmultisig` JSON-RPC method (consumed by `Client::create_multisig_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMultisigOptions {
    /// The address type to use. Options are "legacy", "p2sh-segwit", and "bech32".
    pub address_type: Option<String>,
}

/// Optional parameters for the `deriveaddresses` JSON-RPC method (consumed by `Client::derive_addresses_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeriveAddressesOptions {
    /// If a ranged descriptor is used, this specifies the end or the range (in \[begin,end\] notation) to derive.
    pub range: Option<DeriveAddressesRange>,
}

/// Optional parameters for the `estimatesmartfee` JSON-RPC method (consumed by `Client::estimate_smart_fee_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateSmartFeeOptions {
    /// The fee estimate mode.
    /// unset, economical, conservative
    /// unset means no mode set (default mode will be used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
}

/// Optional parameters for the `getindexinfo` JSON-RPC method (consumed by `Client::get_index_info_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexInfoOptions {
    /// Filter results for an index with a specific name.
    pub index_name: Option<String>,
}

impl Client {
    /// `createmultisig` with required arguments only.
    ///
    /// Creates a multi-signature address with n signatures of m keys required.
    /// It returns a json object with the address and redeemScript.
    pub async fn create_multisig(
        &self,
        n_required: i64,
        keys: Vec<String>,
    ) -> Result<CreateMultisig> {
        self.call_raw("createmultisig", &[json!(n_required), json!(keys)]).await
    }

    /// `createmultisig` with all optional arguments via [`CreateMultisigOptions`].
    ///
    /// Creates a multi-signature address with n signatures of m keys required.
    /// It returns a json object with the address and redeemScript.
    pub async fn create_multisig_with(
        &self,
        n_required: i64,
        keys: Vec<String>,
        opts: CreateMultisigOptions,
    ) -> Result<CreateMultisig> {
        self.call_raw("createmultisig", &[json!(n_required), json!(keys), json!(opts.address_type)])
            .await
    }

    /// `deriveaddresses` with required arguments only.
    ///
    /// Derives one or more addresses corresponding to an output descriptor.
    /// Examples of output descriptors are:
    ///     pkh(\<pubkey\>)                                     P2PKH outputs for the given pubkey
    ///     wpkh(\<pubkey\>)                                    Native segwit P2PKH outputs for the given pubkey
    ///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...))              P2SH-multisig outputs for the given threshold and pubkeys
    ///     raw(\<hex script\>)                                 Outputs whose output script equals the specified hex-encoded bytes
    ///     tr(\<pubkey\>,multi_a(\<n\>,\<pubkey\>,\<pubkey\>,...))   P2TR-multisig outputs for the given threshold and pubkeys
    ///
    /// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
    /// or more path elements separated by "/", where "h" represents a hardened child key.
    /// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
    pub async fn derive_addresses(&self, descriptor: String) -> Result<DeriveAddresses> {
        self.call_raw("deriveaddresses", &[json!(descriptor)]).await
    }

    /// `deriveaddresses` with all optional arguments via [`DeriveAddressesOptions`].
    ///
    /// Derives one or more addresses corresponding to an output descriptor.
    /// Examples of output descriptors are:
    ///     pkh(\<pubkey\>)                                     P2PKH outputs for the given pubkey
    ///     wpkh(\<pubkey\>)                                    Native segwit P2PKH outputs for the given pubkey
    ///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...))              P2SH-multisig outputs for the given threshold and pubkeys
    ///     raw(\<hex script\>)                                 Outputs whose output script equals the specified hex-encoded bytes
    ///     tr(\<pubkey\>,multi_a(\<n\>,\<pubkey\>,\<pubkey\>,...))   P2TR-multisig outputs for the given threshold and pubkeys
    ///
    /// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
    /// or more path elements separated by "/", where "h" represents a hardened child key.
    /// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
    pub async fn derive_addresses_with(
        &self,
        descriptor: String,
        opts: DeriveAddressesOptions,
    ) -> Result<DeriveAddresses> {
        self.call_raw("deriveaddresses", &[json!(descriptor), json!(opts.range)]).await
    }

    /// `estimatesmartfee` with required arguments only.
    ///
    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible and return the number of blocks
    /// for which the estimate is valid. Uses virtual transaction size as defined
    /// in BIP 141 (witness data is discounted).
    pub async fn estimate_smart_fee(&self, conf_target: i64) -> Result<EstimateSmartFee> {
        self.call_raw("estimatesmartfee", &[json!(conf_target)]).await
    }

    /// `estimatesmartfee` with all optional arguments via [`EstimateSmartFeeOptions`].
    ///
    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible and return the number of blocks
    /// for which the estimate is valid. Uses virtual transaction size as defined
    /// in BIP 141 (witness data is discounted).
    pub async fn estimate_smart_fee_with(
        &self,
        conf_target: i64,
        opts: EstimateSmartFeeOptions,
    ) -> Result<EstimateSmartFee> {
        self.call_raw("estimatesmartfee", &[json!(conf_target), json!(opts.estimate_mode)]).await
    }

    /// `getdescriptorinfo` with required arguments only.
    ///
    /// Analyses a descriptor.
    pub async fn get_descriptor_info(&self, descriptor: String) -> Result<GetDescriptorInfo> {
        self.call_raw("getdescriptorinfo", &[json!(descriptor)]).await
    }

    /// `getindexinfo` with required arguments only.
    ///
    /// Returns the status of one or all available indices currently running in the node.
    pub async fn get_index_info(&self) -> Result<GetIndexInfo> {
        self.call_raw("getindexinfo", &[(); 0] as &[()]).await
    }

    /// `getindexinfo` with all optional arguments via [`GetIndexInfoOptions`].
    ///
    /// Returns the status of one or all available indices currently running in the node.
    pub async fn get_index_info_with(&self, opts: GetIndexInfoOptions) -> Result<GetIndexInfo> {
        self.call_raw("getindexinfo", &[json!(opts.index_name)]).await
    }

    /// `signmessagewithprivkey` with required arguments only.
    ///
    /// Sign a message with the private key of an address
    pub async fn sign_message_with_priv_key(
        &self,
        priv_key: String,
        message: String,
    ) -> Result<SignMessageWithPrivKey> {
        self.call_raw("signmessagewithprivkey", &[json!(priv_key), json!(message)]).await
    }

    /// `validateaddress` with required arguments only.
    ///
    /// Return information about the given bitcoin address.
    pub async fn validate_address(&self, address: String) -> Result<ValidateAddress> {
        self.call_raw("validateaddress", &[json!(address)]).await
    }

    /// `verifymessage` with required arguments only.
    ///
    /// Verify a signed message.
    pub async fn verify_message(
        &self,
        address: String,
        signature: String,
        message: String,
    ) -> Result<VerifyMessage> {
        self.call_raw("verifymessage", &[json!(address), json!(signature), json!(message)]).await
    }
}
