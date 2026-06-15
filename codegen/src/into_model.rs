// SPDX-License-Identifier: CC0-1.0

//! Generates the `into_model` conversions from the raw response types into the shared,
//! version-nonspecific `crate::model` types in `corepc-types`.
//!
//! This is the codegen counterpart of the hand-written `vN/<category>/into.rs` files. For every
//! whitelisted type (and every nested type it reaches) it emits
//! `impl RawType { fn into_model(self) -> Result<model::T, E> }` plus the error enum.
//!
//! # How it decides each conversion
//!
//! The generator does NOT hand-encode conversions per RPC. It reads two shapes and bridges them:
//!
//! 1. the RAW type's fields (name + Rust type) from the spec-derived IR, and
//! 2. the CANONICAL `crate::model` type's fields, parsed from the `corepc-types` model source.
//!
//! Fields are matched by a normalized name key (so `chainwork` <-> `chain_work`). The model field
//! name and the target type come from the canonical struct, so per-RPC quirks (a field made
//! `Option`, a field dropped, a field strongly typed) need no special casing: they fall out of
//! whatever the canonical type declares. The conversion expression itself is chosen by generic
//! type-pair rules in [`leaf`] (`String -> BlockHash`, `i64 -> u32`, `f64 -> Amount`, ...) with
//! [`convert`] handling the `Option`/`Vec`/`BTreeMap` wrappers around them.
//!
//! # Nested types
//!
//! When a canonical field is itself a generated type (e.g. `Vec<MempoolAcceptance>`), the matching
//! raw field is a generated type too (`Vec<TestMempoolAcceptItem>`). The pairing comes from the
//! PARENT field, not from a name match: the element is converted with `.into_model()` and the
//! `(raw, canonical)` pair is queued so its own `into_model` is emitted. This walks the whole type
//! graph from each whitelisted root without any nested-type name table.
//!
//! # Escape hatches
//!
//! - A raw field with no canonical counterpart is dropped; an unmatched canonical `Option`/`Vec`
//!   defaults to `None`/`vec![]`, any other unmatched canonical field becomes a `todo!()`.
//! - A handful of top-level entry points whose raw name diverges from the canonical name (verbose
//!   levels, RPC aliases) are listed in [`TYPE_ALIAS`]; a handful of semantic field renames in
//!   [`FIELD_ALIAS`]. These are RPC-level naming choices, not per-field conversion tables.
//! - A `(raw, canonical)` pair the canonical type gets *wrong* is listed in [`COMPAT`] and routed to
//!   a hand-shaped placeholder in `compatibility.rs`, isolating the bug while the crate still builds
//!   (see `corepc_bugs_backlog.md`).
//!
//! Grow [`WHITELIST`] one type at a time toward full coverage; the compile gate against the real
//! `crate::model` catches any wrong field name or type.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

/// Canonical (`crate::model`) type names codegen emits an `into_model` for. Grow toward 100%.
///
/// Only the top-level RPC return types go here; nested helper types (e.g. `MempoolAcceptance`) are
/// discovered automatically from the fields of whitelisted roots.
static WHITELIST: LazyLock<Vec<String>> = LazyLock::new(|| {
    [
        // Already covered.
        "DumpTxOutSet",
        "GetBlockVerboseOne",
        // Structs whose every field is bridged by a generic `leaf` rule.
        "CreateMultisig",
        "CreateWallet",
        "DescriptorProcessPsbt",
        "FinalizePsbt",
        "GetBalancesMine",
        "ListLockUnspentItem",
        "ListReceivedByAddressItem",
        "ListReceivedByLabelItem",
        "LoadWallet",
        "RescanBlockchain",
        "SendAll",
        "SimulateRawTransaction",
        "UnloadWallet",
        "WaitForBlock",
        "WaitForBlockHeight",
        "WaitForNewBlock",
        "WalletDisplayAddress",
        "WalletProcessPsbt",
        // Newtypes.
        "GetBestBlockHash",
        "GetBlockHash",
        "GetBalance",
        "GetDifficulty",
        "GetNewAddress",
        "GetRawChangeAddress",
        "GetReceivedByAddress",
        "GetReceivedByLabel",
        "SendRawTransaction",
        // Raw transactions (this pass). Nested types pulled in automatically:
        // AnalyzePsbtInput(Missing), SignFail, SubmitPackageTxResult(Fees),
        // MempoolAcceptance(Fees).
        "AnalyzePsbt",
        "CombinePsbt",
        "CombineRawTransaction",
        "ConvertToPsbt",
        "CreatePsbt",
        "CreateRawTransaction",
        "JoinPsbts",
        "UtxoUpdatePsbt",
        "DecodePsbt",
        "DecodeRawTransaction",
        "DecodeScript",
        "FundRawTransaction",
        "GetRawTransaction",
        "GetRawTransactionVerbose",
        "SignRawTransaction",
        "SubmitPackage",
        "TestMempoolAccept",
        // Blockchain (this pass). Nested types pulled in automatically: ChainState, MempoolEntry,
        // MempoolEntryFees, DeploymentInfo, Bip9Info, Bip9Statistics, GetTxOutSetInfoBlockInfo,
        // GetTxOutSetInfoUnspendables, ScanTxOutSetUnspent, the verbose mempool-entry types.
        "GetBlockCount",
        "GetBlockVerboseZero",
        "GetBlockHeader",
        "GetBlockHeaderVerbose",
        "VerifyTxOutProof",
        "GetBlockchainInfo",
        "GetChainStates",
        "GetChainTxStats",
        "GetDeploymentInfo",
        "GetMempoolAncestors",
        "GetMempoolAncestorsVerbose",
        "GetMempoolDescendants",
        "GetMempoolDescendantsVerbose",
        "GetMempoolEntry",
        "GetMempoolInfo",
        "GetTxOutSetInfo",
        "LoadTxOutSet",
        "ScanTxOutSetStart",
    ]
    .into_iter()
    .map(String::from)
    .collect()
});

