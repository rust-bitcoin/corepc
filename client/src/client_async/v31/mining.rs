// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method wrappers for Bitcoin Core `31` - mining.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Defines the `*Options` request structs these methods
//! consume; the response types live in the `corepc-types` crate
//! (`types::v31::generated`).

#![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use serde_json::json;
use types::v31::generated::{
    GetBlockTemplate, GetMiningInfo, GetNetworkHashPs, GetPrioritisedTransactions,
    PrioritiseTransaction, SubmitBlock,
};

use crate::client_async::error::Result;
use crate::client_async::Client;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTemplateTemplateRequest {
    /// A list of strings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    /// proposed block data to check, encoded in hexadecimal; valid only for mode="proposal"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// delay processing request until the result would vary significantly from the "longpollid" of a prior template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longpollid: Option<String>,
    /// This must be set to "template", "proposal" (see BIP 23), or omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// A list of strings
    pub rules: Vec<String>,
}

/// Optional parameters for the `getnetworkhashps` JSON-RPC method (consumed by `Client::get_network_hash_ps_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkHashPsOptions {
    /// The number of previous blocks to calculate estimate from, or -1 for blocks since last difficulty change.
    pub n_blocks: Option<i64>,
    /// To estimate at the time of the given height.
    pub height: Option<i64>,
}

/// Optional parameters for the `prioritisetransaction` JSON-RPC method (consumed by `Client::prioritise_transaction_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrioritiseTransactionOptions {
    /// API-Compatibility for previous API. Must be zero or null.
    ///                   DEPRECATED. For forward compatibility use named arguments and omit this parameter.
    pub dummy: Option<f64>,
}

/// Optional parameters for the `submitblock` JSON-RPC method (consumed by `Client::submit_block_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBlockOptions {
    /// dummy value, for compatibility with BIP22. This value is ignored.
    pub dummy: Option<String>,
}

impl Client {
    /// `getblocktemplate` with required arguments only.
    ///
    /// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
    /// It returns data needed to construct a block to work on.
    /// For full specification, see BIPs 22, 23, 9, and 145:
    ///     <https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki>
    ///     <https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki>
    ///     <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes>
    ///     <https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki>
    pub async fn get_block_template(
        &self,
        template_request: GetBlockTemplateTemplateRequest,
    ) -> Result<GetBlockTemplate> {
        self.call_raw("getblocktemplate", &[json!(template_request)]).await
    }

    /// `getmininginfo` with required arguments only.
    ///
    /// Returns a json object containing mining-related information.
    pub async fn get_mining_info(&self) -> Result<GetMiningInfo> {
        self.call_raw("getmininginfo", &[(); 0] as &[()]).await
    }

    /// `getnetworkhashps` with required arguments only.
    ///
    /// Returns the estimated network hashes per second based on the last n blocks.
    /// Pass in \[blocks\] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in \[height\] to estimate the network speed at the time when a certain block was found.
    pub async fn get_network_hash_ps(&self) -> Result<GetNetworkHashPs> {
        self.call_raw("getnetworkhashps", &[(); 0] as &[()]).await
    }

    /// `getnetworkhashps` with all optional arguments via [`GetNetworkHashPsOptions`].
    ///
    /// Returns the estimated network hashes per second based on the last n blocks.
    /// Pass in \[blocks\] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in \[height\] to estimate the network speed at the time when a certain block was found.
    pub async fn get_network_hash_ps_with(
        &self,
        opts: GetNetworkHashPsOptions,
    ) -> Result<GetNetworkHashPs> {
        self.call_raw("getnetworkhashps", &[json!(opts.n_blocks), json!(opts.height)]).await
    }

    /// `getprioritisedtransactions` with required arguments only.
    ///
    /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    pub async fn get_prioritised_transactions(&self) -> Result<GetPrioritisedTransactions> {
        self.call_raw("getprioritisedtransactions", &[(); 0] as &[()]).await
    }

    /// `prioritisetransaction` with required arguments only.
    ///
    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    pub async fn prioritise_transaction(
        &self,
        txid: String,
        fee_delta: i64,
    ) -> Result<PrioritiseTransaction> {
        self.call_raw("prioritisetransaction", &[json!(txid), json!(null), json!(fee_delta)]).await
    }

    /// `prioritisetransaction` with all optional arguments via [`PrioritiseTransactionOptions`].
    ///
    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    pub async fn prioritise_transaction_with(
        &self,
        txid: String,
        fee_delta: i64,
        opts: PrioritiseTransactionOptions,
    ) -> Result<PrioritiseTransaction> {
        self.call_raw("prioritisetransaction", &[json!(txid), json!(opts.dummy), json!(fee_delta)])
            .await
    }

    /// `submitblock` with required arguments only.
    ///
    /// Attempts to submit new block to network.
    /// See <https://en.bitcoin.it/wiki/BIP_0022> for full specification.
    pub async fn submit_block(&self, hex_data: String) -> Result<SubmitBlock> {
        self.call_raw("submitblock", &[json!(hex_data)]).await
    }

    /// `submitblock` with all optional arguments via [`SubmitBlockOptions`].
    ///
    /// Attempts to submit new block to network.
    /// See <https://en.bitcoin.it/wiki/BIP_0022> for full specification.
    pub async fn submit_block_with(
        &self,
        hex_data: String,
        opts: SubmitBlockOptions,
    ) -> Result<SubmitBlock> {
        self.call_raw("submitblock", &[json!(hex_data), json!(opts.dummy)]).await
    }

    /// `submitheader` with required arguments only.
    ///
    /// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
    /// Throws when the header is invalid.
    pub async fn submit_header(&self, hex_data: String) -> Result<()> {
        self.call_raw("submitheader", &[json!(hex_data)]).await
    }
}
