// SPDX-License-Identifier: CC0-1.0

//! Isolation bridges for the async blocking facade.
//!
//! These are the methods the facade implements *itself* instead of reusing the sync client's
//! `impl_client_*` macros, so a bug in the sync argument-encoding cannot reach the async path. The
//! generated `client_async/v{N}/blocking.rs` invokes [`impl_async_bridges!`] once and skips the
//! matching sync macros.
//!
//! Unlike codegen string templates, this is real Rust: the compiler, `rustfmt` and `rust-analyzer`
//! all check it. The version is passed as a token (`impl_async_bridges!(v31)`) so the
//! version-specific paths (`$crate::types::v31::...`) resolve; the curated response types are
//! resolved unqualified at the call site via the facade's `use crate::types::v{N}::*`.
//!
//! Adding a bridge: add the method here AND add its sync-macro suffix to `BRIDGED_METHODS` in
//! `codegen/src/codegen/blocking.rs` so the reused sync macro is skipped (no duplicate definition).

/// Emit the blocking facade's bridged methods for version `$v`, e.g. `impl_async_bridges!(v31)`.
///
/// Expanded inside `client_async/v{N}/blocking.rs`, where `Client`, `Result`, `Error`, `into_json`,
/// `AddressType` and the curated response types are all in scope.
#[macro_export]
macro_rules! impl_async_bridges {
    ($v:ident) => {
        // == Wallet ==
        // `getnewaddress` routes through the GENERATED async wrapper (its own argument encoding),
        // NOT the reused sync macro, so the sync client's arg-encoding (and any bug in it) cannot
        // reach the async path.
        impl Client {
            fn get_new_address_generated(
                &self,
                label: Option<&str>,
                ty: Option<AddressType>,
            ) -> Result<$crate::types::$v::generated::GetNewAddress> {
                let address_type = match ty {
                    Some(ty) => Some(serde_json::from_value::<String>(into_json(ty)?)?),
                    None => None,
                };
                let opts = $crate::client_async::$v::wallet::GetNewAddressOptions {
                    label: label.map(str::to_owned),
                    address_type,
                };
                self.rt.block_on(self.inner.get_new_address_with(opts)).map_err(Self::map_err)
            }

            /// Low-level `getnewaddress` (matches the sync client's signature).
            pub fn get_new_address(
                &self,
                label: Option<&str>,
                ty: Option<AddressType>,
            ) -> Result<$crate::types::$v::generated::GetNewAddress> {
                self.get_new_address_generated(label, ty)
            }

            /// Gets a new address and parses it assuming it is correct.
            pub fn new_address(&self) -> Result<bitcoin::Address> {
                let model = self.get_new_address_generated(None, None)?.into_model().unwrap();
                Ok(model.0.assume_checked())
            }

            /// Gets a new address of the given type and parses it assuming it is correct.
            pub fn new_address_with_type(&self, ty: AddressType) -> Result<bitcoin::Address> {
                let model = self.get_new_address_generated(None, Some(ty))?.into_model().unwrap();
                Ok(model.0.assume_checked())
            }

            /// Gets a new address with a label and parses it assuming it is correct (unchecked).
            pub fn new_address_with_label(
                &self,
                label: &str,
            ) -> Result<bitcoin::Address<bitcoin::address::NetworkUnchecked>> {
                let model = self.get_new_address_generated(Some(label), None)?.into_model().unwrap();
                Ok(model.0)
            }
        }

        // == Generating ==
        // These RPCs are not in Core's OpenRPC, so there is no generated async wrapper to route
        // through; the facade owns a raw `self.call`, which still isolates it from the sync macro.
        impl Client {
            pub fn generate_block(
                &self,
                output: &str,
                transactions: &[String],
                submit: bool,
            ) -> Result<GenerateBlock> {
                self.call(
                    "generateblock",
                    &[into_json(output)?, into_json(transactions)?, into_json(submit)?],
                )
            }

            pub fn generate_to_address(
                &self,
                nblocks: usize,
                address: &bitcoin::Address,
            ) -> Result<GenerateToAddress> {
                self.call("generatetoaddress", &[nblocks.into(), into_json(address)?])
            }

            pub fn generate_to_descriptor(
                &self,
                nblocks: usize,
                descriptor: &str,
            ) -> Result<GenerateToDescriptor> {
                self.call("generatetodescriptor", &[nblocks.into(), descriptor.into()])
            }

            pub fn invalidate_block(&self, hash: BlockHash) -> Result<()> {
                match self.call("invalidateblock", &[into_json(hash)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }

        // == Mining ==
        // Each method routes through the GENERATED async wrapper on `self.inner` (its own
        // arg-encoding plus generated response type), NOT a raw `self.call`, so the generated call
        // surface is exercised and stays isolated from the sync macro. Where the generated response
        // type differs from the curated type the explicit tests consume, the response is converted
        // back through JSON (both sides serialize to Core's shape).
        impl Client {
            pub fn get_block_template(&self, request: &TemplateRequest) -> Result<GetBlockTemplate> {
                // Curated `TemplateRequest` -> the wrapper's own request struct (`rules` is
                // `Vec<TemplateRules>` here, `Vec<String>` there; both serialize to Core's shape).
                let req: $crate::client_async::$v::mining::GetBlockTemplateTemplateRequest =
                    serde_json::from_value(into_json(request)?)?;
                let res = self.rt.block_on(self.inner.get_block_template(req)).map_err(Self::map_err)?;
                // Generated `GetBlockTemplate` is an untagged Null/Text/Object enum with no
                // `into_model`; the test consumes the curated struct, so convert back through JSON.
                Ok(serde_json::from_value(into_json(res)?)?)
            }

            pub fn get_mining_info(&self) -> Result<GetMiningInfo> {
                let res = self.rt.block_on(self.inner.get_mining_info()).map_err(Self::map_err)?;
                Ok(serde_json::from_value(into_json(res)?)?)
            }

            pub fn get_network_hash_ps(&self) -> Result<f64> {
                Ok(*self.rt.block_on(self.inner.get_network_hash_ps()).map_err(Self::map_err)?)
            }

            pub fn get_prioritised_transactions(&self) -> Result<GetPrioritisedTransactions> {
                let res = self
                    .rt
                    .block_on(self.inner.get_prioritised_transactions())
                    .map_err(Self::map_err)?;
                Ok(serde_json::from_value(into_json(res)?)?)
            }

            pub fn prioritise_transaction(
                &self,
                txid: &Txid,
                fee_delta: bitcoin::SignedAmount,
            ) -> Result<bool> {
                let res = self
                    .rt
                    .block_on(self.inner.prioritise_transaction(txid.to_string(), fee_delta.to_sat()))
                    .map_err(Self::map_err)?;
                Ok(*res)
            }

            pub fn submit_block(&self, block: &Block) -> Result<()> {
                let hex = bitcoin::consensus::encode::serialize_hex(block);
                match self.rt.block_on(self.inner.submit_block(hex)).map_err(Self::map_err)? {
                    $crate::types::$v::generated::SubmitBlock::Null(()) => Ok(()),
                    $crate::types::$v::generated::SubmitBlock::Text(reason) =>
                        Err(Error::Returned(reason)),
                }
            }

            pub fn submit_header(&self, header: &bitcoin::block::Header) -> Result<()> {
                let hexdata = bitcoin::consensus::encode::serialize_hex(header);
                self.rt.block_on(self.inner.submit_header(hexdata)).map_err(Self::map_err)
            }
        }
    };
}