/// Top-level raw type names whose canonical model name diverges (verbose levels, RPC aliases).
///
/// `(raw type name, canonical model type name)`. Nested types never need an entry here: their
/// canonical target comes from the parent field's declared type.
const TYPE_ALIAS: &[(&str, &str)] = &[
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
];

/// Semantic field renames the canonical model chose that name-normalization can't bridge.
///
/// `(canonical type, canonical field, raw field rust-name)`. Used before the normalized-name match.
const FIELD_ALIAS: &[(&str, &str, &str)] = &[
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
    ("DecodeScript", "descriptor", "desc"),
    ("DeploymentInfo", "deployment_type", "type_"),
];

/// A `(canonical type, field)` whose canonical Rust type is buggy: codegen routes it through a
/// hand-shaped wrong-but-compilable shim in `compatibility.rs` instead of a real conversion.
struct CompatRule {
    canon_type: &'static str,
    field: &'static str,
    shim: &'static str,
    input: &'static str,
    returns: &'static str,
    placeholder: &'static str,
    correct: &'static str,
    reason: &'static str,
}

const COMPAT: &[CompatRule] = &[
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

/// One field of a raw response struct, handed to the generator by `codegen`.
pub struct RawField {
    pub name: String,
    /// The full Rust type including any `Option<...>`/`Vec<...>` wrapper.
    pub ty: String,
}

/// The shape of a raw response type.
pub enum RawShape {
    /// `pub struct X { .. }`.
    Struct(Vec<RawField>),
    /// `pub struct X(pub Inner);`, the inner Rust type.
    Newtype(String),
}

/// A raw response type and its shape.
pub struct RawTypeInfo {
    pub raw_name: String,
    pub shape: RawShape,
}

/// Per-category lookup the generator threads through the type-graph walk.
struct Ctx<'a> {
    /// Every response type in the category by raw name; membership marks a "generated" type, which
    /// is how a nested field is told apart from a primitive.
    raw: BTreeMap<String, &'a RawTypeInfo>,
    model_dir: &'a Path,
}

/// The canonical `crate::model` type name for a raw type name. [`TYPE_ALIAS`] first, then the
/// verbose digit->word rewrite (`GetBlockVerbose1` -> `GetBlockVerboseOne`), else identity.
pub fn model_typename(raw: &str) -> String {
    if let Some((_, canon)) = TYPE_ALIAS.iter().find(|(r, _)| *r == raw) {
        return (*canon).to_owned();
    }
    let mut s = raw.to_owned();
    for (digit, word) in [
        ("Verbose0", "VerboseZero"),
        ("Verbose1", "VerboseOne"),
        ("Verbose2", "VerboseTwo"),
        ("Verbose3", "VerboseThree"),
    ] {
        s = s.replace(digit, word);
    }
    s
}

/// Whether a raw type's canonical name is a whitelisted root for `into_model` generation.
pub fn is_whitelisted(raw_name: &str) -> bool { WHITELIST.contains(&model_typename(raw_name)) }

/// Normalized name key for matching a raw field to a canonical field (`chain_work` -> `chainwork`).
fn norm(name: &str) -> String {
    name.chars().filter(|c| *c != '_').flat_map(char::to_lowercase).collect()
}

/// PascalCase an error-variant name from a snake_case field (`chain_work` -> `ChainWork`).
fn variant_of(field: &str) -> String {
    field
        .split('_')
        .map(|seg| {
            let mut c = seg.chars();
            match c.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Strip a generic wrapper: `strip_wrap("Option<X>", "Option<")` -> `Some("X")`.
fn strip_wrap<'a>(ty: &'a str, open: &str) -> Option<&'a str> {
    ty.strip_prefix(open)?.strip_suffix('>').map(str::trim)
}

/// If `expr` is `PATH(var)`, return `PATH` so `.map(|var| expr)` can become `.map(PATH)` (avoids a
/// `clippy::redundant_closure`). Only a bare function-path call qualifies, not `var.method()` or
/// `f(&var)`.
fn fn_path<'a>(expr: &'a str, var: &str) -> Option<&'a str> {
    let inner = expr.strip_suffix(&format!("({var})"))?;
    (!inner.is_empty() && !inner.contains(|c: char| !c.is_alphanumeric() && c != '_' && c != ':'))
        .then_some(inner)
}

