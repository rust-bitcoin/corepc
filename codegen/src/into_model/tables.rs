// SPDX-License-Identifier: CC0-1.0

//! The `into_model` override tables: the small hand-maintained data the generic conversion rules
//! can't derive. Naming divergences ([`TYPE_ALIAS`]/[`FIELD_ALIAS`]), multi-field and whole-value
//! reconstructions ([`RECONSTRUCT`]/[`TYPE_RECONSTRUCT`]/[`ENUM_RECONSTRUCT`]), and known-buggy
//! canonical types routed through `compatibility.rs` ([`COMPAT`]). See the `super` module docs.

/// Top-level raw type names whose canonical model name diverges (verbose levels, RPC aliases).
///
/// `(raw type name, canonical model type name)`. Nested types never need an entry here: their
/// canonical target comes from the parent field's declared type.
pub(crate) const TYPE_ALIAS: &[(&str, &str)] = &[
    ("GetRawTransactionVerbose0", "GetRawTransaction"),
    ("GetRawTransactionVerbose1", "GetRawTransactionVerbose"),
    ("SignRawTransactionWithKey", "SignRawTransaction"),
    // Blockchain: verbose levels / action variants whose canonical name drops the suffix.
    ("GetBlockHeaderVerbose0", "GetBlockHeader"),
    ("GetBlockHeaderVerbose1", "GetBlockHeaderVerbose"),
    ("GetMempoolAncestorsVerbose0", "GetMempoolAncestors"),
    ("GetMempoolAncestorsVerbose1", "GetMempoolAncestorsVerbose"),
    ("GetMempoolDescendantsVerbose0", "GetMempoolDescendants"),
    ("GetMempoolDescendantsVerbose1", "GetMempoolDescendantsVerbose"),
    ("ScanTxOutSetVariant0", "ScanTxOutSetStart"),
    ("GetTxOutVariant1", "GetTxOut"),
    // `getrawmempool` is one untagged enum covering all three response shapes; it maps to the model
    // union `GetRawMempoolResult` (bridged by the `ENUM_RECONSTRUCT` arm).
    ("GetRawMempool", "GetRawMempoolResult"),
    // Wallet: verbose levels / RPC-name aliases.
    ("SendResult", "Send"),
    ("SendManyVerbose0", "SendMany"),
    ("SendManyVerbose1", "SendManyVerbose"),
    ("SendToAddressVerbose0", "SendToAddress"),
];

/// Semantic field renames the canonical model chose that name-normalization can't bridge.
///
/// `(canonical type, canonical field, raw field rust-name)`. Used before the normalized-name match.
pub(crate) const FIELD_ALIAS: &[(&str, &str, &str)] = &[
    // The canonical types expose a decoded `Transaction`, decoded from the raw `hex` string.
    ("DescriptorProcessPsbt", "tx", "hex"),
    ("FinalizePsbt", "tx", "hex"),
    ("FundRawTransaction", "tx", "hex"),
    ("SignRawTransaction", "tx", "hex"),
    ("GetRawTransactionVerbose", "transaction", "hex"),
    ("GetRawTransactionVerbose", "transaction_time", "time"),
    // Spelled-out / pluralized names the splitter can't reach from the wire name.
    ("FundRawTransaction", "change_position", "changepos"),
    ("SubmitPackageTxResultFees", "base_fee", "base"),
    ("DeploymentInfo", "deployment_type", "type_"),
    // Wallet.
    ("GetTransaction", "tx", "hex"),
    ("BumpFee", "original_fee", "orig_fee"),
    ("PsbtBumpFee", "original_fee", "orig_fee"),
    ("WalletCreateFundedPsbt", "change_position", "changepos"),
    // Core-wide field renames (any type): the canonical `descriptor`/`parent_descriptors` are
    // Core's `desc`/`parent_descs`.
    ("*", "descriptor", "desc"),
    ("*", "parent_descriptors", "parent_descs"),
    ("*", "parent_descriptor", "parent_desc"),
    // Blockchain.
    ("ChainTips", "branch_length", "branchlen"),
    // `getdescriptoractivity` spend entry: Core's wire field is `spend_vin` (the input index); the
    // canonical model names it `spend_vout`.
    ("SpendActivity", "spend_vout", "spend_vin"),
    // A `scriptPubKey` object's `hex` is the canonical `script_pubkey`.
    ("ScriptPubKey", "script_pubkey", "hex"),
    // `getblockstats`: the canonical spells out the abbreviations Core uses on the wire.
    ("GetBlockStats", "average_fee", "avg_fee"),
    ("GetBlockStats", "average_fee_rate", "avg_fee_rate"),
    ("GetBlockStats", "average_tx_size", "avg_tx_size"),
    ("GetBlockStats", "inputs", "ins"),
    ("GetBlockStats", "minimum_fee", "min_fee"),
    ("GetBlockStats", "minimum_fee_rate", "min_fee_rate"),
    ("GetBlockStats", "minimum_tx_size", "min_tx_size"),
    ("GetBlockStats", "outputs", "outs"),
    ("GetBlockStats", "segwit_total_size", "sw_total_size"),
    ("GetBlockStats", "segwit_total_weight", "sw_total_weight"),
    ("GetBlockStats", "segwit_txs", "swtxs"),
    ("GetBlockStats", "utxo_size_increase", "utxo_size_inc"),
    ("GetBlockStats", "utxo_size_increase_actual", "utxo_size_inc_actual"),
];

