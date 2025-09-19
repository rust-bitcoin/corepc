// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod error;
mod into;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ScriptSig;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use self::error::{
    DecodePsbtError, DecodeScriptError, FundRawTransactionError, GetRawTransactionVerboseError,
    PsbtInputError, PsbtOutputError, SignFailError, SignRawTransactionError, FinalizePsbtError,
};
// Re-export types that appear in the public API of this module.
pub use crate::psbt::{
    Bip32Deriv, InputKeySource, PsbtScript, RawTransaction, RawTransactionInput,
    RawTransactionOutput, WitnessUtxo,
};

/// Result of JSON-RPC method `combinepsbt`.
///
/// > combinepsbt ["psbt",...]
/// >
/// > Combine multiple partially signed Bitcoin transactions into one transaction.
/// > Implements the Combiner role.
/// >
/// > Arguments:
/// > 1. "txs"                   (string) A json array of base64 strings of partially signed transactions
/// >     [
/// >       "psbt"             (string) A base64 string of a PSBT
/// >       ,...
/// >     ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CombinePsbt(
    /// The base64-encoded partially signed transaction.
    pub String,
);

/// Result of JSON-RPC method `combinerawtransaction`.
///
/// > combinerawtransaction ["hexstring",...]
/// >
/// > Combine multiple partially signed transactions into one transaction.
/// > The combined transaction may be another partially signed transaction or a
/// > fully signed transaction.
/// > Arguments:
/// > 1. "txs"         (string) A json array of hex strings of partially signed transactions
/// >     [
/// >       "hexstring"     (string) A transaction hash
/// >       ,...
/// >     ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CombineRawTransaction(
    /// The hex-encoded raw transaction with signature(s).
    pub String,
);

/// Result of JSON-RPC method `converttopsbt`.
///
/// > converttopsbt "hexstring" ( permitsigdata iswitness )
/// >
/// > Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// > createpsbt and walletcreatefundedpsbt should be used for new applications.
/// >
/// > Arguments:
/// > 1. "hexstring"              (string, required) The hex string of a raw transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ConvertToPsbt(
    /// The resulting raw transaction (base64-encoded string).
    pub String,
);

/// Result of JSON-RPC method `createpsbt`.
///
/// > createpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )
/// >
/// > Creates a transaction in the Partially Signed Transaction format.
/// > Implements the Creator role.
/// >
/// > Arguments:
/// > 1. "inputs"                (array, required) A json array of json objects
/// >      [
/// >        {
/// >          "txid":"id",      (string, required) The transaction id
/// >          "vout":n,         (numeric, required) The output number
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreatePsbt(
    /// The resulting raw transaction (base64-encoded string).
    pub String,
);

/// Result of JSON-RPC method `createrawtransaction`.
///
/// > createrawtransaction [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )
/// >
/// > Create a transaction spending the given inputs and creating new outputs.
/// > Outputs can be addresses or data.
/// > Returns hex-encoded raw transaction.
/// > Note that the transaction's inputs are not signed, and
/// > it is not stored in the wallet or transmitted to the network.
/// >
/// > Arguments:
/// > 1. "inputs"                (array, required) A json array of json objects
/// >      [
/// >        {
/// >          "txid":"id",      (string, required) The transaction id
/// >          "vout":n,         (numeric, required) The output number
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateRawTransaction(
    /// hex string of the transaction.
    pub String,
);

/// Result of JSON-RPC method `decodepsbt`.
///
/// > decodepsbt "psbt"
/// >
/// > Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
/// >
/// > Arguments:
/// > 1. "psbt"            (string, required) The PSBT base64 string
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbt {
    /// The decoded network-serialized unsigned transaction.
    pub tx: RawTransaction,
    /// The unknown global fields.
    pub unknown: Option<HashMap<String, String>>,
    /// Array of transaction inputs.
    pub inputs: Vec<PsbtInput>,
    /// Array of transaction outputs.
    pub outputs: Vec<PsbtOutput>,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    pub fee: Option<u64>,
}

/// An input in a partially signed Bitcoin transaction. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtInput {
    /// Decoded network transaction for non-witness UTXOs.
    pub non_witness_utxo: Option<RawTransaction>,
    /// Transaction output for witness UTXOs.
    pub witness_utxo: Option<WitnessUtxo>,
    /// The public key and signature that corresponds to it.
    pub partial_signatures: Option<HashMap<String, String>>,
    /// The sighash type to be used.
    pub sighash: Option<String>,
    /// The redeem script.
    pub redeem_script: Option<PsbtScript>,
    /// The witness script.
    pub witness_script: Option<PsbtScript>,
    /// The public key with the derivation path as the value.
    pub bip32_derivs: Option<HashMap<String, InputKeySource>>,
    /// The final scriptsig.
    #[serde(rename = "final_scriptsig")]
    pub final_script_sig: Option<ScriptSig>,
    /// Hex-encoded witness data (if any).
    #[serde(rename = "final_scriptwitness")]
    pub final_script_witness: Option<Vec<String>>,
    // `s/global/input`: this is a bug in the Core v0.17 docs.
    /// The unknown global fields.
    pub unknown: Option<HashMap<String, String>>,
}

