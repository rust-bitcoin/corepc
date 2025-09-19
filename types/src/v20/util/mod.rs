// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.20` - util.
//!
//! Types for methods found under the `== Util ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

pub use super::CreateMultisigError;

/// Result of JSON-RPC method `createmultisig`.
///
/// > createmultisig nrequired ["key",...] ( "address_type" )
/// >
/// > Creates a multi-signature address with n signature of m keys required.
/// > It returns a json object with the address and redeemScript.
/// >
/// > Arguments:
/// > 1. nrequired                    (numeric, required) The number of required signatures out of the n keys.
/// > 2. "keys"                       (string, required) A json array of hex-encoded public keys
/// >      [
/// >        "key"                    (string) The hex-encoded public key
/// >        ,...
/// >      ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateMultisig {
    /// The value of the new multisig address.
    pub address: String,
    /// The string value of the hex-encoded redemption script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
    /// The descriptor for this multisig.
    pub descriptor: String,
}