/// Split a `BTreeMap<K, V>` (either path) into `(K, V)`, depth-aware on the top-level comma.
fn map_pair(ty: &str) -> Option<(&str, &str)> {
    let inner =
        strip_wrap(ty, "BTreeMap<").or_else(|| strip_wrap(ty, "std::collections::BTreeMap<"))?;
    let mut depth = 0usize;
    for (i, b) in inner.bytes().enumerate() {
        match b {
            b'<' => depth += 1,
            b'>' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => return Some((inner[..i].trim(), inner[i + 1..].trim())),
            _ => {}
        }
    }
    None
}

/// How a leaf conversion can fail.
enum LeafErr {
    /// A named per-field variant carrying an error type (e.g. `Bits(UnprefixedHexError)`).
    Named(String),
    /// The shared `Numeric(NumericError)` variant.
    Numeric,
}

/// A single element-level (unwrapped) conversion expression over `var`, plus how it can fail.
///
/// `expr` is a plain value when the error is `None`, otherwise a `Result` the caller threads through
/// a `map_err`/`?`. This is the heuristic table: every entry is a generic `(raw, canonical)` rule
/// that holds across all RPCs, plus the nested `.into_model()` fallback for generated types.
fn leaf(
    ctx: &Ctx,
    raw: &str,
    canon: &str,
    var: &str,
    field_lit: &str,
    nested: &mut Vec<(String, String)>,
) -> (String, Option<LeafErr>, bool) {
    // A generated type on the raw side converts via its own `into_model`; queue the pair so it is
    // emitted too. The canonical target is whatever the parent field declared. Checked before the
    // identity case because a raw and a canonical type can share a name yet be distinct types (e.g.
    // `GetTxOutSetInfoBlockInfo`), so name equality must not short-circuit them into a no-op move.
    if ctx.raw.contains_key(raw) {
        nested.push((raw.to_owned(), canon.to_owned()));
        return (
            format!("{var}.into_model()"),
            Some(LeafErr::Named(format!("{canon}Error"))),
            true,
        );
    }
    if raw == canon {
        return (var.to_owned(), None, true);
    }
    let named = |expr: String, errty: &str| (expr, Some(LeafErr::Named(errty.to_owned())), true);
    let parse = |ty: &str, errty: &str| named(format!("{var}.parse::<{ty}>()"), errty);
    let unprefixed =
        |ty: &str| named(format!("{ty}::from_unprefixed_hex(&{var})"), "UnprefixedHexError");
    match (raw, canon) {
        // Hashes and hex-encoded rust-bitcoin types (`FromStr`).
        ("String", "BlockHash") => parse("BlockHash", "hex::HexToArrayError"),
        ("String", "Txid") => parse("Txid", "hex::HexToArrayError"),
        ("String", "Wtxid") => parse("Wtxid", "hex::HexToArrayError"),
        ("String", "TxMerkleNode") => parse("TxMerkleNode", "hex::HexToArrayError"),
        ("String", "sha256::Hash") => parse("sha256::Hash", "hex::HexToArrayError"),
        ("String", "hash160::Hash") => parse("hash160::Hash", "hex::HexToArrayError"),
        ("String", "Address<NetworkUnchecked>") =>
            parse("Address<NetworkUnchecked>", "address::ParseError"),
        ("String", "Psbt") => parse("Psbt", "psbt::PsbtParseError"),
        // Difficulty targets / work (unprefixed hex).
        ("String", "CompactTarget") => unprefixed("CompactTarget"),
        ("String", "Work") => unprefixed("Work"),
        ("String", "Target") => unprefixed("Target"),
        // Scripts and transactions (consensus/hex decode).
        ("String", "ScriptBuf") =>
            named(format!("ScriptBuf::from_hex(&{var})"), "hex::HexToBytesError"),
        ("String", "Transaction") =>
            named(format!("encode::deserialize_hex::<Transaction>(&{var})"), "encode::FromHexError"),
        ("String", "Block") =>
            named(format!("encode::deserialize_hex::<Block>(&{var})"), "encode::FromHexError"),
        ("String", "block::Header") =>
            named(format!("encode::deserialize_hex::<block::Header>(&{var})"), "encode::FromHexError"),
        // Network name (`main`/`test`/`signet`/`regtest`); parsed the way `bitcoind` takes it.
        ("String", "Network") =>
            named(format!("Network::from_core_arg(&{var})"), "network::ParseNetworkError"),
        // Amounts: Core renders a `CAmount` as fractional BTC.
        ("f64", "Amount") => named(format!("Amount::from_btc({var})"), "amount::ParseAmountError"),
        ("f64", "SignedAmount") =>
            named(format!("SignedAmount::from_btc({var})"), "amount::ParseAmountError"),
        // Numeric narrowing/widening.
        ("i64", "u32") =>
            (format!("crate::to_u32({var}, {field_lit})"), Some(LeafErr::Numeric), true),
        ("u64", "u32") => (
            format!(
                "u32::try_from({var}).map_err(|_| crate::NumericError::Overflow {{ value: {var} as i64, field: {field_lit}.to_owned() }})"
            ),
            Some(LeafErr::Numeric),
            true,
        ),
        ("i64", "u64") =>
            (format!("crate::to_u64({var}, {field_lit})"), Some(LeafErr::Numeric), true),
        ("i64", "u8") => (
            format!(
                "u8::try_from({var}).map_err(|_| crate::NumericError::Overflow {{ value: {var}, field: {field_lit}.to_owned() }})"
            ),
            Some(LeafErr::Numeric),
            true,
        ),
        // Infallible strong types.
        ("i64", "Weight") => (format!("Weight::from_wu({var} as u64)"), None, true),
        ("u64", "Weight") => (format!("Weight::from_wu({var})"), None, true),
        ("i64", "block::Version") =>
            (format!("block::Version::from_consensus({var} as i32)"), None, true),
        ("i64", "Sequence") => (format!("Sequence::from_consensus({var} as u32)"), None, true),
        _ => (format!("todo!(\"unhandled into_model conversion: {raw} -> {canon}\")"), None, false),
    }
}

