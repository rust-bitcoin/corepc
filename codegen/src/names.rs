// SPDX-License-Identifier: CC0-1.0

//! Identifier conversion: turn JSON-RPC method/field names into idiomatic Rust idents.
//!
//! Core's names are separatorless lowercase compounds (`getblockheader`, `bestblockhash`) with no
//! character-level rule for the boundaries, so one curated [`WORDS`] list drives every splitter
//! ([`method_to_pascal`], [`method_to_snake`], [`to_rust_field`], [`to_pascal`], all via
//! [`greedy_split`]). One list means one place to add a word and no chance of two splitters
//! disagreeing. [`WORDS`] is documented at its definition.

/// Words used to split method and field names (`getblockheader` -> `Get` + `Block` + `Header`),
/// sorted longest-first at use time so `blockchain` wins over `block`.
///
/// Prefer the singular; the clean-boundary check in [`greedy_split`] splits a plural from its
/// singular + `s`, so a plural is listed only where that fails (`blocks`, `fees`, `names`,
/// `networks`). List a compound only when splitting it would be wrong (`network`, not `NetWork`).
/// A few words (`sigops`, `elapsed`) are listed whole to defend their tail against a short word
/// like `ps`.
pub static WORDS: &[&str] = &[
    "abandon",
    "abort",
    "absolute",
    "accept",
    "accounts",
    "active",
    "activity",
    "add",
    "added",
    "addr",
    "address",
    "addresses",
    "all",
    "amount",
    "analyze",
    "ancestor",
    "api",
    "automatic",
    "backup",
    "balance",
    "ban",
    "banned",
    "bare",
    "best",
    "bip125",
    "bitcoin",
    "block",
    "blockchain",
    "blocks",
    "broadcast",
    "bump",
    "burn",
    "by",
    "bytes",
    "carrier",
    "chain",
    "challenge",
    "change",
    "clear",
    "cluster",
    "coin",
    "combine",
    "conf",
    "confirmations",
    "connection",
    "control",
    "convert",
    "count",
    "create",
    "current",
    "data",
    "decode",
    "delete",
    "delta",
    "deployment",
    "depth",
    "derive",
    "desc",
    "descendant",
    "descriptor",
    "diagram",
    "difficulty",
    "dir",
    "disconnect",
    "disk",
    "display",
    "download",
    "dump",
    "echo",
    "effective",
    "elapsed",
    "encrypt",
    "entries",
    "entry",
    "enumerate",
    "error",
    "estimate",
    "estimated",
    "fee",
    "fees",
    "filter",
    "final",
    "finalize",
    "first",
    "flag",
    "for",
    "from",
    "full",
    "fund",
    "funded",
    "generate",
    "get",
    "group",
    "groupings",
    "hash",
    "hd",
    "header",
    "height",
    "help",
    "hex",
    "import",
    "included",
    "incremental",
    "index",
    "info",
    "initial",
    "interface",
    "internal",
    "invalidate",
    "ipc",
    "join",
    "json",
    "key",
    "keypool",
    "label",
    "last",
    "limit",
    "list",
    "load",
    "loaded",
    "lock",
    "locktime",
    "locktimes",
    "log",
    "logging",
    "man",
    "many",
    "max",
    "median",
    "memory",
    "mempool",
    "merkle",
    "message",
    "migrate",
    "min",
    "mining",
    "mock",
    "modified",
    "multipath",
    "multisig",
    "name",
    "names",
    "net",
    "network",
    "networks",
    "new",
    "next",
    "node",
    "nonce",
    "num",
    "object",
    "offset",
    "oldest",
    "only",
    "open",
    "orphan",
    "out",
    "output",
    "package",
    "passphrase",
    "path",
    "peer",
    "permit",
    "pool",
    "pooled",
    "precious",
    "prev",
    "previous",
    "prevout",
    "prioritise",
    "prioritised",
    "priority",
    "priv",
    "private",
    "process",
    "processed",
    "progress",
    "proof",
    "prune",
    "pruned",
    "pruning",
    "ps",
    "psbt",
    "pubkey",
    "pubnonce",
    "queue",
    "rate",
    "raw",
    "rbf",
    "reachable",
    "received",
    "recipient",
    "reconsider",
    "refill",
    "relay",
    "required",
    "rescan",
    "restore",
    "result",
    "root",
    "rpc",
    "save",
    "scan",
    "scanning",
    "scheduler",
    "script",
    "send",
    "sequence",
    "set",
    "sign",
    "signalling",
    "signer",
    "signet",
    "sigops",
    "simulate",
    "since",
    "size",
    "smart",
    "spending",
    "stamp",
    "start",
    "state",
    "stats",
    "status",
    "stop",
    "stripped",
    "submit",
    "success",
    "sync",
    "taproot",
    "target",
    "template",
    "test",
    "time",
    "timestamp",
    "tips",
    "to",
    "total",
    "transaction",
    "tx",
    "txid",
    "type",
    "type_",
    "unbroadcast",
    "unconfirmed",
    "unload",
    "unlock",
    "unspent",
    "update",
    "upgrade",
    "upload",
    "uptime",
    "used",
    "utxo",
    "validate",
    "validation",
    "value",
    "verification",
    "verify",
    "version",
    "vout",
    "vsize",
    "wait",
    "wallet",
    "warnings",
    "watch",
    "weight",
    "with",
    "witness",
    "work",
    "written",
    "wtxid",
    "zmq",
];