/// An output in a partially signed Bitcoin transaction. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtOutput {
    /// The redeem script.
    pub redeem_script: Option<PsbtScript>,
    /// The witness script.
    pub witness_script: Option<PsbtScript>,
    /// The public key with the derivation path as the value.
    pub bip32_derivs: Option<Vec<Bip32Deriv>>,
    /// The unknown global fields.
    pub unknown: Option<HashMap<String, String>>,
}

/// Result of JSON-RPC method `decoderawtransaction`.
///
/// > decoderawtransaction "hexstring" ( iswitness )
/// >
/// > Return a JSON object representing the serialized, hex-encoded transaction.
/// >
/// > Arguments:
/// > 1. "hexstring"      (string, required) The transaction hex string
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransaction(pub RawTransaction);

/// Result of JSON-RPC method `decodescript`.
///
/// > decodescript "hexstring"
/// >
/// > Decode a hex-encoded script.
/// >
/// > Arguments:
/// > 1. "hexstring"     (string) the hex encoded script
// The docs on Core v0.17 appear to be way off what is actually returned.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScript {
    /// Script public key.
    pub asm: String,
    /// Hex encoded public key.
    pub hex: Option<String>,
    /// The output type.
    #[serde(rename = "type")]
    pub type_: String,
    /// The required signatures.
    #[serde(rename = "reqSigs")]
    pub required_signatures: Option<u64>,
    /// List of bitcoin addresses.
    pub addresses: Option<Vec<String>>,
    /// Address of P2SH script wrapping this redeem script (not returned if the script is already a P2SH).
    pub p2sh: Option<String>,
    /// Segwit data (see `DecodeScriptSegwit` for explanation).
    pub segwit: Option<DecodeScriptSegwit>,
    /// Address of the P2SH script wrapping this witness redeem script
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: Option<String>,
}

/// Segwit data. Part of `decodescript`.
//
/// Seemingly undocumented data returned in the `segwit` field of `DecodeScript`.
// This seems to be the same as `DecodeScript` except the `p2sh` field is called `p2sh-segwit`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScriptSegwit {
    /// Script public key.
    pub asm: String,
    /// Hex encoded public key.
    pub hex: String,
    /// The output type.
    #[serde(rename = "type")]
    pub type_: String,
    /// The required signatures.
    #[serde(rename = "reqSigs")]
    pub required_signatures: Option<u64>,
    /// List of bitcoin addresses.
    pub addresses: Option<Vec<String>>,
    /// Address of P2SH script wrapping this redeem script (not returned if the script is already a P2SH).
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: Option<String>,
}

/// Result of JSON-RPC method `finalizepsbt`.
///
/// > finalizepsbt "psbt" ( extract )
/// > Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// > network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// > created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// > Implements the Finalizer and Extractor roles.
/// >
/// > Arguments:
/// > 1. "psbt"                 (string) A base64 string of a PSBT
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FinalizePsbt {
    /// The base64-encoded partially signed transaction if not extracted.
    pub psbt: Option<String>,
    /// The hex-encoded network transaction if extracted.
    pub hex: Option<String>,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
}

/// Result of JSON-RPC method `fundrawtransaction`.
///
/// > fundrawtransaction "hexstring" ( options iswitness )
/// >
/// > Add inputs to a transaction until it has enough in value to meet its out value.
/// > This will not modify existing inputs, and will add at most one change output to the outputs.
/// > No existing outputs will be modified unless "subtractFeeFromOutputs" is specified.
/// > Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// > The inputs added will not be signed, use signrawtransaction for that.
/// > Note that all existing inputs must have their previous output transaction be in the wallet.
/// > Note that all inputs selected must be of standard form and P2SH scripts must be
/// > in the wallet using importaddress or addmultisigaddress (to calculate fees).
/// > You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// > Only pay-to-pubkey, multisig, and P2SH versions thereof are currently supported for watch-only
/// >
/// > Arguments:
/// > 1. "hexstring"           (string, required) The hex string of the raw transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundRawTransaction {
    /// The resulting raw transaction (hex-encoded string).
    pub hex: String,
    /// Fee in BTC the resulting transaction pays.
    pub fee: f64,
    /// The position of the added change output, or -1.
    #[serde(rename = "changepos")]
    pub change_position: i64,
}

/// Result of JSON-RPC method `getrawtransaction` with verbose set to `false`.
///
/// > getrawtransaction "txid" ( verbose "blockhash" )
/// >
/// > NOTE: By default this function only works for mempool transactions. If the -txindex option is
/// > enabled, it also works for blockchain transactions. If the block which contains the transaction
/// > is known, its hash can be provided even for nodes without -txindex. Note that if a blockhash is
/// > provided, only that block will be searched and if the transaction is in the mempool or other
/// > blocks, or if this node does not have the given block available, the transaction will not be found.
/// > DEPRECATED: for now, it also works for transactions with unspent outputs.
/// >
/// > Return the raw transaction data.
/// >
/// > If verbose is 'true', returns an Object with information about 'txid'.
/// > If verbose is 'false' or omitted, returns a string that is serialized, hex-encoded data for 'txid'.
/// >
/// > Arguments:
/// > 1. "txid"      (string, required) The transaction id
/// > 2. verbose     (bool, optional, default=false) If false, return a string, otherwise return a json object
/// > 3. "blockhash" (string, optional) The block in which to look for the transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransaction(
    /// The serialized, hex-encoded data for 'txid'.
    pub String,
);