/// A `(canonical type, field)` whose canonical Rust type is buggy: codegen routes it through a
/// hand-shaped wrong-but-compilable shim in `compatibility.rs` instead of a real conversion.
pub(crate) struct CompatRule {
    pub(crate) canon_type: &'static str,
    pub(crate) field: &'static str,
    pub(crate) shim: &'static str,
    pub(crate) input: &'static str,
    pub(crate) returns: &'static str,
    pub(crate) placeholder: &'static str,
    pub(crate) correct: &'static str,
    pub(crate) reason: &'static str,
}

pub(crate) const COMPAT: &[CompatRule] = &[
    CompatRule {
        canon_type: "DumpTxOutSet",
        field: "coins_written",
        shim: "dump_tx_out_set_coins_written",
        input: "u64",
        returns: "Amount",
        placeholder: "Amount::from_sat(0)",
        correct: "self.coins_written",
        reason: "canonical types `coins_written` as `Amount` but Core returns a UTXO count \
                 (corepc_bugs_backlog.md #1)",
    },
    CompatRule {
        canon_type: "LoadTxOutSet",
        field: "coins_loaded",
        shim: "load_tx_out_set_coins_loaded",
        input: "u64",
        returns: "Amount",
        placeholder: "Amount::from_sat(0)",
        correct: "self.coins_loaded",
        reason: "canonical types `coins_loaded` as `Amount` but Core returns a UTXO count \
                 (corepc_bugs_backlog.md #1)",
    },
];

/// A canonical field built by composing several raw fields into one rust-bitcoin value (an
/// `OutPoint` from `txid` + `vout`, say) - something the per-field matcher can't express. The
/// generator drops in `expr` verbatim instead of matching a single raw field.
pub(crate) struct ReconstructRule {
    pub(crate) canon_type: &'static str,
    pub(crate) field: &'static str,
    /// Expression building the value from `self.<raw fields>`; may use `?` with the declared error
    /// variants (`E::<variant>`) and `crate::to_u32`/etc.
    pub(crate) expr: &'static str,
    /// Error variants the `expr` references via `map_err(E::..)`: `(variant, error type)`.
    pub(crate) errs: &'static [(&'static str, &'static str)],
    /// Whether `expr` uses `crate::NumericError` (so the shared `Numeric` variant is added).
    pub(crate) numeric: bool,
    /// `(raw, canonical)` type pairs the `expr` converts via `.into_model()`; queued so their own
    /// `into_model` is emitted (the generic field walk can't reach a type a reconstruction reads).
    pub(crate) nested: &'static [(&'static str, &'static str)],
}

