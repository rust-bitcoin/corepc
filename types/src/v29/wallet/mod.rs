// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v29` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Arguments of JSON-RPC method `createwallet`.
///
/// # Note
///
/// This can also be used for the `loadwallet` JSON-RPC method.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateWalletArguments {
    /// Wallet name
    pub name: String,
    /// Load on startup
    pub load_on_startup: Option<bool>,
}

/// Inputs of JSON-RPC method `importdescriptors`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ImportDescriptorInput {
    /// The descriptor.
    pub desc: String,
    /// Set this descriptor to be the active descriptor
    /// for the corresponding output type/externality.
    pub active: Option<bool>,
    /// Time from which to start rescanning the blockchain for this descriptor,
    /// in UNIX epoch time. Can also be a string "now"
    pub timestamp: String,
}

/// Query options for filtering unspent transaction outputs.
///
/// Used with `list_unspent` to apply additional filtering criteria
/// beyond confirmation counts and addresses, allowing precise UTXO selection
/// based on amount ranges and result limits.
///
/// # Note
///
/// All fields are optional and can be combined. UTXOs must satisfy all
/// specified criteria to be included in the results.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUnspentQueryOptions {
    /// Minimum amount that UTXOs must have to be included.
    ///
    /// Only unspent outputs with a value greater than or equal to this amount
    /// will be returned. Useful for filtering out dust or very small UTXOs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<f64>,

    /// Maximum amount that UTXOs can have to be included.
    ///
    /// Only unspent outputs with a value less than or equal to this amount
    /// will be returned. Useful for finding smaller UTXOs or avoiding large ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<f64>,

    /// Maximum number of UTXOs to return in the result set.
    ///
    /// Limits the total number of unspent outputs returned, regardless of how many
    /// match the other criteria. Useful for pagination or limiting response size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_count: Option<u32>,
}

/// Optional previous transaction outputs argument for the method `signrawtransactionwithwallet`.
///
/// These are the outputs that this transaction depends on but may not yet be in the block chain.
/// Widely used for One Parent One Child (1P1C) Relay in Bitcoin >=28.0.
///
/// > transaction outputs
/// > [
/// > {                            (json object)
/// > "txid": "hex",             (string, required) The transaction id
/// > "vout": n,                 (numeric, required) The output number
/// > "scriptPubKey": "hex",     (string, required) The output script
/// > "redeemScript": "hex",     (string, optional) (required for P2SH) redeem script
/// > "witnessScript": "hex",    (string, optional) (required for P2WSH or P2SH-P2WSH) witness
/// > script
/// > "amount": amount,          (numeric or string, optional) (required for Segwit inputs) the
/// > amount spent
/// > },
/// > ...
/// > ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PreviousTransactionOutput {
    /// The transaction id.
    pub txid: String,
    /// The output number.
    pub vout: u32,
    /// The output script.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// The redeem script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<String>,
    /// The witness script.
    #[serde(rename = "witnessScript")]
    pub witness_script: Option<String>,
    /// The amount spent.
    pub amount: Option<f64>,
}

/// Options for `psbtbumpfee` RPC method.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct PsbtBumpFeeOptions {
    /// Confirmation target in blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conf_target: Option<u16>,

    /// Fee rate in sat/vB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,

    /// Whether the new transaction should be BIP-125 replaceable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,

    /// Fee estimate mode ("unset", "economical", "conservative").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_mode: Option<String>,

    /// New transaction outputs to replace the existing ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<super::CreateRawTransactionOutput>>,

    /// Index of the change output to recycle from the original transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_change_index: Option<u32>,
}

/// Options for creating a funded PSBT with wallet inputs.
///
/// Used with `wallet_create_funded_psbt` to control funding behavior,
/// fee estimation, and transaction policies when the wallet automatically
/// selects inputs to fund the specified outputs.
///
/// # Note
///
/// All fields are optional and will use Bitcoin Core defaults if not specified.
/// Fee rate takes precedence over confirmation target if both are provided.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct WalletCreateFundedPsbtOptions {
    /// Fee rate in sat/vB (satoshis per virtual byte) for the transaction.
    ///
    /// If specified, this overrides the `conf_target` parameter for fee estimation.
    /// Must be a positive value representing the desired fee density.
    #[serde(default, rename = "fee_rate", skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,

    /// Whether to lock the selected UTXOs to prevent them from being spent by other transactions.
    ///
    /// When `true`, the wallet will temporarily lock the selected unspent outputs
    /// until the transaction is broadcast or manually unlocked. Default is `false`.
    #[serde(default, rename = "lockUnspents", skip_serializing_if = "Option::is_none")]
    pub lock_unspents: Option<bool>,

    /// Target number of confirmations for automatic fee estimation.
    ///
    /// Represents the desired number of blocks within which the transaction should
    /// be confirmed. Higher values result in lower fees but longer confirmation times.
    /// Ignored if `fee_rate` is specified.
    #[serde(default, rename = "conf_target", skip_serializing_if = "Option::is_none")]
    pub conf_target: Option<u16>,

    /// Whether the transaction should be BIP-125 opt-in Replace-By-Fee (RBF) enabled.
    ///
    /// When `true`, allows the transaction to be replaced with a higher-fee version
    /// before confirmation. Useful for fee bumping if the initial fee proves insufficient.
    #[serde(default, rename = "replaceable", skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,
}
