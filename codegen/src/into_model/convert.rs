// SPDX-License-Identifier: CC0-1.0

//! The conversion-expression engine: the per-`(raw, canonical)`-type-pair rules ([`leaf`]) and the
//! `Option`/`Vec`/`BTreeMap` wrapper handling around them ([`convert`]). Pure string generation.

use super::*;

/// Strip a generic wrapper: `strip_wrap("Option<X>", "Option<")` -> `Some("X")`.
pub(crate) fn strip_wrap<'a>(ty: &'a str, open: &str) -> Option<&'a str> {
    ty.strip_prefix(open)?.strip_suffix('>').map(str::trim)
}

/// If `expr` is `PATH(var)`, return `PATH` so `.map(|var| expr)` can become `.map(PATH)` (avoids a
/// `clippy::redundant_closure`). Only a bare function-path call qualifies, not `var.method()` or
/// `f(&var)`.
pub(crate) fn fn_path<'a>(expr: &'a str, var: &str) -> Option<&'a str> {
    let inner = expr.strip_suffix(&format!("({var})"))?;
    (!inner.is_empty() && !inner.contains(|c: char| !c.is_alphanumeric() && c != '_' && c != ':'))
        .then_some(inner)
}

/// Split a `BTreeMap<K, V>` (either path) into `(K, V)`, depth-aware on the top-level comma.
pub(crate) fn map_pair(ty: &str) -> Option<(&str, &str)> {
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
pub(crate) enum LeafErr {
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
pub(crate) fn leaf(
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
        // A nested raw struct whose canonical target is a whole rust-bitcoin value can't delegate to
        // an `into_model`. A `Transaction` is rebuilt by the shared `reconstruct` helper (bridging
        // the decoded fields through the curated `RawTransaction`); other targets have no helper yet,
        // so leave an honest field-level `todo!()` that only panics if Core returns the value.
        if canon == "Transaction" {
            return (
                format!("crate::reconstruct::transaction(&{var})"),
                Some(LeafErr::Named("crate::reconstruct::ReconstructError".to_owned())),
                true,
            );
        }
        if matches!(canon, "Block" | "Psbt" | "block::Header") {
            return (format!("todo!(\"reconstruct {canon} from decoded `{raw}`\")"), None, false);
        }
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
    // Core renders these model enums as a fixed lowercase/snake/kebab string; the model enum carries
    // the matching `serde(rename_all = ..)`, so deserialize the string straight into it.
    let enum_de = |ty: &str| {
        named(
            format!("serde_json::from_value::<model::{ty}>(serde_json::Value::String({var}))"),
            "serde_json::Error",
        )
    };
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
        // Wallet keys (WIF / base58 extended keys).
        ("String", "PrivateKey") => parse("PrivateKey", "key::FromWifError"),
        ("String", "PublicKey") => parse("PublicKey", "key::ParsePublicKeyError"),
        ("String", "Xpub") => parse("Xpub", "bip32::Error"),
        ("String", "Xpriv") => parse("Xpriv", "bip32::Error"),
        ("String", "bip32::Fingerprint") => parse("bip32::Fingerprint", "hex::HexToArrayError"),
        ("String", "bip32::DerivationPath") => parse("bip32::DerivationPath", "bip32::Error"),
        ("String", "sign_message::MessageSignature") =>
            parse("sign_message::MessageSignature", "sign_message::MessageSignatureError"),
        // Core string-enums -> model enums (deserialized via the model enum's serde rename).
        ("String", "AddressPurpose") => enum_de("AddressPurpose"),
        ("String", "TransactionCategory") => enum_de("TransactionCategory"),
        ("String", "Bip125Replaceable") => enum_de("Bip125Replaceable"),
        ("String", "Bip9SoftforkStatus") => enum_de("Bip9SoftforkStatus"),
        ("String", "SoftforkType") => enum_de("SoftforkType"),
        ("String", "ChainTipsStatus") => enum_de("ChainTipsStatus"),
        ("String", "ScriptType") => enum_de("ScriptType"),
        // `getwalletinfo.scanning` is `false` or `{duration, progress}`; the model is an untagged
        // enum that deserializes either shape straight from the raw `serde_json::Value`.
        ("serde_json::Value", "GetWalletInfoScanning") => named(
            format!("serde_json::from_value::<model::GetWalletInfoScanning>({var})"),
            "serde_json::Error",
        ),
        // Difficulty targets / work (unprefixed hex).
        ("String", "CompactTarget") => unprefixed("CompactTarget"),
        ("String", "Work") => unprefixed("Work"),
        ("String", "Target") => unprefixed("Target"),
        // Scripts and transactions (consensus/hex decode).
        ("String", "ScriptBuf") =>
            named(format!("ScriptBuf::from_hex(&{var})"), "hex::HexToBytesError"),
        ("String", "Vec<u8>") => named(format!("Vec::<u8>::from_hex(&{var})"), "hex::HexToBytesError"),
        ("String", "bitcoin::bip158::FilterHash") =>
            parse("bitcoin::bip158::FilterHash", "hex::HexToArrayError"),
        ("String", "Transaction") =>
            named(format!("encode::deserialize_hex::<Transaction>(&{var})"), "encode::FromHexError"),
        ("String", "Block") =>
            named(format!("encode::deserialize_hex::<Block>(&{var})"), "encode::FromHexError"),
        ("String", "block::Header") =>
            named(format!("encode::deserialize_hex::<block::Header>(&{var})"), "encode::FromHexError"),
        // Network name (`main`/`test`/`signet`/`regtest`); parsed the way `bitcoind` takes it.
        ("String", "Network") =>
            named(format!("Network::from_core_arg(&{var})"), "network::ParseNetworkError"),
        // Amounts: Core renders a `CAmount` as fractional BTC when floating, raw satoshis when an
        // integer (e.g. the `getblockstats` totals).
        ("f64", "Amount") => named(format!("Amount::from_btc({var})"), "amount::ParseAmountError"),
        ("i64", "Amount") => (format!("Amount::from_sat({var} as u64)"), None, true),
        ("f64", "SignedAmount") =>
            named(format!("SignedAmount::from_btc({var})"), "amount::ParseAmountError"),
        // An untyped percentile array element (`getblockstats.feerate_percentiles`) is a sat/vB
        // integer; `FeeRate::from_sat_per_vb` already yields the `Option<FeeRate>` the model holds.
        ("serde_json::Value", "Option<FeeRate>") =>
            (format!("{var}.as_i64().and_then(|n| FeeRate::from_sat_per_vb(n as u64))"), None, true),
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
        ("i64", "u16") => (
            format!(
                "u16::try_from({var}).map_err(|_| crate::NumericError::Overflow {{ value: {var}, field: {field_lit}.to_owned() }})"
            ),
            Some(LeafErr::Numeric),
            true,
        ),
        ("i64", "i32") => (
            format!(
                "i32::try_from({var}).map_err(|_| crate::NumericError::Overflow {{ value: {var}, field: {field_lit}.to_owned() }})"
            ),
            Some(LeafErr::Numeric),
            true,
        ),
        ("u64", "i64") => (
            format!(
                "i64::try_from({var}).map_err(|_| crate::NumericError::Overflow {{ value: {var} as i64, field: {field_lit}.to_owned() }})"
            ),
            Some(LeafErr::Numeric),
            true,
        ),
        // `usize`/`isize`: Core JSON numbers the model happens to type as machine-sized.
        ("i64", "usize") => (
            format!(
                "usize::try_from({var}).map_err(|_| crate::NumericError::Overflow {{ value: {var}, field: {field_lit}.to_owned() }})"
            ),
            Some(LeafErr::Numeric),
            true,
        ),
        ("u64", "usize") => (
            format!(
                "usize::try_from({var}).map_err(|_| crate::NumericError::Overflow {{ value: {var} as i64, field: {field_lit}.to_owned() }})"
            ),
            Some(LeafErr::Numeric),
            true,
        ),
        ("i64", "isize") => (
            format!(
                "isize::try_from({var}).map_err(|_| crate::NumericError::Overflow {{ value: {var}, field: {field_lit}.to_owned() }})"
            ),
            Some(LeafErr::Numeric),
            true,
        ),
        // A `CAmount` of satoshis (not BTC); `prioritisetransaction` `fee_delta`, etc.
        ("i64", "SignedAmount") => (format!("SignedAmount::from_sat({var})"), None, true),
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

/// Canonical leaf types we can fill with `Default::default()` when an optional raw value is absent.
/// Everything else (addresses, hashes, scripts, keys, ...) has no meaningful default, so an absent
/// value becomes a `MissingField` error instead.
pub(crate) fn has_default(canon: &str) -> bool {
    // `Amount`/`SignedAmount` default to zero, which is what the hand-written types use (via
    // `#[serde(default)]`) for amounts Core omits, e.g. `fee` on a non-`send` wallet transaction.
    matches!(
        canon,
        "String"
            | "bool"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "usize"
            | "i32"
            | "i64"
            | "f64"
            | "Amount"
            | "SignedAmount"
    ) || canon.starts_with("Vec<")
}

/// The full conversion for one canonical field: the value expression plus any error variants and
/// nested `(raw, canonical)` pairs it pulls in. Handles `Option`/`Vec`/`BTreeMap` wrappers around a
/// [`leaf`] conversion.
pub(crate) fn convert(
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

    // Fee rates. Core reports a feerate either as a BTC/kvB float (`estimatesmartfee`, ...) or as a
    // sat/vB integer (`getblockstats`). An integer goes straight through `FeeRate::from_sat_per_vb`
    // (itself returning `Option`); a float goes through `btc_per_kb`. Both land in `Option<FeeRate>`.
    if canon == "Option<FeeRate>" {
        let raw_inner = strip_wrap(raw, "Option<").unwrap_or(raw);
        if raw_inner == "i64" || raw_inner == "u64" {
            let expr = if raw.starts_with("Option<") {
                format!("{access}.and_then(|f| FeeRate::from_sat_per_vb(f as u64))")
            } else {
                format!("FeeRate::from_sat_per_vb({access} as u64)")
            };
            return (expr, true);
        }
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
            // `Option<Vec<R>>` -> `Option<Vec<U>>`: the element conversion can be fallible, and `?`
            // cannot cross the `Option::map` closure, so build the element `collect()` as a `Result`
            // and lift it back out with `transpose()`.
            if let (Some(cinner), Some(rinner)) = (strip_wrap(cu, "Vec<"), strip_wrap(ru, "Vec<")) {
                let (e, err, known) = leaf(ctx, rinner, cinner, "y", &field_lit, nested);
                if e == "y" {
                    return (access.to_owned(), true); // Option<Vec<T>> -> Option<Vec<T>>.
                }
                return match err {
                    None =>
                        (format!("{access}.map(|x| x.into_iter().map(|y| {e}).collect())"), known),
                    Some(LeafErr::Numeric) => {
                        *numeric = true;
                        (
                            format!(
                                "{access}.map(|x| x.into_iter().map(|y| {e}).collect::<Result<Vec<_>, _>>()).transpose()?"
                            ),
                            known,
                        )
                    }
                    Some(LeafErr::Named(ty)) => {
                        errs.push((v.clone(), ty));
                        (
                            format!(
                                "{access}.map(|x| x.into_iter().map(|y| {e}.map_err(E::{v})).collect::<Result<Vec<_>, _>>()).transpose()?"
                            ),
                            known,
                        )
                    }
                };
            }
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

    // Option<R> -> bare U. When U has a `Default` we fill it in for an absent value; otherwise (an
    // `Address`, hash, key, ...) Core dropping the field is an error, not a fabricated default.
    if let Some(ru) = strip_wrap(raw, "Option<") {
        let tail = if has_default(canon) {
            ".unwrap_or_default()".to_owned()
        } else {
            errs.push((format!("{v}Missing"), "crate::MissingField".to_owned()));
            format!(".ok_or(E::{v}Missing(crate::MissingField {{ field: {field_lit} }}))?")
        };
        if ru == canon {
            return (format!("{access}{tail}"), true);
        }
        let (e, err, known) = leaf(ctx, ru, canon, "x", &field_lit, nested);
        let mapped = match fn_path(&e, "x") {
            Some(p) => format!("{access}.map({p})"),
            None => format!("{access}.map(|x| {e})"),
        };
        return match err {
            None => (format!("{mapped}{tail}"), known),
            Some(LeafErr::Numeric) => {
                *numeric = true;
                (format!("{mapped}.transpose()?{tail}"), known)
            }
            Some(LeafErr::Named(ty)) => {
                errs.push((v.clone(), ty));
                (format!("{mapped}.transpose().map_err(E::{v})?{tail}"), known)
            }
        };
    }

    // Bare scalar.
    wrap(leaf(ctx, raw, canon, access, &field_lit, nested), &v, errs, numeric)
}