/// Apply a leaf result used as a field value or map part: thread its error through `map_err`/`?`.
fn wrap(
    e: (String, Option<LeafErr>, bool),
    variant: &str,
    errs: &mut Vec<(String, String)>,
    numeric: &mut bool,
) -> (String, bool) {
    let (expr, err, known) = e;
    match err {
        None => (expr, known),
        Some(LeafErr::Numeric) => {
            *numeric = true;
            (format!("{expr}?"), known)
        }
        Some(LeafErr::Named(ty)) => {
            errs.push((variant.to_owned(), ty));
            (format!("{expr}.map_err(E::{variant})?"), known)
        }
    }
}

/// The full conversion for one canonical field: the value expression plus any error variants and
/// nested `(raw, canonical)` pairs it pulls in. Handles `Option`/`Vec`/`BTreeMap` wrappers around a
/// [`leaf`] conversion.
fn convert(
    ctx: &Ctx,
    raw: &str,
    canon: &str,
    field: &str,
    access: &str,
    errs: &mut Vec<(String, String)>,
    numeric: &mut bool,
    nested: &mut Vec<(String, String)>,
) -> (String, bool) {
    let v = variant_of(field);
    let field_lit = format!("\"{field}\"");

    // Fee rates: `btc_per_kb` yields `Option<FeeRate>`, so it interacts with the `Option` wrapper.
    if canon == "Option<FeeRate>" {
        errs.push((v.clone(), "amount::ParseAmountError".to_owned()));
        let expr = if raw.starts_with("Option<") {
            format!("{access}.map(|f| crate::btc_per_kb(f).map_err(E::{v})).transpose()?.flatten()")
        } else {
            format!("crate::btc_per_kb({access}).map_err(E::{v})?")
        };
        return (expr, true);
    }

    // Maps: convert key and value, both potentially fallible (nested value `into_model`).
    if let Some((kc, uc)) = map_pair(canon) {
        if let Some(rv) = strip_wrap(raw, "std::collections::BTreeMap<String, ")
            .or_else(|| strip_wrap(raw, "BTreeMap<String, "))
        {
            let kpart = wrap(
                leaf(ctx, "String", kc, "k", &field_lit, nested),
                &format!("{v}Key"),
                errs,
                numeric,
            );
            let vpart = wrap(
                leaf(ctx, rv, uc, "v", &field_lit, nested),
                &format!("{v}Value"),
                errs,
                numeric,
            );
            let expr = format!(
                "{access}.into_iter().map(|(k, v)| Ok::<_, E>(({}, {}))).collect::<Result<std::collections::BTreeMap<_, _>, _>>()?",
                kpart.0, vpart.0
            );
            return (expr, kpart.1 && vpart.1);
        }
    }

    // Lists: raw side is `Vec<R>` or `Option<Vec<R>>` (the latter defaulted to empty).
    if let Some(cu) = strip_wrap(canon, "Vec<") {
        let (ru, base) = match strip_wrap(raw, "Vec<") {
            Some(ru) => (Some(ru), access.to_owned()),
            None => match strip_wrap(raw, "Option<").and_then(|o| strip_wrap(o, "Vec<")) {
                Some(ru) => (Some(ru), format!("{access}.unwrap_or_default()")),
                None => (None, access.to_owned()),
            },
        };
        if let Some(ru) = ru {
            let (e, err, known) = leaf(ctx, ru, cu, "x", &field_lit, nested);
            if e == "x" {
                return (base, true); // Vec<T> -> Vec<T>.
            }
            return match err {
                None => (format!("{base}.into_iter().map(|x| {e}).collect()"), known),
                Some(LeafErr::Numeric) => {
                    *numeric = true;
                    (
                        format!("{base}.into_iter().map(|x| {e}).collect::<Result<Vec<_>, _>>()?"),
                        known,
                    )
                }
                Some(LeafErr::Named(ty)) => {
                    errs.push((v.clone(), ty));
                    (
                        format!(
                            "{base}.into_iter().map(|x| {e}.map_err(E::{v})).collect::<Result<Vec<_>, _>>()?"
                        ),
                        known,
                    )
                }
            };
        }
    }

    // Optionals.
    if let Some(cu) = strip_wrap(canon, "Option<") {
        if let Some(ru) = strip_wrap(raw, "Option<") {
            let (e, err, known) = leaf(ctx, ru, cu, "x", &field_lit, nested);
            if e == "x" {
                return (access.to_owned(), true); // Option<T> -> Option<T>.
            }
            let mapped = match fn_path(&e, "x") {
                Some(p) => format!("{access}.map({p})"),
                None => format!("{access}.map(|x| {e})"),
            };
            return match err {
                None => (mapped, known),
                Some(LeafErr::Numeric) => {
                    *numeric = true;
                    (format!("{mapped}.transpose()?"), known)
                }
                Some(LeafErr::Named(ty)) => {
                    errs.push((v.clone(), ty));
                    (format!("{mapped}.transpose().map_err(E::{v})?"), known)
                }
            };
        }
        // R -> Option<U>: convert `R -> U` (recursing so `Vec`/`Map` inners work) and wrap `Some`.
        let (inner, known) = convert(ctx, raw, cu, field, access, errs, numeric, nested);
        return (format!("Some({inner})"), known);
    }

    // Option<R> -> bare U: convert the inner value, defaulting to `U::default()` when absent.
    if let Some(ru) = strip_wrap(raw, "Option<") {
        if ru == canon {
            return (format!("{access}.unwrap_or_default()"), true);
        }
        let (e, err, known) = leaf(ctx, ru, canon, "x", &field_lit, nested);
        return match err {
            None => (format!("{access}.map(|x| {e}).unwrap_or_default()"), known),
            Some(LeafErr::Numeric) => {
                *numeric = true;
                (format!("{access}.map(|x| {e}).transpose()?.unwrap_or_default()"), known)
            }
            Some(LeafErr::Named(ty)) => {
                errs.push((v.clone(), ty));
                (
                    format!(
                        "{access}.map(|x| {e}).transpose().map_err(E::{v})?.unwrap_or_default()"
                    ),
                    known,
                )
            }
        };
    }

    // Bare scalar.
    wrap(leaf(ctx, raw, canon, access, &field_lit, nested), &v, errs, numeric)
}