pub(crate) const RECONSTRUCT: &[ReconstructRule] = &[
    ReconstructRule {
        canon_type: "GetTxSpendingPrevoutItem",
        field: "outpoint",
        expr: "OutPoint { txid: self.txid.parse::<Txid>().map_err(E::OutpointTxid)?, vout: crate::to_u32(self.vout, \"vout\")? }",
        errs: &[("OutpointTxid", "hex::HexToArrayError")],
        numeric: true,
        nested: &[],
    },
    // `gettxout` returns a flat `{ value, scriptPubKey: { hex, address } }`; the canonical re-types
    // it as a strong `TxOut` plus an `Option<Address>`.
    ReconstructRule {
        canon_type: "GetTxOut",
        field: "tx_out",
        expr: "TxOut { value: Amount::from_btc(self.value).map_err(E::TxOutValue)?, script_pubkey: ScriptBuf::from_hex(&self.script_pub_key.hex).map_err(E::TxOutScript)? }",
        errs: &[("TxOutValue", "amount::ParseAmountError"), ("TxOutScript", "hex::HexToBytesError")],
        numeric: false,
        nested: &[],
    },
    ReconstructRule {
        canon_type: "GetTxOut",
        field: "address",
        expr: "self.script_pub_key.address.map(|a| a.parse::<Address<NetworkUnchecked>>()).transpose().map_err(E::Address)?",
        errs: &[("Address", "address::ParseError")],
        numeric: false,
        nested: &[],
    },
    // `getaddressinfo` reports the witness program as a `(witness_version, witness_program)` pair of
    // an int and a hex string; the canonical model couples them into `WitnessVersion`/`WitnessProgram`
    // (both present or both absent). One rule per canonical field, each reading both raw fields. The
    // same shape is reused by the recursive `embedded` object.
    ReconstructRule {
        canon_type: "GetAddressInfo",
        field: "witness_version",
        expr: "match (self.witness_version, self.witness_program.as_ref()) { (Some(v), Some(_)) => Some(WitnessVersion::try_from(u8::try_from(v).map_err(|_| crate::NumericError::Overflow { value: v, field: \"witness_version\".to_owned() })?).map_err(E::WitnessVersion)?), _ => None }",
        errs: &[("WitnessVersion", "witness_version::TryFromError")],
        numeric: true,
        nested: &[],
    },
    ReconstructRule {
        canon_type: "GetAddressInfo",
        field: "witness_program",
        expr: "match (self.witness_version, self.witness_program.as_ref()) { (Some(v), Some(p)) => Some({ let wv = WitnessVersion::try_from(u8::try_from(v).map_err(|_| crate::NumericError::Overflow { value: v, field: \"witness_program\".to_owned() })?).map_err(E::WitnessVersion)?; let bytes = Vec::<u8>::from_hex(p).map_err(E::WitnessProgramBytes)?; WitnessProgram::new(wv, &bytes).map_err(E::WitnessProgram)? }), _ => None }",
        errs: &[("WitnessVersion", "witness_version::TryFromError"), ("WitnessProgramBytes", "hex::HexToBytesError"), ("WitnessProgram", "witness_program::Error")],
        numeric: true,
        nested: &[],
    },
    ReconstructRule {
        canon_type: "GetAddressInfoEmbedded",
        field: "witness_version",
        expr: "match (self.witness_version, self.witness_program.as_ref()) { (Some(v), Some(_)) => Some(WitnessVersion::try_from(u8::try_from(v).map_err(|_| crate::NumericError::Overflow { value: v, field: \"witness_version\".to_owned() })?).map_err(E::WitnessVersion)?), _ => None }",
        errs: &[("WitnessVersion", "witness_version::TryFromError")],
        numeric: true,
        nested: &[],
    },
    ReconstructRule {
        canon_type: "GetAddressInfoEmbedded",
        field: "witness_program",
        expr: "match (self.witness_version, self.witness_program.as_ref()) { (Some(v), Some(p)) => Some({ let wv = WitnessVersion::try_from(u8::try_from(v).map_err(|_| crate::NumericError::Overflow { value: v, field: \"witness_program\".to_owned() })?).map_err(E::WitnessVersion)?; let bytes = Vec::<u8>::from_hex(p).map_err(E::WitnessProgramBytes)?; WitnessProgram::new(wv, &bytes).map_err(E::WitnessProgram)? }), _ => None }",
        errs: &[("WitnessVersion", "witness_version::TryFromError"), ("WitnessProgramBytes", "hex::HexToBytesError"), ("WitnessProgram", "witness_program::Error")],
        numeric: true,
        nested: &[],
    },
    // `validateaddress` reports the same `(witness_version, witness_program)` int+hex pair as
    // `getaddressinfo`; the canonical couples them into `WitnessVersion`/`WitnessProgram`.
    ReconstructRule {
        canon_type: "ValidateAddress",
        field: "witness_version",
        expr: "match (self.witness_version, self.witness_program.as_ref()) { (Some(v), Some(_)) => Some(WitnessVersion::try_from(u8::try_from(v).map_err(|_| crate::NumericError::Overflow { value: v, field: \"witness_version\".to_owned() })?).map_err(E::WitnessVersion)?), _ => None }",
        errs: &[("WitnessVersion", "witness_version::TryFromError")],
        numeric: true,
        nested: &[],
    },
    ReconstructRule {
        canon_type: "ValidateAddress",
        field: "witness_program",
        expr: "match (self.witness_version, self.witness_program.as_ref()) { (Some(v), Some(p)) => Some({ let wv = WitnessVersion::try_from(u8::try_from(v).map_err(|_| crate::NumericError::Overflow { value: v, field: \"witness_program\".to_owned() })?).map_err(E::WitnessVersion)?; let bytes = Vec::<u8>::from_hex(p).map_err(E::WitnessProgramBytes)?; WitnessProgram::new(wv, &bytes).map_err(E::WitnessProgram)? }), _ => None }",
        errs: &[("WitnessVersion", "witness_version::TryFromError"), ("WitnessProgramBytes", "hex::HexToBytesError"), ("WitnessProgram", "witness_program::Error")],
        numeric: true,
        nested: &[],
    },
    // `getblock` verbosity 2 nests each transaction as a flat `getrawtransaction`-verbose object the
    // generated raw type captures in an untyped `extra` map. Re-type that map as the sibling raw
    // verbose-tx type and run its own `into_model` (the `transaction` field has no single raw field).
    ReconstructRule {
        canon_type: "GetBlockVerboseTwoTransaction",
        field: "transaction",
        expr: "crate::reconstruct::from_flat_fields::<super::super::GetRawTransactionVerbose1>(&self.extra).map_err(E::Transaction)?.into_model().map_err(E::TransactionModel)?",
        errs: &[("Transaction", "serde_json::Error"), ("TransactionModel", "super::super::GetRawTransactionVerboseError")],
        numeric: false,
        nested: &[],
    },
    // `getblock` verbosity 3 is verbosity 2 plus per-input `prevout` data. The raw item pulls `vin`
    // out of the flattened tx body (to carry the prevouts) and keeps the block-only `fee` in
    // `extra`; rebuild the verbose tx by re-inserting `vin` and dropping `fee`, take the prevouts
    // from each input, and read `fee` back out of `extra`.
    ReconstructRule {
        canon_type: "GetBlockVerboseThreeTransaction",
        field: "transaction",
        expr: "crate::reconstruct::block_verbose3_tx::<super::super::GetRawTransactionVerbose1, _>(&self.extra, &self.vin).map_err(E::Transaction)?.into_model().map_err(E::TransactionModel)?",
        errs: &[("Transaction", "serde_json::Error"), ("TransactionModel", "super::super::GetRawTransactionVerboseError")],
        numeric: false,
        nested: &[],
    },
    ReconstructRule {
        canon_type: "GetBlockVerboseThreeTransaction",
        field: "prevouts",
        expr: "self.vin.iter().map(|vi| vi.prevout.clone().map(|p| p.into_model()).transpose()).collect::<Result<Vec<_>, _>>().map_err(E::Prevouts)?",
        errs: &[("Prevouts", "GetBlockVerboseThreePrevoutError")],
        numeric: false,
        nested: &[("GetBlockVerbose3TxItemVinItemPrevout", "GetBlockVerboseThreePrevout")],
    },
    ReconstructRule {
        canon_type: "GetBlockVerboseThreeTransaction",
        field: "fee",
        expr: "self.extra.get(\"fee\").and_then(|v| v.as_f64()).map(Amount::from_btc).transpose().map_err(E::Fee)?",
        errs: &[("Fee", "amount::ParseAmountError")],
        numeric: false,
        nested: &[],
    },
    // `decodepsbt` returns the PSBT already decomposed into its parts (no base64 wire form), so the
    // canonical `psbt: bitcoin::Psbt` is reassembled from the decoded fields by the shared helper.
    ReconstructRule {
        canon_type: "DecodePsbt",
        field: "psbt",
        expr: "crate::reconstruct::psbt(&self).map_err(E::Psbt)?",
        errs: &[("Psbt", "crate::reconstruct::ReconstructPsbtError")],
        numeric: false,
        nested: &[],
    },
    // `listsinceblock.removed` has the same shape as `transactions` but the OpenRPC spec leaves the
    // array element untyped (`serde_json::Value`). Decode each element through the sibling raw item
    // type, then run its own `into_model` (emitted via the `transactions` field pairing).
    ReconstructRule {
        canon_type: "ListSinceBlock",
        field: "removed",
        expr: "self.removed.unwrap_or_default().into_iter().map(|v| serde_json::from_value::<ListSinceBlockTransactionsItem>(v).map_err(E::Removed)?.into_model().map_err(E::RemovedItem)).collect::<Result<Vec<_>, _>>()?",
        errs: &[("Removed", "serde_json::Error"), ("RemovedItem", "TransactionItemError")],
        numeric: false,
        nested: &[],
    },
];