/// Rust keywords that need an underscore suffix when they appear as identifiers.
const RUST_KEYWORDS: &[(&str, &str)] = &[
    ("type", "type_"),
    ("match", "match_"),
    ("ref", "ref_"),
    ("self", "self_"),
    ("mod", "mod_"),
    ("async", "async_"),
    ("await", "await_"),
    ("use", "use_"),
];

/// [`WORDS`] sorted longest-first, the order the greedy splitter needs (`blockchain` before
/// `block`). Built fresh per call; the list is small.
fn sorted_words() -> Vec<&'static str> {
    let mut sorted = WORDS.to_vec();
    sorted.sort_by_key(|w| std::cmp::Reverse(w.len()));
    sorted
}

/// Convert an RPC method name to PascalCase using the [`WORDS`] list.
pub fn method_to_pascal(name: &str) -> String {
    pascal_pieces(&name.to_ascii_lowercase(), &sorted_words())
}

/// Convert an RPC method name to snake_case (the Rust function-name convention).
pub fn method_to_snake(name: &str) -> String {
    let pascal = method_to_pascal(name);
    let mut out = String::with_capacity(pascal.len() + 4);
    for (i, ch) in pascal.char_indices() {
        if i > 0 && ch.is_ascii_uppercase() {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    rust_keyword_safe(&out)
}

/// Convert name to snake_case using the [`WORDS`] list.
/// - dashes become underscores and camelCase gains underscores at lower-to-upper transitions;
/// - every underscore-separated segment is then word-split, so compounds are fully broken
///   (`txoutset_hash` -> `tx_out_set_hash`, `chainwork` -> `chain_work`).
///
/// This keeps field identifiers consistent with the strongly typed `crate::model` names. The raw
/// struct keeps a `#[serde(rename)]` back to the original wire key, so the wire format is unchanged.
pub fn to_rust_field(name: &str) -> String {
    let cleaned = name.replace('-', "_");
    let de_camel = decamel(&cleaned);
    let words = sorted_words();
    let split = de_camel
        .split('_')
        .filter(|s| !s.is_empty())
        .flat_map(|seg| greedy_split(&seg.to_ascii_lowercase(), &words))
        .collect::<Vec<_>>()
        .join("_");
    rust_keyword_safe(&split)
}

/// Convert name to PascalCase.
pub fn to_pascal(s: &str) -> String {
    let cleaned = s.replace('-', "_");
    let de_camel = decamel(&cleaned);
    if de_camel.contains('_') {
        return de_camel.split('_').filter(|p| !p.is_empty()).map(capitalise_first).collect();
    }
    if cleaned.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) && !cleaned.is_empty()
    {
        return pascal_pieces(&cleaned, &sorted_words());
    }
    capitalise_first(&cleaned)
}

/// Split an all-lowercase compound into pieces. At each position take the longest word ending on a
/// clean boundary (string end or the start of another word); otherwise consume the run up to the
/// next clean word start as one piece.
///
/// The boundary check rejects a short word that is a prefix of a longer unlisted word: `priv` in
/// `private` would strand `atebroadcast`, so `abortprivatebroadcast` stays whole rather than
/// becoming `PrivAtebroadcast`, while `priv` in `privkey` is kept (followed by `key`).
fn greedy_split(name: &str, sorted_words: &[&str]) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let bytes = name.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if let Some(w) = clean_word(name, i, sorted_words) {
            parts.push(w.to_owned());
            i += w.len();
            continue;
        }
        let mut j = i + 1;
        while j < bytes.len() && clean_word(name, j, sorted_words).is_none() {
            j += 1;
        }
        parts.push(name[i..j].to_owned());
        i = j;
    }
    parts
}