/// The parsed shape of a canonical `crate::model` type.
enum CanonShape {
    /// `pub struct X { .. }`: `(field_name, field_type)` pairs in declaration order.
    Struct(Vec<(String, String)>),
    /// `pub struct X(pub Inner);`, the inner Rust type.
    Newtype(String),
}

/// Parse the canonical model type `name` from the model source under `model_dir`.
fn parse_canonical(model_dir: &Path, name: &str) -> Option<CanonShape> {
    let brace = format!("pub struct {name} {{");
    let tuple = format!("pub struct {name}(pub ");
    for entry in fs::read_dir(model_dir).ok()?.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let src = fs::read_to_string(&path).ok()?;
        if let Some(start) = src.find(&tuple) {
            let rest = &src[start + tuple.len()..];
            let end = rest.find(");")?;
            return Some(CanonShape::Newtype(rest[..end].trim().to_owned()));
        }
        let Some(start) = src.find(&brace) else { continue };
        let body = &src[start + brace.len()..];
        let end = body.find("\n}")?;
        let mut fields = Vec::new();
        for line in body[..end].lines() {
            let line = line.trim();
            let Some(rest) = line.strip_prefix("pub ") else { continue };
            let Some((fname, tail)) = rest.split_once(':') else { continue };
            // type is up to the field-terminating comma, ignoring a trailing `// comment`.
            let ty = tail.split("//").next().unwrap_or("").trim().trim_end_matches(',').trim();
            fields.push((fname.trim().to_owned(), ty.to_owned()));
        }
        return Some(CanonShape::Struct(fields));
    }
    None
}

/// Find the raw field a canonical field maps to: a [`FIELD_ALIAS`] override, else normalized name.
fn match_raw<'a>(canon_type: &str, cfield: &str, rfields: &'a [RawField]) -> Option<&'a RawField> {
    if let Some((_, _, raw_name)) =
        FIELD_ALIAS.iter().find(|(t, f, _)| *t == canon_type && *f == cfield)
    {
        return rfields.iter().find(|f| f.name == *raw_name);
    }
    rfields.iter().find(|f| norm(&f.name) == norm(cfield))
}

