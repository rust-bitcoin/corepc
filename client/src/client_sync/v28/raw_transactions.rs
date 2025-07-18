// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of Bitcoin Core `v28`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `submitpackage`.
#[macro_export]
macro_rules! impl_client_v28__submit_package {
    () => {
        impl Client {
            pub fn submit_package(
                &self,
                package: &[bitcoin::Transaction],
                max_fee_rate: Option<bitcoin::FeeRate>,
                max_burn_amount: Option<bitcoin::Amount>,
            ) -> Result<SubmitPackage> {
                let package_txs = package
                    .into_iter()
                    .map(|tx| bitcoin::consensus::encode::serialize_hex(tx))
                    .collect::<Vec<_>>();
                let max_fee_rate_btc_kvb =
                    max_fee_rate.map(|r| r.to_sat_per_vb_floor() as f64 / 100_000.0);
                let max_burn_amount_btc = max_burn_amount.map(|a| a.to_btc());
                self.call(
                    "submitpackage",
                    &[package_txs.into(), max_fee_rate_btc_kvb.into(), max_burn_amount_btc.into()],
                )
            }
        }
    };
}