/// Split a compound and PascalCase each piece (`getblock` -> `GetBlock`).
fn pascal_pieces(name: &str, sorted_words: &[&str]) -> String {
    greedy_split(name, sorted_words).iter().map(|p| capitalise_first(p)).collect()
}

/// The first word from the list that matches `haystack` starting at `at`, ignoring boundaries.
fn match_word<'a>(haystack: &'a str, at: usize, sorted_words: &[&'a str]) -> Option<&'a str> {
    sorted_words
        .iter()
        .copied()
        .find(|w| haystack.len() >= at + w.len() && &haystack[at..at + w.len()] == *w)
}

/// Longest word matching at `at` whose end is a clean boundary: the end of the string, or the
/// start of another word. Rejects prefix-shadowing matches (`priv` inside `private`) that would
/// otherwise strand a non-word remainder.
fn clean_word<'a>(haystack: &'a str, at: usize, sorted_words: &[&'a str]) -> Option<&'a str> {
    sorted_words.iter().copied().find(|w| {
        let end = at + w.len();
        haystack.len() >= end
            && &haystack[at..end] == *w
            && (end == haystack.len() || match_word(haystack, end, sorted_words).is_some())
    })
}

/// Uppercase the first character of `s`, leaving the rest unchanged.
fn capitalise_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// Insert underscores at camelCase boundaries and lowercase the string (`maxFee` -> `max_fee`).
fn decamel(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            if !out.is_empty() && out.chars().last().is_some_and(|p| p.is_ascii_lowercase()) {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// Append an underscore to `s` if it is a Rust keyword (`type` -> `type_`).
fn rust_keyword_safe(s: &str) -> String {
    for &(kw, replacement) in RUST_KEYWORDS {
        if s == kw {
            return replacement.to_owned();
        }
    }
    s.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_pascal_simple_compounds() {
        assert_eq!(method_to_pascal("getbestblockhash"), "GetBestBlockHash");
        assert_eq!(method_to_pascal("getblockcount"), "GetBlockCount");
        assert_eq!(method_to_pascal("getblockheader"), "GetBlockHeader");
    }

    #[test]
    fn method_pascal_resists_plural_shadow() {
        assert_eq!(method_to_pascal("getblockstats"), "GetBlockStats");
        assert_eq!(method_to_pascal("getchaintxstats"), "GetChainTxStats");
    }

    #[test]
    fn method_pascal_handles_split_compounds() {
        assert_eq!(method_to_pascal("gettxout"), "GetTxOut");
        assert_eq!(method_to_pascal("gettxoutsetinfo"), "GetTxOutSetInfo");
        assert_eq!(method_to_pascal("loadtxoutset"), "LoadTxOutSet");
    }

    #[test]
    fn method_pascal_handles_unknown_chunks() {
        assert_eq!(method_to_pascal("reconsiderblock"), "ReconsiderBlock");
        assert_eq!(
            method_to_pascal("syncwithvalidationinterfacequeue"),
            "SyncWithValidationInterfaceQueue"
        );
    }

    #[test]
    fn method_snake_round_trips() {
        assert_eq!(method_to_snake("getbestblockhash"), "get_best_block_hash");
        assert_eq!(method_to_snake("sendrawtransaction"), "send_raw_transaction");
        assert_eq!(method_to_snake("invalidateblock"), "invalidate_block");
        assert_eq!(method_to_snake("setmocktime"), "set_mock_time");
        assert_eq!(method_to_snake("getopenrpc"), "get_open_rpc");
    }

    #[test]
    fn fields_split_correctly() {
        assert_eq!(to_rust_field("bestblockhash"), "best_block_hash");
        assert_eq!(to_rust_field("maxfeerate"), "max_fee_rate");
        assert_eq!(to_rust_field("maxburnamount"), "max_burn_amount");
        assert_eq!(to_rust_field("verificationprogress"), "verification_progress");
        assert_eq!(to_rust_field("max_burn_amount"), "max_burn_amount");
        assert_eq!(to_rust_field("maxBurnAmount"), "max_burn_amount");
    }

    #[test]
    fn fields_avoid_keyword_collisions() {
        assert_eq!(to_rust_field("type"), "type_");
        assert_eq!(to_rust_field("ref"), "ref_");
    }

    #[test]
    fn pascal_helper_uses_word_list_for_lowercase_compounds() {
        assert_eq!(to_pascal("scanobjects"), "ScanObjects");
        assert_eq!(to_pascal("blockheader"), "BlockHeader");
    }

    #[test]
    fn pascal_pieces_agrees_with_greedy_split() {
        let words = sorted_words();
        for name in ["getblockheader", "gettxoutsetinfo", "reconsiderblock", "bestblockhash"] {
            let joined: String =
                greedy_split(name, &words).iter().map(|p| capitalise_first(p)).collect();
            assert_eq!(pascal_pieces(name, &words), joined, "mismatch on {name}");
        }
    }

    #[test]
    fn plural_and_singular_coexist() {
        assert_eq!(to_rust_field("blockstats"), "block_stats");
        assert_eq!(to_rust_field("chaintxstats"), "chain_tx_stats");
        assert_eq!(method_to_snake("scanblocks"), "scan_blocks");
        assert_eq!(method_to_pascal("getblockstats"), "GetBlockStats");
    }

    #[test]
    fn longest_word_wins_over_shorter_prefix() {
        assert_eq!(method_to_pascal("getblockchaininfo"), "GetBlockchainInfo");
    }

    #[test]
    fn unknown_leading_run_stays_one_chunk() {
        assert_eq!(method_to_pascal("xyzzyblock"), "XyzzyBlock");
    }

    #[test]
    fn merged_list_splits_former_failures() {
        assert_eq!(to_rust_field("networkactive"), "network_active");
        assert_eq!(to_rust_field("timeoffset"), "time_offset");
        assert_eq!(to_rust_field("pingwait"), "ping_wait");
        assert_eq!(to_rust_field("openrpc"), "open_rpc");
        assert_eq!(to_rust_field("subtractfeefromamount"), "subtract_fee_from_amount");
        assert_eq!(to_rust_field("chainstates"), "chain_states");
        assert_eq!(method_to_pascal("getunconfirmedbalance"), "GetUnconfirmedBalance");
    }

    #[test]
    fn plurals_split_from_singular() {
        assert_eq!(to_rust_field("descriptors"), "descriptors");
        assert_eq!(to_rust_field("transactions"), "transactions");
        assert_eq!(method_to_pascal("listdescriptors"), "ListDescriptors");
    }

    #[test]
    fn bitcoin_compound_nouns_stay_whole() {
        assert_eq!(method_to_pascal("gettxspendingprevout"), "GetTxSpendingPrevout");
        assert_eq!(to_rust_field("prevout"), "prevout");
        assert_eq!(to_rust_field("vout"), "vout");
        assert_eq!(to_pascal("vout"), "Vout");
        assert_eq!(to_rust_field("vin"), "vin");
        assert_eq!(to_rust_field("pubnonce"), "pubnonce");
        assert_eq!(to_rust_field("pubnonces"), "pubnonces");
        assert_eq!(to_pascal("pubnonce"), "Pubnonce");
    }

    #[test]
    fn decomposes_addrman_and_hashps_without_breaking_sigops() {
        assert_eq!(method_to_pascal("getaddrmaninfo"), "GetAddrManInfo");
        assert_eq!(method_to_pascal("getnetworkhashps"), "GetNetworkHashPs");
        assert_eq!(method_to_snake("getnetworkhashps"), "get_network_hash_ps");
        assert_eq!(to_rust_field("networkhashps"), "network_hash_ps");
        assert_eq!(to_rust_field("sigops"), "sigops");
        assert_eq!(to_rust_field("sigoplimit"), "sigop_limit");
        assert_eq!(to_rust_field("elapsed"), "elapsed");
    }

    #[test]
    fn word_list_invariants() {
        for w in WORDS {
            assert!(!w.is_empty(), "empty entry");
            assert!(
                w.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "non-lowercase entry: {w}"
            );
        }
        let mut sorted = WORDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), WORDS.len(), "duplicate entries in WORDS");
        assert_eq!(sorted, WORDS, "WORDS must stay alphabetically sorted");
    }

    #[test]
    fn greedy_split_is_lossless() {
        let words = sorted_words();
        let check = |input: &str| {
            let joined: String = greedy_split(input, &words).concat();
            assert_eq!(joined, input, "lossy split of {input}");
        };
        for a in WORDS {
            check(a);
            check(&format!("zz{a}"));
            check(&format!("{a}zz"));
        }
        for a in WORDS.iter().step_by(7) {
            for b in WORDS.iter().step_by(11) {
                check(&format!("{a}{b}"));
            }
        }
    }
}