/// Build the constructor body for a canonical struct. `lookup(cfield) -> (access expr, raw type)`
/// resolves where each canonical field's value comes from (shared by the struct and the
/// newtype->one-field-struct shapes).
fn struct_ctor(
    ctx: &Ctx,
    canon_type: &str,
    cfields: &[(String, String)],
    errs: &mut Vec<(String, String)>,
    numeric: &mut bool,
    nested: &mut Vec<(String, String)>,
    mut lookup: impl FnMut(&str) -> Option<(String, String)>,
) -> String {
    let mut ctor = String::new();
    for (cfield, cty) in cfields {
        // Known-buggy canonical field: route through a compatibility shim.
        if let Some(c) = COMPAT.iter().find(|c| c.canon_type == canon_type && c.field == cfield) {
            let access = lookup(cfield).map(|(a, _)| a).unwrap_or_else(|| "()".into());
            ctor.push_str(&format!(
                "            // codegen-correct conversion ({}): `{}`.\n            // TODO(compat): {}. Routed through a wrong placeholder in `compatibility` to compile.\n",
                c.input, c.correct, c.reason
            ));
            ctor.push_str(&format!("            {cfield}: compatibility::{}({access}),\n", c.shim));
            continue;
        }
        match lookup(cfield) {
            Some((access, rty)) => {
                let (expr, known) = convert(ctx, &rty, cty, cfield, &access, errs, numeric, nested);
                if !known {
                    ctor.push_str(&format!(
                        "            // TODO(codegen): no rule for `{rty}` -> `{cty}`.\n"
                    ));
                }
                ctor.push_str(&format!("            {cfield}: {expr},\n"));
            }
            None if cty.starts_with("Option<") => ctor.push_str(&format!(
                "            {cfield}: None, // no raw field; canonical is optional\n"
            )),
            None if cty.starts_with("Vec<") => ctor.push_str(&format!(
                "            {cfield}: vec![], // no raw field; canonical is a list\n"
            )),
            None if cty.starts_with("BTreeMap<") || cty.starts_with("std::collections::BTreeMap<") =>
                ctor.push_str(&format!(
                    "            {cfield}: Default::default(), // no raw field; canonical is a map\n"
                )),
            None => ctor.push_str(&format!(
                "            // TODO(codegen): canonical field `{cfield}: {cty}` has no raw counterpart; needs manual reconstruction.\n            {cfield}: todo!(\"no raw field for `{cfield}`\"),\n"
            )),
        }
    }
    ctor
}

/// Emit one `(raw, canonical)` pair's `into_model` impl plus its error enum. Returns the source, the
/// error-enum name (for the module re-export), and any nested pairs to also emit.
fn emit_type(
    ctx: &Ctx,
    raw_name: &str,
    canon_name: &str,
) -> (String, String, String, Vec<(String, String)>) {
    let err = format!("{canon_name}Error");
    let info = ctx.raw[raw_name];

    let Some(canon) = parse_canonical(ctx.model_dir, canon_name) else {
        return (
            format!("compile_error!(\"codegen: canonical model type `{canon_name}` not found for into_model\");\n\n"),
            String::new(),
            err,
            vec![],
        );
    };

    let mut errs: Vec<(String, String)> = Vec::new();
    let mut numeric = false;
    let mut nested: Vec<(String, String)> = Vec::new();

    let body = match (&info.shape, &canon) {
        (RawShape::Struct(rfields), CanonShape::Struct(cfields)) => {
            let ctor =
                struct_ctor(ctx, canon_name, cfields, &mut errs, &mut numeric, &mut nested, |cf| {
                    match_raw(canon_name, cf, rfields)
                        .map(|rf| (format!("self.{}", rf.name), rf.ty.clone()))
                });
            format!("Ok(model::{canon_name} {{\n{ctor}        }})")
        }
        (RawShape::Newtype(rinner), CanonShape::Newtype(cinner)) => {
            let (expr, known) = convert(
                ctx, rinner, cinner, "inner", "self.0", &mut errs, &mut numeric, &mut nested,
            );
            let todo = if known {
                String::new()
            } else {
                format!("// TODO(codegen): no rule for `{rinner}` -> `{cinner}`.\n        ")
            };
            format!("{todo}Ok(model::{canon_name}({expr}))")
        }
        // A raw newtype around a list/scalar feeding a canonical struct with a single field.
        (RawShape::Newtype(rinner), CanonShape::Struct(cfields)) if cfields.len() == 1 => {
            let ctor =
                struct_ctor(ctx, canon_name, cfields, &mut errs, &mut numeric, &mut nested, |_| {
                    Some(("self.0".to_owned(), rinner.clone()))
                });
            format!("Ok(model::{canon_name} {{\n{ctor}        }})")
        }
        // A raw struct feeding a canonical newtype: if the newtype wraps a model struct, build it
        // inline from the raw fields. Otherwise it is a genuine reconstruction (e.g. `Transaction`
        // from decoded fields); leave that honest and half-done so the gap is visible.
        (RawShape::Struct(rfields), CanonShape::Newtype(cinner)) => {
            match parse_canonical(ctx.model_dir, cinner) {
                Some(CanonShape::Struct(cfields)) => {
                    let ctor = struct_ctor(
                        ctx, cinner, &cfields, &mut errs, &mut numeric, &mut nested, |cf| {
                            match_raw(cinner, cf, rfields)
                                .map(|rf| (format!("self.{}", rf.name), rf.ty.clone()))
                        },
                    );
                    format!("Ok(model::{canon_name}(model::{cinner} {{\n{ctor}        }}))")
                }
                _ => format!(
                    "// TODO(codegen): reconstruct `{cinner}` from the decoded fields of `{raw_name}`.\n        // The canonical `{canon_name}` is a newtype around `{cinner}`, but the raw response is the\n        // already-decoded value with no single field to convert. Needs manual assembly.\n        Ok(model::{canon_name}(todo!(\"reconstruct {cinner} from decoded fields\")))"
                ),
            }
        }
        // A raw newtype feeding a multi-field canonical struct can't be bridged structurally.
        (RawShape::Newtype(_), CanonShape::Struct(_)) => format!(
            "compile_error!(\"codegen: cannot map newtype `{raw_name}` onto multi-field struct `{canon_name}`\");\n        unreachable!()"
        ),
    };

    let mut imp = String::new();
    imp.push_str(&format!("impl {raw_name} {{\n"));
    imp.push_str("    /// Converts the raw type into the version-nonspecific model type.\n");
    imp.push_str(&format!(
        "    pub fn into_model(self) -> Result<model::{canon_name}, {err}> {{\n"
    ));
    imp.push_str(&format!("        use {err} as E;\n\n"));
    imp.push_str(&format!("        {body}\n    }}\n}}\n\n"));

    let error_enum = emit_error_enum(&err, canon_name, &errs, numeric);
    (imp, error_enum, err, nested)
}

