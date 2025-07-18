// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Util ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `createmultisig`.
#[macro_export]
macro_rules! impl_client_v17__create_multisig {
    () => {
        impl Client {
            pub fn create_multisig(
                &self,
                nrequired: u32,
                keys: Vec<PublicKey>,
            ) -> Result<CreateMultisig> {
                self.call("createmultisig", &[nrequired.into(), into_json(keys)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `estimatesmartfee`.
#[macro_export]
macro_rules! impl_client_v17__estimate_smart_fee {
    () => {
        impl Client {
            pub fn estimate_smart_fee(&self, blocks: u32) -> Result<EstimateSmartFee> {
                self.call("estimatesmartfee", &[blocks.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signmessagewithprivkey`.
#[macro_export]
macro_rules! impl_client_v17__sign_message_with_priv_key {
    () => {
        impl Client {
            pub fn sign_message_with_privkey(
                &self,
                privkey: &bitcoin::PrivateKey,
                message: &str,
            ) -> Result<SignMessageWithPrivKey> {
                self.call("signmessagewithprivkey", &[into_json(privkey)?, message.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `validateaddress`.
#[macro_export]
macro_rules! impl_client_v17__validate_address {
    () => {
        impl Client {
            pub fn validate_address(
                &self,
                address: &Address<NetworkChecked>,
            ) -> Result<ValidateAddress> {
                self.call("validateaddress", &[address.to_string().into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `verifymessage`.
#[macro_export]
macro_rules! impl_client_v17__verify_message {
    () => {
        impl Client {
            pub fn verify_message(
                &self,
                address: &Address<NetworkChecked>,
                signature: &sign_message::MessageSignature,
                message: &str,
            ) -> Result<VerifyMessage> {
                self.call(
                    "verifymessage",
                    &[address.to_string().into(), signature.to_string().into(), message.into()],
                )
            }
        }
    };
}
