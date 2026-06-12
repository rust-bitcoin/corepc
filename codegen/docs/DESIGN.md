# Design

A one-shot code generator for Bitcoin Core JSON-RPC bindings. It reads Core's OpenRPC export and
writes versioned Rust modules into the sibling `corepc-types` and `corepc-client` crates. The output
is committed; the generator is not part of the consumers' build.

It does not execute RPC calls. It emits response types, their model conversions, and the async call
surface that the consumers compile.

A small compiler: `src/spec.rs` (front end) deserializes the OpenRPC subset used; `codegen.rs::lower()`
(middle) converts it to generator-owned data; `Modules::write()` (back end) emits the files.
`src/names.rs` handles identifier splitting, `main.rs` is the CLI. `btc-codegen` is a private
workspace member depending only on `serde` + `serde_json`. Specs live in `specs/` (one per release
v17..v31, from a Core patch adding `getopenrpcinfo`).

## Flow

`main.rs` resolves the version and `find_spec()` picks `specs/v{version}_*_openrpc.json`. `generate()`
parses it, `lower()` builds `Modules`, `Modules::write()` emits per Core help category:

- `types/src/v{N}/generated/{category}.rs`: response structs, tuple newtypes, nested helpers.
- `types/src/v{N}/generated/model/{category}.rs`: model counterparts (`into_model()`, `*Error`).
- `client/src/client_async/v{N}/{category}.rs`: `*Options` structs, param helpers, `impl Client`.
- `mod.rs` for both trees.

Type names dedup by `BTreeSet`; `dedup_structural()` then collapses structurally identical helpers
within a category file (merges stay in-file since categories have no cross-imports). Two shapes
differing in any field stay separate. Naming rules: `docs/NAMING.md`.

## Return type shapes

| schema | Rust |
| --- | --- |
| `type: null` | `Result<()>`, no type |
| scalar | tuple newtype + `Deref` (`pub struct GetBlockCount(pub i64);`) |
| object | named struct from `properties` |
| array | tuple newtype around `Vec<T>` |
| dynamic object/map | tuple newtype around `BTreeMap<String, T>` |
| `oneOf`/`anyOf` by `verbose`/`verbosity` | one type + one method per level (`GetBlockVerboseZero..Three`); higher levels accumulate lower fields |
| other `oneOf`/`anyOf` | one `#[serde(untagged)]` enum named after the method |

Nested object/array schemas recurse into `{Parent}{Field}` helpers (`DecodePsbtInputsItem`).

## Object fields

Requiredness from the schema `required` array; `x-bitcoin-optional` also forces `Option<T>`. Names
via `to_rust_field()`, with `#[serde(rename)]` when the Rust name differs from the wire name. Empty
object -> empty struct, dynamic object -> `BTreeMap`, unrepresentable -> `serde_json::Value`. Special
case: a commentary-only response referencing `decoderawtransaction` aliases to `DecodeRawTransaction`.

## Parameter types

Inferred separately from return types (Core's spec is loose around numbers). `amount` -> `f64`,
`hex` -> `String`, `oneOf`/`anyOf` -> `Value`, string/bool/integer/array map directly, object -> a
param struct or `Value`. `type: number` params default to `f64`, switching to `i64` when the default
is integer-shaped or the name is in `INTEGER_PARAM_NAMES`. Response-side `number` is classified by
`classify_number` (keyed by parent type + wire field) into `i64`/`u64`/`f64`, default `i64`.

## Optional arguments

Core's optional args are positional; a skipped slot is JSON `null`. Each method with optional params
emits `foo(req)` and `foo_with(req, opts)`. The `*Options` struct holds one `Option<T>` per optional
param, derives `Default`, and is sent either as spread positional args (`json!(opts.field)`, `None`
-> `null`) or, when the single optional param is itself an object, whole as one trailing arg with
unset fields skipped (`skip_serializing_if`).

## Method emission

Each client file is an `impl Client` block, categories sorted alphabetically, methods by wire name.
Bodies are thin: `self.call_raw("getblockhash", &[json!(height)]).await`. No-param methods use
`&[(); 0] as &[()]`. Files import from `types::v{N}::generated` and `client_async::error::Result`.

## Naming

`WORDS` (in `src/names.rs`) is sorted longest-first and fed to a greedy splitter: each match must end
on a clean boundary (input end or another known word), unmatched runs are kept whole. So plurals
coexist with singulars (`blocks` never shadows `block` + `stats`), single-word forms beat compound
entries, Rust keywords get an `_` suffix, and snake/camelCase is handled before lowercase splitting.
`WORDS` is a maintenance hotspot: a new awkward RPC noun means adding the word and regenerating.

`RESERVED_TYPE_NAMES` rewrites prelude collisions (`Send`->`SendResult`, `Vec`->`VecResult`,
`Result`->`ResultResponse`, and so on) for both the type def and the return type.

## Model layer

Each raw type gets a same-named model counterpart converted via `into_model()`; semantic strings
(hashes, addresses, amounts) become `rust-bitcoin` types. Fallibility is a fixpoint over the types;
fallible ones get a `{Type}Error` enum (one variant per failing field), pure passthroughs become
`pub type` aliases.

## Docs

Doc comments come from Core's RPC help. `esc_doc` escapes them to pass rustdoc `-D warnings`
(bracket placeholders/ranges escaped, existing autolinks kept, bare URLs wrapped), otherwise leaving
the text close to upstream for review.

## Tests

Inline in `codegen.rs` / `names.rs`: name splitting, keyword handling, doc escaping, number
classification, verbose suffix detection, structural dedup, options emission, reserved-name rewrites,
and an end-to-end `lower()` over v30. Real acceptance is external: regenerate (`just codegen 30`),
build the workspace, run the consumer suites against a real `bitcoind`.