/// Emit the error enum for a converted type: one variant per named parse error, plus a shared
/// `Numeric` if any narrowing happened.
fn emit_error_enum(err: &str, canon: &str, named: &[(String, String)], numeric: bool) -> String {
    let mut out = String::new();
    out.push_str(&format!("/// Error when converting a `{canon}` type into the model type.\n"));
    out.push_str("#[derive(Debug)]\n");
    out.push_str(&format!("pub enum {err} {{\n"));
    for (variant, ty) in named {
        out.push_str(&format!(
            "    /// Conversion of the `{variant}` field failed.\n    {variant}({ty}),\n"
        ));
    }
    if numeric {
        out.push_str("    /// Conversion of a numeric type to the expected type failed.\n    Numeric(crate::NumericError),\n");
    }
    out.push_str("}\n\n");

    // An infallible conversion has an uninhabited (empty) error enum, so `match *self {}` never
    // touches the formatter; name it `_f` to avoid an unused-variable warning.
    let fparam = if named.is_empty() && !numeric { "_f" } else { "f" };
    out.push_str(&format!(
        "impl fmt::Display for {err} {{\n    fn fmt(&self, {fparam}: &mut fmt::Formatter) -> fmt::Result {{\n        match *self {{\n"
    ));
    for (variant, _) in named {
        out.push_str(&format!(
            "            Self::{variant}(ref e) => write_err!(f, \"conversion of the `{variant}` field failed\"; e),\n"
        ));
    }
    if numeric {
        out.push_str("            Self::Numeric(ref e) => write_err!(f, \"numeric conversion failed\"; e),\n");
    }
    out.push_str("        }\n    }\n}\n\n");

    out.push_str(&format!(
        "#[cfg(feature = \"std\")]\nimpl std::error::Error for {err} {{\n    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {{\n        match *self {{\n"
    ));
    for (variant, _) in named {
        out.push_str(&format!("            Self::{variant}(ref e) => Some(e),\n"));
    }
    if numeric {
        out.push_str("            Self::Numeric(ref e) => Some(e),\n");
    }
    out.push_str("        }\n    }\n}\n\n");

    if numeric {
        out.push_str(&format!(
            "impl From<crate::NumericError> for {err} {{\n    fn from(e: crate::NumericError) -> Self {{ Self::Numeric(e) }}\n}}\n\n"
        ));
    }
    out
}

/// The result of generating a category's `into.rs`.
pub struct Generated {
    pub source: String,
    /// Error-enum names to re-export from the category `mod.rs`.
    pub error_names: Vec<String>,
    /// Whether the category had any whitelisted root (else it stays a flat module).
    pub has_roots: bool,
}