/// A whole canonical type whose value is rebuilt by a hand-written expression rather than
/// field-matched: a `bitcoin::Transaction` (etc.) assembled from a decoded response. `inner_expr`
/// produces the value the canonical newtype wraps, from `self`, using `E::<variant>` for errors.
pub(crate) struct TypeReconstruct {
    pub(crate) canon_type: &'static str,
    pub(crate) inner_expr: &'static str,
    pub(crate) errs: &'static [(&'static str, &'static str)],
}

pub(crate) const TYPE_RECONSTRUCT: &[TypeReconstruct] = &[
    TypeReconstruct {
        canon_type: "DecodeRawTransaction",
        inner_expr: "crate::reconstruct::transaction(&self).map_err(E::Reconstruct)?",
        errs: &[("Reconstruct", "crate::reconstruct::ReconstructError")],
    },
    // `listaddressgroupings` returns nested arrays whose innermost element is a positional,
    // mixed-type array `[address, amount, label?]` (the OpenRPC-derived raw is therefore
    // `Vec<Vec<Vec<serde_json::Value>>>`). Decode each positional array by index.
    TypeReconstruct {
        canon_type: "ListAddressGroupings",
        inner_expr: "self.0.into_iter().map(|group| group.into_iter().map(|item| { let address = item.first().and_then(|v| v.as_str()).ok_or(E::MissingAddress(crate::MissingField { field: \"address\" }))?.parse::<Address<NetworkUnchecked>>().map_err(E::Address)?; let amount = Amount::from_btc(item.get(1).and_then(|v| v.as_f64()).ok_or(E::MissingAmount(crate::MissingField { field: \"amount\" }))?).map_err(E::Amount)?; let label = item.get(2).and_then(|v| v.as_str()).map(|s| s.to_owned()); Ok::<_, E>(model::ListAddressGroupingsItem { address, amount, label }) }).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?",
        errs: &[
            ("Address", "address::ParseError"),
            ("Amount", "amount::ParseAmountError"),
            ("MissingAddress", "crate::MissingField"),
            ("MissingAmount", "crate::MissingField"),
        ],
    },
];