/// Result of JSON-RPC method `getrawtransaction` with verbose set to `true`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose {
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument).
    pub in_active_chain: Option<bool>,
    /// The serialized, hex-encoded data for 'txid'.
    pub hex: String,
    /// The transaction id (same as provided).
    pub txid: String,
    /// The transaction hash (differs from txid for witness transactions).
    pub hash: String,
    /// The serialized transaction size.
    pub size: u64,
    /// The virtual transaction size (differs from size for witness transactions).
    pub vsize: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4).
    pub weight: u64,
    /// The version.
    pub version: i32,
    /// The lock time.
    #[serde(rename = "locktime")]
    pub lock_time: u32,
    /// Array of transaction inputs.
    #[serde(rename = "vin")]
    pub inputs: Vec<RawTransactionInput>,
    /// Array of transaction outputs.
    #[serde(rename = "vout")]
    pub outputs: Vec<RawTransactionOutput>,
    // The following fields are all `None` if the transaction is in the mempool.
    /// The block hash.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The confirmations.
    pub confirmations: Option<u64>,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "time")]
    pub transaction_time: Option<u64>,
    /// The block time in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: Option<u64>,
}

/// Result of JSON-RPC method `sendrawtransaction`.
///
/// > sendrawtransaction "hexstring" ( allowhighfees )
/// >
/// > Submits raw transaction (serialized, hex-encoded) to local node and network.
/// >
/// > Also see createrawtransaction and signrawtransactionwithkey calls.
/// >
/// > Arguments:
/// > 1. hexstring        (string, required) The hex string of the raw transaction
/// > 2. allowhighfees    (boolean, optional, default=false) Allow high fees
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendRawTransaction(
    /// The transaction hash in hex.
    pub String,
);

/// Result of JSON-RPC method `signrawtransaction`.
///
/// > signrawtransaction "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] ["privatekey1",...] sighashtype )
/// >
/// > DEPRECATED. Sign inputs for raw transaction (serialized, hex-encoded).
/// > The second optional argument (may be null) is an array of previous transaction outputs that
/// > this transaction depends on but may not yet be in the block chain.
/// > The third optional argument (may be null) is an array of base58-encoded private
/// > keys that, if given, will be the only keys used to sign the transaction.
/// >
/// >
/// > Arguments:
/// > 1. "hexstring"     (string, required) The transaction hex string
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransaction {
    /// The hex-encoded raw transaction with signature(s).
    pub hex: String,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
    /// Script verification errors (if there are any).
    pub errors: Option<Vec<SignFail>>,
}

/// Result of JSON-RPC method `signrawtransactionwithkey`.
///
/// > signrawtransactionwithkey "hexstring" ["privatekey1",...] ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )
/// >
/// > Sign inputs for raw transaction (serialized, hex-encoded).
/// > The second argument is an array of base58-encoded private
/// > keys that will be the only keys used to sign the transaction.
/// > The third optional argument (may be null) is an array of previous transaction outputs that
/// > this transaction depends on but may not yet be in the block chain.
/// >
/// > Arguments:
/// > 1. "hexstring"                      (string, required) The transaction hex string
/// > 2. "privkeys"                       (string, required) A json array of base58-encoded private keys for signing
///
/// **Note:** This is a type alias of [`SignRawTransaction`] because the RPC response
/// shape is identical, and our policy is to have a return type for every RPC method.
pub type SignRawTransactionWithKey = SignRawTransaction;

/// A script verification error. Part of `signrawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignFail {
    /// The hash of the referenced, previous transaction.
    pub txid: String,
    /// The index of the output to spent and used as input.
    pub vout: u64,
    /// The hex-encoded signature script.
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number.
    pub sequence: u32,
    /// Verification or signing error related to the input.
    pub error: String,
}

/// Result of JSON-RPC method `testmempoolaccept`.
///
/// > testmempoolaccept ["rawtxs"] ( allowhighfees )
/// >
/// > Returns if raw transaction (serialized, hex-encoded) would be accepted by mempool.
/// >
/// > This checks if the transaction violates the consensus or policy rules.
/// >
/// > See sendrawtransaction call.
/// >
/// > Arguments:
/// > 1. ["rawtxs"]       (array, required) An array of hex strings of raw transactions.
/// >                                         Length must be one for now.
/// > 2. allowhighfees    (boolean, optional, default=false) Allow high fees
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAccept(pub Vec<MempoolAcceptance>);

/// A single mempool acceptance test result. Part of `testmempoolaccept`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolAcceptance {
    /// The transaction hash in hex.
    pub txid: String,
    /// If the mempool allows this tx to be inserted.
    pub allowed: bool,
    /// Rejection string (only present when 'allowed' is false).
    #[serde(rename = "reject-reason")]
    pub reject_reason: Option<String>,
}