/// Generate the `into.rs` for one category: walk the type graph from every whitelisted root in
/// `all` (every response type in the category), emitting each reachable `(raw, canonical)` pair.
pub fn generate_category(
    version: &str,
    category: &str,
    all: &[RawTypeInfo],
    model_dir: &Path,
) -> Generated {
    let ctx = Ctx { raw: all.iter().map(|i| (i.raw_name.clone(), i)).collect(), model_dir };

    let mut queue: VecDeque<(String, String)> = all
        .iter()
        .filter(|i| is_whitelisted(&i.raw_name))
        .map(|i| (i.raw_name.clone(), model_typename(&i.raw_name)))
        .collect();
    let has_roots = !queue.is_empty();

    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut emitted: Vec<(String, String)> = Vec::new(); // (raw_name, source)
    let mut error_names: Vec<String> = Vec::new();
    // Several raw types can map to the same canonical type (e.g. the ancestor/descendant verbose
    // mempool entries both become `MempoolEntry`); their error enum must be emitted only once.
    let mut emitted_errors: BTreeSet<String> = BTreeSet::new();

    while let Some((raw, canon)) = queue.pop_front() {
        if !seen.insert(raw.clone()) {
            continue;
        }
        if !ctx.raw.contains_key(&raw) {
            continue; // a nested type outside this category's response set; skip.
        }
        let (imp, error_enum, err, nested) = emit_type(&ctx, &raw, &canon);
        let mut src = imp;
        if emitted_errors.insert(err.clone()) {
            src.push_str(&error_enum);
            error_names.push(err);
        }
        emitted.push((raw, src));
        for pair in nested {
            queue.push_back(pair);
        }
    }

    emitted.sort_by(|a, b| a.0.cmp(&b.0));
    error_names.sort();

    let mut source = header(version, category);
    for (_, src) in emitted {
        source.push_str(&src);
    }

    Generated { source, error_names, has_roots }
}

/// The `into.rs` file header (module docs + imports).
fn header(version: &str, category: &str) -> String {
    format!(
        "// SPDX-License-Identifier: CC0-1.0\n\n\
         //! Auto-generated `into_model` conversions for Bitcoin Core `{version}` - {category}.\n//!\n\
         //! Generated by `codegen`. Do not edit by hand, re-run `just codegen` to regenerate. Each\n\
         //! conversion turns a raw response type (from the sibling `mod.rs`) into its shared,\n\
         //! version-nonspecific `crate::model` counterpart. Conversions the canonical types get\n\
         //! wrong are routed through `crate::v{version}::generated::compatibility`; see that file and\n\
         //! `corepc_bugs_backlog.md`.\n\n\
         // `unreachable_code` is allowed for the half-done `todo!()` reconstructions (see the\n\
         // per-site TODOs); they diverge, making later struct fields unreachable.\n\
         #![allow(unreachable_code, unused_imports)]\n\n\
         use core::fmt;\n\n\
         use bitcoin::address::{{self, NetworkUnchecked}};\n\
         use bitcoin::consensus::encode;\n\
         use bitcoin::error::UnprefixedHexError;\n\
         use bitcoin::hashes::{{hash160, sha256}};\n\
         use bitcoin::{{amount, block, hex, network, psbt, Address, Amount, Block, BlockHash, CompactTarget, FeeRate, Network, Psbt, ScriptBuf, Sequence, SignedAmount, Target, Transaction, TxMerkleNode, Txid, Weight, Work, Wtxid}};\n\n\
         use super::*;\n\
         use crate::error::write_err;\n\
         use crate::model;\n\
         use crate::v{version}::generated::compatibility;\n\n"
    )
}

/// Emit the shared `compatibility.rs`: the hand-shaped wrong-but-compilable shims that isolate
/// canonical-type bugs. Same content for every version (the override table lives in the generator).
pub fn emit_compatibility(version: &str) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "// SPDX-License-Identifier: CC0-1.0\n\n\
         //! Manual compatibility shims for `into_model` conversions that `codegen` produces\n\
         //! correctly but that do not match the (buggy) canonical `crate::model` types.\n//!\n\
         //! Emitted by `codegen` from a fixed override table, so every version carries the same\n\
         //! shims. Each one is a deliberate WRONG, compilable placeholder that isolates a known bug\n\
         //! in `types/` so the generated code keeps building. Each has a matching entry in\n\
         //! `corepc_bugs_backlog.md`. When a canonical type is fixed, drop the override and let\n\
         //! codegen emit the correct conversion inline (left commented at each call site).\n//!\n\
         //! Bitcoin Core `{version}`.\n\n\
         #![allow(unused_imports)]\n\n\
         use bitcoin::{{Address, Amount, BlockHash, FeeRate}};\n\n"
    ));
    for c in COMPAT {
        s.push_str(&format!(
            "/// WRONG placeholder: {reason}.\n///\n/// Codegen produces `{correct}` ({input}); the canonical field wrongly wants `{returns}`, so this\n/// discards the real value to compile. TODO: fix the canonical type, then delete this shim.\npub fn {shim}(_v: {input}) -> {returns} {{ {placeholder} }}\n\n",
            reason = c.reason,
            correct = c.correct,
            input = c.input,
            returns = c.returns,
            shim = c.shim,
            placeholder = c.placeholder,
        ));
    }
    s
}