/// A whole canonical enum whose `into_model` body is a hand-written `match self { .. }`: a raw
/// untagged enum whose variants map to several unrelated canonical types (not a uniform
/// `.into_model()` per arm). `body` is the full `Ok(match self { .. })` expression.
pub(crate) struct EnumReconstruct {
    pub(crate) canon_type: &'static str,
    pub(crate) body: &'static str,
    /// Error variants the `body` references via `map_err(E::..)`: `(variant, error type)`.
    pub(crate) errs: &'static [(&'static str, &'static str)],
    /// `(raw, canonical)` pairs the `body` converts via `.into_model()`; queued so their own
    /// `into_model` is emitted (the variant inner types the generic walk can't otherwise reach).
    pub(crate) nested: &'static [(&'static str, &'static str)],
}

pub(crate) const ENUM_RECONSTRUCT: &[EnumReconstruct] = &[
    // `getrawmempool` is a single untagged enum (the `oneOf` is selected by two parameters, which the
    // verbose splitter deliberately does not split) whose three shapes map to three unrelated model
    // types. Bridge each arm by hand: the id list and the verbose map wrap their newtypes directly,
    // the sequence struct delegates to `GetRawMempoolVariant2::into_model` (queued via `nested`).
    EnumReconstruct {
        canon_type: "GetRawMempoolResult",
        body: "Ok(match self {\n            GetRawMempool::List(txids) => model::GetRawMempoolResult::List(model::GetRawMempool(txids.into_iter().map(|t| t.parse::<Txid>()).collect::<Result<_, _>>().map_err(E::Txid)?)),\n            GetRawMempool::Object(map) => model::GetRawMempoolResult::Verbose(model::GetRawMempoolVerbose(map.into_iter().map(|(k, v)| Ok::<_, E>((k.parse::<Txid>().map_err(E::Txid)?, v.into_model().map_err(E::Verbose)?))).collect::<Result<_, _>>()?)),\n            GetRawMempool::Object2(seq) => model::GetRawMempoolResult::Sequence(seq.into_model().map_err(E::Sequence)?),\n        })",
        errs: &[
            ("Txid", "hex::HexToArrayError"),
            ("Verbose", "MempoolEntryError"),
            ("Sequence", "GetRawMempoolSequenceError"),
        ],
        nested: &[
            ("GetRawMempoolVariant1", "MempoolEntry"),
            ("GetRawMempoolVariant2", "GetRawMempoolSequence"),
        ],
    },
];
