# Naming schema (codegen rules)

This document is the complete reference for how the OpenRPC-driven generator turns Bitcoin Core's
JSON-RPC surface into Rust identifiers. It is the source of truth for the naming behaviour spread
across `src/names.rs` (identifier splitting) and `src/codegen.rs` (type, field, method, and model
naming). Everything here is mechanical: given the same spec and the same word list, the output is
deterministic.

The generator emits three layers and the rules below cover all of them:

- raw types (`corepc-types`, `v{N}/generated/{category}.rs`): one type per RPC return plus nested
  helpers.
- model types (`corepc-types`, `v{N}/generated/model/{category}.rs`): strongly typed counterparts
  with `into_model` conversions and `*Error` enums.
- the call surface (`corepc-client`, `v{N}/{category}.rs`): the `*Options` request structs and
  their parameter helper types, plus an `impl Client` block of async method wrappers.

Throughout, "wire name" means the lowercase string Bitcoin Core actually uses on the JSON-RPC
connection (the method name like `getblockheader`, or a JSON field key like `bestblockhash`).

## 1. The word list

Bitcoin Core RPC names and many of its field names are all-lowercase compounds with no separators
(`getblockheader`, `bestblockhash`). There is no rule derivable from the characters alone that finds
the word boundaries, so the generator carries one curated vocabulary in `src/names.rs`:

- `WORDS` splits both RPC method names and JSON field names (and the free-form names used for nested
  types). Used by `method_to_pascal`, `method_to_snake`, `to_rust_field` and `to_pascal`.

A single list means there is exactly one place to add a word when a new Core release introduces a
noun, and the method and field splitters can never disagree on a boundary because they feed the same
vocabulary to the same `greedy_split`. So a field `scanobjects` and a method fragment both split the
same way (`scan` + `objects`).

### 1.1 Plural rule

Plurals can be listed freely; the clean-boundary check in `greedy_split` (section 2) keeps them from
shadowing a singular. `blocks` and `block` coexist: `blocks` only wins when the character after it
begins another known word. So `getblockstats` still splits `block` + `stats` (the trailing `s` is
consumed by `stats`, never stranded as `tats`), while `scanblocks` correctly splits `scan` +
`blocks`. This was previously handled by *omitting* consonant-plus-`s` plurals; the boundary check
makes that omission unnecessary.

### 1.2 Single-word-forms-preferred rule

When a compound can be decomposed cleanly into listed single words, prefer that over adding a
compound entry. `txout` is split as `tx` + `out`, giving `GetTxOut`; `addrman` as `addr` + `man`;
`networkhashps` as `network` + `hash` + `ps`. Only compounds that cannot decompose cleanly get their
own entry (`prevout` is listed whole).

A few entries exist purely to protect against an over-eager short word. Listing `ps` (so
`networkhashps` splits) would otherwise let it eat the tail of `sigops` and `elapsed`, so those two
are listed whole to keep their own boundary and win by longest-match.

### 1.3 When to touch the list

This module is a maintenance hotspot. When a new Core release adds an RPC noun that generates an
awkward identifier, add the missing word to `WORDS` and regenerate. The test
`word_list_remains_consistent` (codegen) and the splitter tests in `names.rs` guard the invariants
above: `txout` and `outset` must stay absent (so they decompose), and the `addrman`/`hashps`
decomposition plus the `sigops`/`elapsed` protection are pinned directly.

## 2. The splitting algorithm

The list is sorted longest-first at use time, then fed to one greedy splitter (`greedy_split`).
The PascalCase path (`pascal_pieces`) is exactly `greedy_split` followed by capitalising each piece,
so the snake and Pascal renderings can never disagree on boundaries (pinned by
`pascal_pieces_agrees_with_greedy_split`).

At each position the splitter takes the longest listed word that ends on a **clean boundary**: the
end of the string, or the start of another listed word. If nothing qualifies, it consumes the run of
characters up to the next position where some word does start, and keeps that whole run as one piece.

Two consequences:

- Longest-match-wins: `blockchain` beats the shorter `block` prefix, so `getblockchaininfo` ->
  `GetBlockchainInfo`.
- Unknown tokens stay readable: an unrecognised leading run is kept whole, not shattered into
  characters. `reconsiderblock` -> `ReconsiderBlock` (the word `reconsider` is not listed),
  `xyzzyblock` -> `XyzzyBlock`.

The clean-boundary check is what keeps the splitter robust as the word list and the RPC surface
drift across Core versions. Without it a short word that is a prefix of a longer unlisted word wins
and strands the remainder: `priv` inside `private` would leave `atebroadcast`, giving
`PrivAtebroadcast`. Requiring the next character to begin a known word rejects that `priv` (its
remainder is not a word) while still accepting `priv` in `privkey` (followed by `key`). So
`abortprivatebroadcast` -> `AbortPrivatebroadcast`, never garbage.

## 3. Method identifiers

For a wire method name like `getblockheader`:

- PascalCase (`method_to_pascal`) -> `GetBlockHeader`. This is the base for the return-type ident,
  the `*Options` struct, and every nested helper type.
- snake*case (`method_to_snake`) -> `get_block_header`. This is the async fn name. It is derived by
  re-casing the PascalCase form (insert `*` before each interior uppercase, lowercase all), so it
  always agrees with the Pascal split.

Worked examples: `getbestblockhash` -> `get_best_block_hash`, `sendrawtransaction` ->
`send_raw_transaction`, `setmocktime` -> `set_mock_time`, `getopenrpc` -> `get_open_rpc`.

### 3.1 Reserved type-name rewrites

A method whose PascalCase ident collides with a std/prelude name would emit uncompilable Rust
(`struct Send`). `safe_type_name` rewrites the exact collisions, consistently for both the emitted
type definition and the method's `Result<...>` return type:

| Raw ident | Rewritten to     |
| --------- | ---------------- |
| `Send`    | `SendResult`     |
| `Sync`    | `SyncResult`     |
| `Drop`    | `DropResult`     |
| `Box`     | `BoxResult`      |
| `Vec`     | `VecResult`      |
| `Option`  | `OptionResult`   |
| `Result`  | `ResultResponse` |

Only these exact idents are rewritten; everything else passes through. `getsend...`-style names that
merely contain `Send` as a substring are unaffected.

### 3.2 Rust-keyword safety

When a snake*case identifier lands exactly on a Rust keyword, it gets an underscore suffix
(`rust_keyword_safe`). The mapped keywords are `type` -> `type*`, `match`->`match*`,
`ref`->`ref*`, `self`->`self*`, `mod`->`mod*`, `async`->`async*`, `await`->`await*`,
`use`->`use\_`. This applies to method fn names and to field names.

## 4. Field identifiers

`to_rust_field` converts a JSON field key to a snake_case Rust field name, handling three input
shapes in order:

1. Already separated (`-` or `_`): replace `-` with `_`, lowercase, done. `max_burn_amount` stays
   `max_burn_amount`.
2. camelCase: insert `_` at every lower->upper transition, lowercase. `maxBurnAmount` ->
   `max_burn_amount`.
3. All-lowercase compound: split against `WORDS`. `bestblockhash` -> `best_block_hash`,
   `maxfeerate` -> `max_fee_rate`, `verificationprogress` -> `verification_progress`.

Keyword safety from 3.2 applies last, so a field named `type` becomes `type_`.

### 4.1 serde rename

Whenever the Rust field name differs from the wire key, the struct field carries
`#[serde(rename = "<wirekey>")]` so the original key still deserializes. Fields that already match
their wire key get no rename.

### 4.2 Field-name collision disambiguation

Distinct wire keys can normalise to the same Rust ident. The classic case is an options object
carrying both `feeRate` and `fee_rate`, which both reduce to `fee_rate`. The generator keeps a
per-struct used-set and suffixes the duplicate with `2`, `3`, ... (`uniquify`). The serde rename on
each field still carries the true wire key, so the collision is invisible on the wire.

## 5. Return-type shapes (raw layer)

The return type for a method is named after the method's PascalCase ident (after reserved-name
rewriting). The Rust shape depends on the OpenRPC result schema:

- `type: null` -> no type emitted; the method returns `Result<()>`.
- simple scalar -> tuple newtype: `pub struct GetBlockCount(pub i64);`,
  `pub struct GetBestBlockHash(pub String);`.
- object with properties -> named struct with one field per property.
- array -> tuple newtype around `Vec<Item>`: `pub struct Foo(pub Vec<FooItem>);`.
- dynamic object / map (`x-bitcoin-object-dynamic`, or `additionalProperties` schema with no real
  properties of its own) -> tuple newtype around `std::collections::BTreeMap<String, V>`.
- `oneOf` / `anyOf` data union (not verbosity-selected) -> a `#[serde(untagged)] enum` named after
  the method (see section 7).

Empty object schemas become empty structs (`pub struct Foo {}`). Unrepresentable or ambiguous shapes
fall back to `serde_json::Value`.

## 6. Nested helper type names

Object and array schemas nested inside a return type (or a parameter type) recursively spawn their
own named helper types. The name is built by concatenating the parent type name with the PascalCased
field or item role. The pieces:

- struct field that is itself an object: `{Parent}{Field}`. Field is PascalCased with `to_pascal`
  (so it splits against `WORDS`). Example: a `scriptPubKey` object field under
  `GetRawTransactionVerboseOneVOutItem` becomes `GetRawTransactionVerboseOneVOutItemScriptPubKey`.
- array element object: `{Parent}{Field}Item`. The `Item` suffix marks "one element of". Example:
  the `inputs` array of `decodepsbt` gives `DecodePsbtInputsItem`, and its `bip32_derivs` array in
  turn gives `DecodePsbtInputsItemBip32DerivsItem`.
- whole-result array element (the return type is itself an array): `{Parent}Item`.
- map value object: `{Parent}Entry`, and if the parent name already ends in `Entry`, the next level
  uses `Item` instead (so two map levels read `...Entry` then `...EntryItem` rather than
  `...EntryEntry`).
- map value reached via `additionalProperties` on a dynamic object: `{Parent}Entry`.

This is applied recursively to arbitrary depth, which is why real names get long but stay
unambiguous and locally traceable to their JSON path:
`DecodePsbtInputsItemWitnessUtxoScriptPubKey`,
`GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey`.

Note the capitalisation of an internal noun reflects how `WORDS` splits it. `vout` has no listed
split that applies in this position and surfaces as `VOut` in these compound type idents
(`...VOutItem`); `txout` splits to `TxOut`; `muhash` splits to `MuHash` (field `mu_hash`).

After lowering, a structural deduplication pass (`dedup_structural`) collapses helper types that
are structurally identical onto one definition and rewrites every reference to it. Merges are kept
within one category and one side of the RPC (response vs request): each of those pairs is emitted
to its own file with no cross-imports, so a winner must live in the same file as every reference.
The surviving definition keeps the lexicographically smallest name of the merged group, and a
method's own return type is an anchor that can win a merge but never be removed. Two shapes that
differ in any field hash differently and stay separate, so a wrong merge is impossible by
construction. Names are additionally deduplicated as exact-string collisions via a `BTreeSet`;
section 9 covers what happens when two different schemas want the same name.

## 7. Union (oneOf / anyOf) naming

There are two distinct union cases and they are named differently.

### 7.1 Data unions -> one untagged enum

A union that is not selected by a `verbose`/`verbosity` argument becomes a single
`#[serde(untagged)] enum` named after the method (or after `{Parent}{Field}` when the union is a
nested field). Each arm becomes one variant, with the variant ident chosen by the arm's JSON kind:

| Arm JSON kind      | Variant ident |
| ------------------ | ------------- |
| string             | `Text`        |
| boolean            | `Bool`        |
| integer / number   | `Number`      |
| array              | `List`        |
| object             | `Object`      |
| null               | `Null`        |
| nested oneOf/anyOf | `Nested`      |
| anything else      | `Value`       |

Collisions (two object arms, say) are disambiguated by appending `2`, `3`, ... to the ident. An
object or array arm that needs its own nested type names it `{EnumName}V{i}` (zero-based arm index),
e.g. `CreatePsbtOutputsV1`, `BumpFeeOptionsOutputsV1`. A bare `number` arm in a union is treated as
`f64` (these unions model fee rates and amounts), not the struct-field default of `i64`.

### 7.2 Verbosity-selected unions -> one type and one method per level

When the union is cleanly selected by a `verbose` or `verbosity` parameter, the generator does not
emit an enum. Instead it emits one concrete type per verbosity level and one method per level (see
9.3). The per-level type suffix is derived from the arm's condition text:

| Condition contains                                                      | Suffix            | Method word |
| ----------------------------------------------------------------------- | ----------------- | ----------- |
| `verbose=false` / `verbose=0` / `verbose is not set` / `verbosity=0`    | `VerboseZero`     | `zero`      |
| `verbose=true` / `verbose=1` / `verbose is set to true` / `verbosity=1` | `VerboseOne`      | `one`       |
| `verbose=2` / `verbosity=2`                                             | `VerboseTwo`      | `two`       |
| `verbose=3` / `verbosity=3`                                             | `VerboseThree`    | `three`     |
| (fallback by arm index)                                                 | `VerboseFour` ... | `four` ...  |

So `getblock` yields `GetBlockVerboseZero`, `GetBlockVerboseOne`, `GetBlockVerboseTwo`,
`GetBlockVerboseThree`, and their nested helpers carry the suffix forward
(`GetBlockVerboseTwoTxItem`). The suffix is matched on the condition text, not the arm position, so
a spec listing the `true` arm before the `false` arm still maps each to the right level.

"Cleanly selected" requires the selector parameter to exist, only required parameters to precede it,
and no arm condition that combines the selector with another parameter (no `" and "`, as in
getrawmempool's `verbose=false and mempool_sequence=true`). When any of those fail, the union falls
back to the single untagged enum of 7.1.

Higher verbosity levels in Core return the lower level's object plus extra fields, but the OpenRPC
export records only the delta. The generator accumulates the object arms so each later struct
regains the inherited fields, which is why `GetBlockVerboseThree` is a full struct rather than a
two-field delta.

## 8. `*Options` request structs

Every method with at least one optional positional parameter gets an options struct named
`{MethodPascal}Options` (`AddNodeOptions`, `BumpFeeOptions`, `CreateWalletOptions`). It derives
`Default` and holds one `Option<T>` field per optional parameter. There are two construction modes:

- spread: each optional parameter is a field, sent as a separate positional argument. The struct
  carries `#[serde(rename_all = "camelCase")]` but that is cosmetic, since the fields go on the wire
  positionally, not as a serialized object.
- object options: when a method's single optional parameter is itself a JSON object, that object's
  properties are flattened into the options struct and the struct is sent whole as one trailing
  positional argument. In this mode each field carries its exact wire key via
  `#[serde(rename = "...")]` (when it differs) plus `skip_serializing_if = "Option::is_none"`, so an
  unset field disappears rather than sending `null`.

A parameter-derived helper type whose name would collide with the method's own `*Options` struct
(for instance a parameter literally named `options`) is given an `Arg` suffix, then `Arg2`, `Arg3`,
... if needed (`unique_type_name`). This is why you see `DumpTxOutSetOptionsArg`: the `*Options`
name was already taken by the method's own options struct, so the parameter object's type took the
`Arg` form.

## 9. Exact-name collision handling

Three different mechanisms keep the flat type namespace collision-free, each with its own suffix:

1. prelude/std collisions on the top-level method type: rewritten by `safe_type_name` (`Send` ->
   `SendResult`, etc; section 3.1).
2. parameter helper vs the method's `*Options` struct: `Arg` / `Arg2` suffix (`unique_type_name`,
   section 8).
3. enum-variant idents and struct field idents within one definition: numeric `2` / `3` suffix
   (`uniquify`, sections 4.2 and 7.1).

Distinct generated definitions that resolve to the same name via the `BTreeSet` keep the first real
body; the later one is dropped as an empty/opaque duplicate rather than renamed, on the assumption
the shapes are equivalent (the structural dedup pass then removes stubs whose name lost a merge).

## 10. Method surface (client layer)

Each category's client file (`client_async/v{N}/{category}.rs`) holds the `*Options` request
structs followed by an `impl Client` block. The naming rules there:

- the fn name is the method's snake_case ident (section 3).
- a method with optional parameters emits two fns: the required-only `{snake}` and `{snake}_with`,
  the latter taking the `{MethodPascal}Options` struct as a trailing `opts` argument.
- a verbosity-selected union (7.2) emits one fn per level instead. The verbosity-0 level is the
  plain `{snake}`; every higher level is `{snake}_verbose_{word}` (`get_block`,
  `get_block_verbose_one`, `get_block_verbose_two`, `get_block_verbose_three`). Each such fn returns
  the concrete `{MethodPascal}Verbose{Level}` type and hardcodes the selector argument.

The category a method lands in (and therefore which `{category}.rs` file holds it) is Bitcoin Core's
`x-bitcoin-category`, lowercased. The only spelling adjustment is `rawtransactions` ->
`raw_transactions` as the module name, matching the hand-written layout. All other categories are
already single lowercase words and are used verbatim.

## 11. Model layer naming

The model layer mirrors each raw type one-to-one and reuses the raw type's name. For a raw type
`Foo`:

- the model struct/enum/newtype is also named `Foo` (in the `model` module), so
  `raw::Foo::into_model() -> model::Foo`.
- when the conversion can fail, the error enum is `{Foo}Error` (`GetBlockStatsError`,
  `GetBlockVerboseOneError`).
- model field names are identical to the raw field names (same `to_rust_field` output); only the
  field _type_ changes (e.g. `String` -> `BlockHash`, `f64` -> `Amount`).
- error-enum variant idents come from the field that failed, PascalCased with `to_pascal`. Field
  `next_block_hash` -> variant `NextBlockHash`; field `previous_block_hash` -> `PreviousBlockHash`.
  Variant collisions within one error enum are de-duplicated with the numeric `uniquify` suffix.
- a newtype's single fallible conversion uses the variant ident `Inner`.
- a model type that is a pure passthrough alias to its raw type is emitted as
  `pub type Foo = Bar;` reusing the target's conversion and error type.

The set of "fallible" types (those whose `into_model` returns a `Result` and therefore get an
`*Error` enum) is computed as a fixpoint over the generated types: a type is fallible if any field,
element, or map value it transitively contains performs a fallible conversion (a string parsed into
a `rust-bitcoin` hash/address, or a `BTC` float parsed into `Amount`). Cycles are broken
conservatively as non-fallible.

## 12. Quick reference

| Input (wire)                      | Output                                           | Rule      |
| --------------------------------- | ------------------------------------------------ | --------- |
| `getblockheader` (method)         | `GetBlockHeader` / `get_block_header`            | 3         |
| `send` (method)                   | `SendResult` / `send`                            | 3.1       |
| `bestblockhash` (field)           | `best_block_hash`                                | 4         |
| `maxBurnAmount` (field)           | `max_burn_amount`                                | 4 (camel) |
| `type` (field)                    | `type_`                                          | 3.2 / 4   |
| `blockstats`                      | `block_stats` (never `blocks`+`tats`)            | 1.1       |
| `txout`                           | `tx_out` / `TxOut`                               | 1.2       |
| `decodepsbt` inputs[]             | `DecodePsbtInputsItem`                           | 6         |
| `getblock` verbosity union        | `GetBlockVerboseZero..Three` + per-level methods | 7.2       |
| `number \| string` union arm      | `Number(f64)` / `Text(String)`                   | 7.1       |
| method with optional object param | `{Pascal}Options`, sent whole                    | 8         |
| param named `options` clashing    | `...OptionsArg`                                  | 8 / 9     |
| raw `GetBlockStats`               | model `GetBlockStats` + `GetBlockStatsError`     | 11        |

## Appendix: where each rule lives

- `src/names.rs`: the word list (`WORDS`), the splitter (`greedy_split`, `clean_word`),
  `method_to_pascal` / `method_to_snake` / `to_rust_field` / `to_pascal`, keyword safety.
- `src/codegen.rs`: reserved-name rewrites (`safe_type_name`, `RESERVED_TYPE_NAMES`), nested-type
  names (`schema_to_type`, `array_field`, `object_field`, `inner_type`, `map_value`), union naming
  (`enum_type`, `arm_variant_ident`, `one_of`, `verbose_variants`, `verbose_suffix`), options
  structs (`emit_options_struct`, `unique_type_name`), field collision handling (`uniquify`),
  category module mapping (`category_module`), and the whole model layer (`emit_model_type`,
  `emit_error_enum`).

# Client method naming: sync vs async (corepc-client)

This section catalogues every method name on the `corepc-client` clients and reconciles the two
implementations:

- the **sync** client, hand-written under `client/src/client_sync/v17..v31/`. Each method is defined
  in an `impl_client_v{N}__name!` macro and a version's surface is assembled by invoking those
  macros in `v{N}/mod.rs`. Macros can define more than one `pub fn`, and a few define a `pub fn`
  whose name differs from the macro name.
- the **async** client, generated under `client/src/client_async/v30/`. Each method is a
  `pub async fn` calling `self.call_raw("wire", ...).await`.

The async client currently ships **v30 only**, so the correspondence below is sync-v30 against
async-v30. Versions v17..v29 and v31 exist on the sync side with no async counterpart at all; they
are listed in the version section at the end.

Counts (v30): sync exposes **191** real `pub fn`s; async exposes **235** `pub async fn`s. The async
total is larger only because of the `_with` convention (78 of them) explained next; collapsing each
`X_with` onto its base `X` leaves 157 async base methods, of which **143 match a sync name exactly**.

## The two naming philosophies

Names match for the large common core because both ultimately snake_case the same Bitcoin Core RPC
method name (`getblockheader` -> `get_block_header`). They diverge in three systematic ways, and
every mismatch below traces to one of these:

1. **Optional arguments.** Sync folds a method's options into the single base method (its module
   header literally says "We ignore option arguments unless they effect the shape of the returned
   JSON data"). Async instead emits a uniform pair: the required-only `X` and an `X_with(.., opts)`
   overload. So a base `X` present on both sides may carry an async-only `X_with` twin (73 of the
   matched names do).

2. **Verbosity-selected returns.** Both sides split these into one method per verbosity level, but
   name the levels differently. Async (from the generator) makes the base method verbosity-0 and
   suffixes the rest `_verbose_{word}`: `get_block` (=0), `get_block_verbose_one/two/three`. Sync
   spells level 0 explicitly and uses a single `_verbose` for the higher object form, or its own
   words: `get_block_verbose_zero/one/two/three`, but `get_raw_transaction` + `_verbose`.

3. **Ergonomic convenience methods.** Sync hand-adds variants that hardcode an argument or
   post-process the result into a richer `rust-bitcoin` type. These have no async equivalent:
   `send_to_address_rbf`, `get_block_stats_by_height`, `new_address`, `best_block_hash`, the
   `scan_blocks_start/status/abort` triplet, and so on.

One pair still differs in **how a compound noun is split** at the fn level
(`sign_message_with_privkey` vs `sign_message_with_priv_key`); it is the single cross-spelling pair
in the next section.

## Cross-spelling pairs (same RPC, name differs): `sync -> async`

The generator's word list (`WORDS`) is tuned so its split matches the hand-written `corepc`
spelling. Two former drifts have been aligned: `addrman` splits `addr` + `man` (so the generator
emits `get_addr_man_info` / `GetAddrManInfo` / `GetRawAddrMan`, matching legacy) and `hashps` splits
`hash` + `ps` (`get_network_hash_ps`, field `network_hash_ps`).

One irreconcilable case remains, and it is irreconcilable by construction: the generator forces the
snake fn to be a re-casing of the PascalCase type (codegen rules section 2), but legacy `corepc`
splits `privkey` *differently* for the two — `PrivKey` in the type, `privkey` (one token) in the fn.
The word list is set so the **type** matches legacy (`SignMessageWithPrivKey`), which means the fn
necessarily reads `sign_message_with_priv_key` rather than legacy's `sign_message_with_privkey`. The
type is the load-bearing public `corepc-types` name, so it wins the tie. The same single-underscore
divergence applies to `getrawaddrman` (type `GetRawAddrMan` matches; generated fn `get_raw_addr_man`
vs legacy `get_raw_addrman`).

| wire RPC                 | sync (legacy) name          | ->  | async (generated) name       | note |
| ------------------------ | --------------------------- | --- | ---------------------------- | ---- |
| `signmessagewithprivkey` | `sign_message_with_privkey` | ->  | `sign_message_with_priv_key` | type `SignMessageWithPrivKey` matches; fn differs by one `_` |
| `getrawaddrman`          | `get_raw_addrman`           | ->  | `get_raw_addr_man`           | type `GetRawAddrMan` matches; fn differs by one `_` |

## Verbosity / multi-method families (same RPC, both sides split it, names differ)

These wire RPCs are split into several methods on each side, but with different level naming. Listed
as `wire: { sync set } -> { async set }`. Where a name is identical on both sides it is in the exact-
match table too; the split is shown here whole so the family reads coherently.

| wire RPC                | sync methods                                                                                          | async methods                                                                                    |
| ----------------------- | ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `getblock`              | `get_block_verbose_zero`, `get_block_verbose_one`, `get_block_verbose_two`, `get_block_verbose_three` | `get_block` (=0), `get_block_verbose_one`, `get_block_verbose_two`, `get_block_verbose_three`    |
| `getblockheader`        | `get_block_header`, `get_block_header_verbose`                                                        | `get_block_header` (=0), `get_block_header_verbose_one`                                          |
| `getrawtransaction`     | `get_raw_transaction`, `get_raw_transaction_verbose`                                                  | `get_raw_transaction` (=0), `get_raw_transaction_verbose_one`, `get_raw_transaction_verbose_two` |
| `getmempoolancestors`   | `get_mempool_ancestors`, `get_mempool_ancestors_verbose`                                              | `get_mempool_ancestors` (=0), `get_mempool_ancestors_verbose_one`                                |
| `getmempooldescendants` | `get_mempool_descendants`, `get_mempool_descendants_verbose`                                          | `get_mempool_descendants` (=0), `get_mempool_descendants_verbose_one`                            |
| `getrawmempool`         | `get_raw_mempool`, `get_raw_mempool_verbose`, `get_raw_mempool_sequence`                              | `get_raw_mempool`, `get_raw_mempool_with`                                                        |
| `getblockstats`         | `get_block_stats_by_height`, `get_block_stats_by_block_hash`                                          | `get_block_stats`, `get_block_stats_with`                                                        |
| `scanblocks`            | `scan_blocks_start`, `scan_blocks_status`, `scan_blocks_abort`                                        | `scan_blocks`, `scan_blocks_with`                                                                |
| `scantxoutset`          | `scan_tx_out_set_start`, `scan_tx_out_set_status`, `scan_tx_out_set_abort`                            | `scan_tx_out_set`, `scan_tx_out_set_with`                                                        |
| `getdeploymentinfo`     | `get_deployment_info`, `get_deployment_info_tip`                                                      | `get_deployment_info`, `get_deployment_info_with`                                                |
| `deriveaddresses`       | `derive_addresses`, `derive_addresses_multipath`                                                      | `derive_addresses`, `derive_addresses_with`                                                      |
| `estimatesmartfee`      | `estimate_smart_fee`, `estimate_smart_fee_with_mode`                                                  | `estimate_smart_fee`, `estimate_smart_fee_with`                                                  |
| `sendtoaddress`         | `send_to_address`, `send_to_address_rbf`                                                              | `send_to_address`, `send_to_address_with`                                                        |

Reading these: where the difference is `..._verbose` vs `..._verbose_one/two`, it is philosophy #2
(level naming). Where the difference is a named sync variant (`_by_height`, `_multipath`,
`_with_mode`, `_rbf`, `_tip`, `_start/_status/_abort`, `_sequence`) vs an async `_with`, it is
philosophy #1/#3: sync hardcodes a specific argument shape into a named method; async exposes the
same surface through the generic `_with(opts)` overload.

## Methods present on BOTH sides with the identical name

The shared core (143 names; async may additionally carry an `X_with` twin per philosophy #1). These
are byte-for-byte identical sync `pub fn` and async `pub async fn` names:

```
abandon_transaction            get_block_template             list_wallets
abort_rescan                   get_block_verbose_one          load_tx_out_set
add_node                       get_block_verbose_three        load_wallet
analyze_psbt                   get_block_verbose_two          lock_unspent
backup_wallet                  get_chain_states               logging
bump_fee                       get_chain_tips                 migrate_wallet
clear_banned                   get_chain_tx_stats             ping
combine_psbt                   get_connection_count           precious_block
combine_raw_transaction        get_deployment_info            prioritise_transaction
convert_to_psbt                get_descriptor_activity        prune_blockchain
create_multisig                get_descriptor_info            psbt_bump_fee
create_psbt                    get_difficulty                 remove_pruned_funds
create_raw_transaction         get_hd_keys                    rescan_blockchain
create_wallet                  get_index_info                 restore_wallet
create_wallet_descriptor       get_memory_info                save_mempool
decode_psbt                    get_mempool_ancestors          send
decode_raw_transaction         get_mempool_descendants        send_all
decode_script                  get_mempool_entry              send_many
derive_addresses               get_mempool_info               send_raw_transaction
disconnect_node                get_mining_info                send_to_address
dump_tx_out_set                get_net_totals                 set_ban
encrypt_wallet                 get_network_info               set_network_active
enumerate_signers              get_new_address                set_tx_fee
estimate_smart_fee             get_node_addresses             set_wallet_flag
finalize_psbt                  get_peer_info                  sign_message
fund_raw_transaction           get_prioritised_transactions   sign_raw_transaction_with_key
get_added_node_info            get_raw_change_address         sign_raw_transaction_with_wallet
get_addresses_by_label         get_raw_mempool                simulate_raw_transaction
get_address_info               get_raw_transaction            stop
get_balance                    get_received_by_address        submit_block
get_balances                   get_received_by_label          submit_header
get_best_block_hash            get_rpc_info                   submit_package
get_block                      get_transaction                test_mempool_accept
get_blockchain_info            get_tx_out                     unload_wallet
get_block_count                get_tx_out_proof               uptime
get_block_filter               get_tx_out_set_info            utxo_update_psbt
get_block_from_peer            get_tx_spending_prevout        validate_address
get_block_hash                 get_wallet_info                verify_chain
get_block_header               help                           verify_message
get_block_template             import_descriptors             verify_tx_out_proof
                               import_mempool                 wait_for_block
                               import_pruned_funds            wait_for_block_height
                               join_psbts                     wait_for_new_block
                               key_pool_refill                wallet_create_funded_psbt
                               list_address_groupings         wallet_display_address
                               list_banned                    wallet_lock
                               list_descriptors               wallet_passphrase
                               list_labels                    wallet_passphrase_change
                               list_lock_unspent              wallet_process_psbt
                               list_received_by_address
                               list_received_by_label
                               list_since_block
                               list_transactions
                               list_unspent
                               list_wallet_dir
```

The async `_with` overloads that exist for these (no sync equivalent, since sync folds options into
the base method): `add_node_with`, `bump_fee_with`, `convert_to_psbt_with`, `create_multisig_with`,
`create_psbt_with`, `create_raw_transaction_with`, `create_wallet_descriptor_with`,
`create_wallet_with`, `decode_raw_transaction_with`, `derive_addresses_with`,
`descriptor_process_psbt_with`, `disconnect_node_with`, `dump_tx_out_set_with`,
`estimate_smart_fee_with`, `finalize_psbt_with`, `fund_raw_transaction_with`,
`get_added_node_info_with`, `get_balance_with`, `get_block_filter_with`, `get_block_stats_with`,
`get_chain_tx_stats_with`, `get_deployment_info_with`, `get_descriptor_activity_with`,
`get_hd_keys_with`, `get_index_info_with`, `get_memory_info_with`, `get_network_hash_ps_with`,
`get_new_address_with`, `get_node_addresses_with`, `get_raw_change_address_with`,
`get_raw_mempool_with`, `get_received_by_address_with`, `get_received_by_label_with`,
`get_transaction_with`, `get_tx_out_proof_with`, `get_tx_out_set_info_with`, `get_tx_out_with`,
`help_with`, `import_descriptors_with`, `import_mempool_with`, `key_pool_refill_with`,
`list_descriptors_with`, `list_labels_with`, `list_received_by_address_with`,
`list_received_by_label_with`, `list_since_block_with`, `list_transactions_with`,
`list_unspent_with`, `load_wallet_with`, `lock_unspent_with`, `logging_with`, `migrate_wallet_with`,
`prioritise_transaction_with`, `rescan_blockchain_with`, `restore_wallet_with`, `scan_blocks_with`,
`scan_tx_out_set_with`, `send_all_with`, `send_many_with`, `send_raw_transaction_with`,
`send_to_address_with`, `set_ban_with`, `set_wallet_flag_with`, `sign_raw_transaction_with_key_with`,
`sign_raw_transaction_with_wallet_with`, `simulate_raw_transaction_with`, `submit_block_with`,
`submit_package_with`, `test_mempool_accept_with`, `unload_wallet_with`, `utxo_update_psbt_with`,
`verify_chain_with`, `wait_for_block_with`, `wait_for_block_height_with`, `wait_for_new_block_with`,
`wallet_process_psbt_with`.

## Sync-only methods (no async counterpart at all)

These exist only on the sync client. Grouped by why.

**Different RPC entirely (the async v30 client never binds these wires).** Mostly hidden/regtest or
addrman/zmq/orphan RPCs:

| sync method                            | wire RPC                                                       |
| -------------------------------------- | -------------------------------------------------------------- |
| `add_connection`                       | `addconnection`                                                |
| `add_peer_address`                     | `addpeeraddress`                                               |
| `estimate_raw_fee`                     | `estimaterawfee`                                               |
| `generate_block`                       | `generateblock`                                                |
| `generate_to_address`                  | `generatetoaddress`                                            |
| `generate_to_descriptor`               | `generatetodescriptor`                                         |
| `get_orphan_txs`                       | `getorphantxs`                                                 |
| `get_orphan_txs_verbosity_1`           | `getorphantxs`                                                 |
| `get_orphan_txs_verbosity_2`           | `getorphantxs`                                                 |
| `get_raw_addrman`                      | `getrawaddrman`                                                |
| `get_zmq_notifications`                | `getzmqnotifications`                                          |
| `invalidate_block`                     | `invalidateblock`                                              |
| `mock_scheduler`                       | `mockscheduler`                                                |
| `reconsider_block`                     | `reconsiderblock`                                              |
| `sign_raw_transaction`                 | `signrawtransaction` (deprecated alias of the `_with_*` forms) |
| `sync_with_validation_interface_queue` | `syncwithvalidationinterfacequeue`                             |

**Ergonomic convenience wrappers** (call another RPC and post-process, or hardcode an argument).
These intentionally have no generated async twin; the async client exposes the underlying method
plus its `_with` form instead:

- result re-typed into a richer `rust-bitcoin` value: `best_block_hash` (over `getbestblockhash`),
  `new_address` / `new_address_with_label` / `new_address_with_type` (over `getnewaddress`),
  `server_version` (over `getnetworkinfo`).
- argument hardcoded into a named method: `create_legacy_wallet`, `create_wallet_external_signer`
  (over `createwallet`); `send_many_verbose` (over `sendmany`); `send_to_address_rbf` (over
  `sendtoaddress`); `get_block_stats_by_height`, `get_block_stats_by_block_hash` (over
  `getblockstats`); `get_deployment_info_tip` (over `getdeploymentinfo`); `derive_addresses_multipath`
  (over `deriveaddresses`); `estimate_smart_fee_with_mode` (over `estimatesmartfee`);
  `get_raw_mempool_sequence`, `get_raw_mempool_verbose` (over `getrawmempool`); the
  `scan_blocks_start/status/abort` and `scan_tx_out_set_start/status/abort` action triplets;
  `get_block_verbose_zero`, `get_block_header_verbose`, `get_raw_transaction_verbose`,
  `get_mempool_ancestors_verbose`, `get_mempool_descendants_verbose` (verbosity variants spelled the
  sync way); `unlock_unspent` (over `lockunspent` with the unlock flag); `sign_message_with_privkey`
  (the one remaining cross-spelling twin — async fn `sign_message_with_priv_key`, listed above).
  (`get_network_hash_ps` and `get_addr_man_info` are no longer sync-only: the generator was aligned
  to emit those exact fn names, so they now match.)

**Not RPC methods at all** (struct constructors, included for completeness because they share the
`pub fn` namespace): `new` (on `Output`, `Input`, `ImportDescriptorsRequest`, and `Client` itself).

## Async-only methods (no sync counterpart at all)

These exist only on the async client. Two kinds:

**Genuinely new RPC bindings the sync v30 client does not expose:**

| async method                          | wire RPC                |
| ------------------------------------- | ----------------------- |
| `descriptor_process_psbt` (+ `_with`) | `descriptorprocesspsbt` |
| `get_open_rpc_info`                   | `getopenrpcinfo`        |
| `set_label`                           | `setlabel`              |

**The `_with` optional-argument overloads** (philosophy #1) and the **`_verbose_{word}` level methods**
(philosophy #2). Every one of these shares its base/wire with a sync method but the exact name is
async-only by construction:

- `_with` overloads: the full list is in the matched-core section above (78 total), plus
  `descriptor_process_psbt_with` from the new binding.
- verbosity-level methods whose exact spelling is async-only: `get_block_verbose_one`,
  `get_block_verbose_two`, `get_block_verbose_three` (sync also has one/two/three but additionally
  `get_block_verbose_zero` where async uses bare `get_block`); `get_block_header_verbose_one` (sync:
  `get_block_header_verbose`); `get_raw_transaction_verbose_one`, `get_raw_transaction_verbose_two`
  (sync: `get_raw_transaction_verbose`); `get_mempool_ancestors_verbose_one` (sync:
  `get_mempool_ancestors_verbose`); `get_mempool_descendants_verbose_one` (sync:
  `get_mempool_descendants_verbose`).

## Versions with no async client

The async client is v30-only. The sync client additionally exists for **v17, v18, v19, v20, v21,
v22, v23, v24, v25, v26, v27, v28, v29, v31**. Every method in those modules is, by definition,
sync-only today: there is no async surface to match against until the generator is run for those
versions. The naming rules above (and the codegen rules in the first half of this document) are what
those async surfaces will follow once generated.

# Type naming: generated vs hand-written (corepc-types)

This section does for the response/parameter **types** what the previous one did for client methods:
catalogue every generated type name and reconcile it against the hand-written `corepc-types` names.

- **generated**: the raw types the generator emits, sampled from a v31 run of the generator
  (the raw layer, before the model conversions). 373 `pub struct`/`pub enum` names.
- **hand-written**: the existing `corepc-types` type universe, every `pub struct`/`pub enum` under
  `types/src/**` *excluding* the `generated/` folders (which are themselves generator output). 357
  names across all version modules (v17..v31) plus the shared `model/`.

Comparing by exact name: **127 identical**, 246 generated-only, 230 hand-only. That large
non-overlap is not disagreement about which RPCs exist; it is one deep philosophical difference plus
a few mechanical ones. Read the philosophies first or the lists look like noise.

## The naming philosophies (why the lists diverge)

1. **Nested types: path-derived (generated) vs shared vocabulary (hand-written).** This is the big
   one. For a sub-object like a PSBT input's witness-UTXO scriptPubKey, the generator emits a unique
   name spelling out the full JSON path: `DecodePsbtInputsItemWitnessUtxoScriptPubKey`. corepc-types
   instead has a small curated vocabulary of *shared* nested types reused across every RPC that
   contains that shape: `ScriptPubKey`, `ScriptSig`, `WitnessUtxo`, `PsbtInput`, `PsbtOutput`,
   `Bip32Deriv`, `Proprietary`, `TaprootLeaf`, `Musig2PartialSig`, `MempoolEntry`, `UploadTarget`,
   and so on. So one hand-written `ScriptPubKey` corresponds to several generated `...ScriptPubKey`
   names, and neither side's nested names match the other's. This accounts for the bulk of both the
   generated-only and the non-`*Error` hand-only names.

   **Update — duplication is now cut (structural dedup).** The generator collapses structurally
   identical helper types onto one definition *within each category file* and rewrites every
   reference to it (`dedup_structural` in `src/codegen.rs`, codegen rules section 6a). Equality is
   purely structural — two shapes that differ in any field (`is_change` on gettransaction's vout, the
   optional `prevout` on a verbose-2 vin) stay separate, so a wrong merge is impossible. On v30 this
   removed ~4,150 lines of generated output (the 12 `…ScriptPubKey` shapes collapse to 2 — one per
   category that uses them — and `…ScriptSig` likewise). What it does **not** change is the *names*:
   the surviving type keeps a path-derived name (the lexicographically smallest of the merged group),
   not corepc's shared noun. Merges are kept within a category on purpose because each category is a
   separate file with no cross-imports; collapsing across files would need generated `use` lines.
   Matching corepc's *vocabulary* (`ScriptPubKey`, `PsbtInput`, ...) is the separate, higher-cost
   curated-table problem and is intentionally still out of scope.

2. **Verbosity levels: numeric suffix vs `Verbose`/`WithPrevout` words.** Both number the low level
   the same in places, but the generator always suffixes every level (`...VerboseZero/One/Two`),
   while corepc uses the bare type for verbosity-0 and a single `Verbose` (and `WithPrevout`) word
   for the higher forms. See the cross-spelling table.

3. **Trivial returns: newtype (generated) vs nothing (hand-written).** The generator emits a newtype
   for *every* non-null return, including `Stop(pub String)`, `Uptime(pub i64)`, `Help(pub String)`,
   `PrioritiseTransaction(pub bool)`, `SubmitBlock`, `SendResult`. corepc-types does not define these
   at all: the hand-written client returns the bare `String`/`i64`/`bool`. So they are generated-only.

4. **`*Options` / `*OptionsArg` and param-derived helpers.** The generator emits request-parameter
   structs (`FundRawTransactionOptionsArg`, `BumpFeeOptionsOutputs`, `...SolvingData`, `...PrevTxs`,
   `...ScanObjects`, the union `...V0/V1` arms). corepc-types models request arguments by hand with
   different names (`ImportMulti`, `WalletCreateFundedPsbtInput`, ...) or not as named types at all.
   These never match and are expected to differ (you flagged this). 8 names end in `Options`/
   `OptionsArg`; many more are their nested children.

5. **Error enums live in different files.** corepc-types keeps a `{Type}Error` enum next to each
   fallible type in the same module (94 of the hand-only names end in `Error`). The generator also
   emits these, but into the **model layer** (`generated/model/{category}.rs`), not the raw
   `types.rs` sampled here. So they show as hand-only in this raw-vs-raw comparison, yet the
   generator does produce matching `{Type}Error` names (codegen rules section 11). Compared against
   the generated model files they line up.

## Types present on BOTH sides with the identical name (127)

Byte-for-byte identical generated and hand-written type names. These are the top-level RPC return
types and the few nested helpers where both happened to pick the same word:

```
AbortPrivateBroadcast        GetChainStates               ListReceivedByAddressItem
AbortRescan                  GetChainTips                 ListReceivedByLabel
AnalyzePsbt                  GetChainTxStats              ListReceivedByLabelItem
BumpFee                      GetConnectionCount           ListSinceBlock
CombinePsbt                  GetDeploymentInfo            ListTransactions
CombineRawTransaction        GetDescriptorActivity        ListUnspent
ConvertToPsbt                GetDescriptorInfo            ListUnspentItem
CreateMultisig               GetDifficulty                ListWalletDir
CreatePsbt                   GetHdKeys                    ListWallets
CreateRawTransaction         GetIndexInfo                 LoadTxOutSet
CreateWallet                 GetMempoolCluster            LoadWallet
CreateWalletDescriptor       GetMempoolEntry              LockUnspent
DecodePsbt                   GetMempoolInfo               Logging
DecodeRawTransaction         GetMiningInfo                MigrateWallet
DecodeScript                 GetNetTotals                 PruneBlockchain
DecodeScriptSegwit           GetNetworkInfo               PsbtBumpFee
DeriveAddresses              GetNewAddress                RescanBlockchain
DescriptorProcessPsbt        GetNodeAddresses             RestoreWallet
DumpTxOutSet                 GetPeerInfo                  SaveMempool
EncryptWallet                GetPrioritisedTransactions   SendAll
EnumerateSigners             GetPrivateBroadcastInfo      SendMany
EstimateSmartFee             GetRawChangeAddress          SendRawTransaction
FinalizePsbt                 GetRawMempool                SendToAddress
FundRawTransaction           GetReceivedByAddress         SetNetworkActive
GetAddedNodeInfo             GetReceivedByLabel           SetWalletFlag
GetAddressesByLabel          GetRpcInfo                   SignMessage
GetAddressInfo               GetTransaction               SignMessageWithPrivKey
GetAddressInfoEmbedded       GetTxOut                     SimulateRawTransaction
GetBalance                   GetTxOutSetInfo              SubmitPackage
GetBalances                  GetTxOutSetInfoBlockInfo     TestMempoolAccept
GetBalancesMine              GetTxSpendingPrevout         UnloadWallet
GetBestBlockHash             GetTxSpendingPrevoutItem     UtxoUpdatePsbt
GetBlockchainInfo            GetWalletInfo                ValidateAddress
GetBlockCount                GetWalletInfoScanning        VerifyChain
GetBlockFilter               ImportDescriptors            VerifyMessage
GetBlockHash                 JoinPsbts                    VerifyTxOutProof
GetBlockStats                ListAddressGroupings         WaitForBlock
GetBlockTemplate             ListBanned                   WaitForBlockHeight
GetBlockVerboseOne           ListDescriptors              WaitForNewBlock
GetBlockVerboseThree         ListLabels                   WalletCreateFundedPsbt
GetBlockVerboseTwo           ListLockUnspent              WalletDisplayAddress
GetBlockVerboseZero          ListLockUnspentItem          WalletProcessPsbt
```

## Cross-spelling type pairs (same RPC/role, name differs): `generated -> hand-written`

Where both sides define a type for the same thing but spell it differently. The generated side
follows the mechanical rules (path-derived, always-suffixed); the hand-written side uses the curated
short name.

(`getaddrmaninfo` and `getnetworkhashps` used to drift here — `GetAddrmanInfo`, `GetNetworkHashps` —
but the word lists were aligned, so the generator now emits `GetAddrManInfo` and `GetNetworkHashPs`,
matching legacy. `GetAddrManInfo` is therefore in the identical-name set above; `getnetworkhashps`
has no corepc type either way.)

| role / RPC | generated | -> | hand-written (corepc-types) |
| --- | --- | --- | --- |
| getmemoryinfo result | `GetMemoryInfo` (enum) | -> | `GetMemoryInfoStats` |
| getrawtransaction verbosity 0 | `GetRawTransactionVerboseZero` | -> | `GetRawTransaction` |
| getrawtransaction verbosity 1 | `GetRawTransactionVerboseOne` | -> | `GetRawTransactionVerbose` |
| getrawtransaction verbosity 2 | `GetRawTransactionVerboseTwo` | -> | `GetRawTransactionVerboseWithPrevout` |
| getblockheader verbosity 0 | `GetBlockHeaderVerboseZero` | -> | `GetBlockHeader` |
| getblockheader verbosity 1 | `GetBlockHeaderVerboseOne` | -> | `GetBlockHeaderVerbose` |
| getmempoolancestors verbosity 0 | `GetMempoolAncestorsVerboseZero` | -> | `GetMempoolAncestors` |
| getmempoolancestors verbosity 1 | `GetMempoolAncestorsVerboseOne` | -> | `GetMempoolAncestorsVerbose` |
| getmempooldescendants verbosity 0 | `GetMempoolDescendantsVerboseZero` | -> | `GetMempoolDescendants` |
| getmempooldescendants verbosity 1 | `GetMempoolDescendantsVerboseOne` | -> | `GetMempoolDescendantsVerbose` |
| getblock vrb2/3 tx element | `GetBlockVerboseTwoTxItem` / `...ThreeTxItem` | -> | `GetBlockVerboseTwoTransaction` / `...ThreeTransaction` |
| getblock vrb3 input prevout | `GetBlockVerboseThreeTxItemVinItemPrevout` | -> | `GetBlockVerboseThreePrevout` |
| gettransaction detail element | `GetTransactionDetailsItem` | -> | `GetTransactionDetail` |
| gettxoutsetinfo unspendables | `GetTxOutSetInfoBlockInfoUnspendables` | -> | `GetTxOutSetInfoUnspendables` |
| scanblocks `start`/`status`/`abort` | `ScanBlocksV1` / `ScanBlocksV2` (arms of one enum `ScanBlocks`) | -> | `ScanBlocksStart` / `ScanBlocksStatus` / `ScanBlocksAbort` |
| scantxoutset `start`/`status` | `ScanTxOutSetV0` / `ScanTxOutSetV2` (arms of `ScanTxOutSet`) | -> | `ScanTxOutSetStart` / `ScanTxOutSetStatus` (+ `ScanTxOutSetAbort`) |
| scantxoutset unspent element | `ScanTxOutSetV0UnspentsItem` | -> | `ScanTxOutSetUnspent` |
| any output script sub-object | `{Path}ScriptPubKey` (e.g. `GetTxOutV1ScriptPubKey`) | -> | `ScriptPubKey` (one shared type) |
| any input script sub-object | `{Path}ScriptSig` | -> | `ScriptSig` |
| any segwit-utxo sub-object | `{Path}WitnessUtxo` | -> | `WitnessUtxo` |
| psbt input / output object | `DecodePsbtInputsItem` / `DecodePsbtOutputsItem` | -> | `PsbtInput` / `PsbtOutput` |
| bip32 derivation element | `{Path}Bip32DerivsItem` | -> | `Bip32Deriv` |
| bip9 softfork info | `GetDeploymentInfoDeploymentsBip9` | -> | `Bip9SoftforkInfo` / `Bip9Info` |
| mempool-entry fees | `GetMempoolEntryFees` (matches) / `GetRawMempoolV1Fees` | -> | `MempoolEntryFees` |

The pattern: a generated name is the parent type plus the field path plus `Item`/`Vn`; the matching
corepc name is a single domain noun shared by every site. To go from one to the other you strip the
path prefix and the `Item`/`Vn`/`Verbose{Level}` decoration and map to the curated noun.

## Generated-only type names (246)

No hand-written corepc-types name. By bucket (philosophies 1, 3, 4 above):

- **trivial-return newtypes corepc omits** (philosophy 3): `Stop`, `Uptime`, `Help`,
  `PrioritiseTransaction`, `SendResult`, `SubmitBlock`, `GetTxOutProof`, `GetNetworkHashPs`,
  `GetBlockFromPeer`, `ImportMempool`, and the rest of the no-args/primitive-return set.
- **`*Options` / `*OptionsArg` request structs and their children** (philosophy 4; 8 ending in
  `Options`/`OptionsArg` plus their nested `...SolvingData`, `...InputWeightsItem`, `...FeeRate`,
  `...PrevTxs`, `...ScanObjects`, `...Inputs`, `...Outputs`, `...Recipients`, `...Descriptors`,
  `...Amount`, `...Range` helpers). You already excluded these from the expected match set.
- **path-derived nested helpers** (philosophy 1) — the large remainder: every `{Parent}...Item`,
  `{Parent}...ScriptPubKey`, `{Parent}...ScriptSig`, `{Parent}...Prevout`, `{Parent}...V{n}`,
  `{Parent}...CoinbaseTx`, `{Parent}...Fees`, `{Parent}...Locked`, etc. that corepc covers with a
  shared short type. Counts in this file: 70 end in `Item`, 31 in `V{n}`, 23 in a script/prevout
  noun.

## Hand-written-only type names (230)

No generated name. By bucket:

- **`{Type}Error` model enums** (philosophy 5; 94 of them): `BumpFeeError`, `GetBlockHeaderError`,
  `GetTxOutError`, `DecodePsbtError`, ... The generator emits matching names but into its model
  layer, not the raw `types.rs` compared here.
- **shared nested-vocabulary types** (philosophy 1; the non-`Error` remainder, 136): `ScriptPubKey`,
  `ScriptSig`, `WitnessUtxo`, `PsbtInput`, `PsbtOutput`, `PsbtScript`, `Bip32Deriv`,
  `TaprootBip32Deriv`, `TaprootLeaf`, `TaprootScript`, `TaprootScriptPathSig`, `Proprietary`,
  `Musig2PartialSig`, `Musig2ParticipantPubKeys`, `Musig2Pubnonce`, `GlobalXpub`, `MempoolEntry`,
  `MempoolEntryFees`, `MempoolAcceptance`, `MempoolAcceptanceFees`, `UploadTarget`, `NextBlockInfo`,
  `Locked`, `ChainState`, `Softfork`/`SoftforkType`/`SoftforkReject`, `Bip9Softfork`/`Bip9Info`/
  `Bip9SoftforkStatus`/`Bip9SoftforkStatistics`, `ScriptType`, `TransactionCategory`,
  `Bip125Replaceable`, `AddressPurpose`, `InputKeySource`, `FinalScript`, `RawTransactionInput`/
  `RawTransactionOutput`/`RawTransactionInputWithPrevout`, `SpendActivity`/`ReceiveActivity`/
  `ActivityEntry`, `SignFail`, `RemovedTransaction`, `TransactionItem`, ... These are the curated
  shapes the generator instead spells out per path.
- **types for RPCs the sampled v31 generated file does not bind**, mostly hand-written-client
  conveniences and hidden/legacy RPCs: `AddConnection`, `AddPeerAddress`, `AddMultisigAddress`,
  `EstimateRawFee`, `Generate`/`GenerateBlock`/`GenerateToAddress`/`GenerateToDescriptor`,
  `GetOrphanTxs`/`GetOrphanTxsVerboseOne`/`...Two` (+ their `...Entry`), `GetRawAddrMan`/
  `RawAddrManEntry`, `GetZmqNotifications`, `GetUnconfirmedBalance`, `GetBalancesWatchOnly`,
  `DumpPrivKey`, `DumpWallet`, `UpgradeWallet`, `SetTxFee`, `Send` (the model type), `ImportMulti`/
  `ImportMultiEntry`, `DeriveAddressesMultipath`, `GetRawMempoolSequence`/`GetRawMempoolVerbose`,
  `SendManyVerbose`, the `corepc`-style `ScanBlocksStart`/`Status`/`Abort` and `ScanTxOutSetStart`/
  `Status`/`Abort`/`Unspent` split (listed as cross-spelling above), and various `*Network`/
  `*Address`/`*Name`/`*Label` member structs (`GetNetworkInfoNetwork`, `AddrManInfoNetwork`,
  `GetIndexInfoName`, `GetAddressInfoLabel`, ...).

## How to map a generated type to its corepc-types counterpart

A mechanical recipe that covers most of the diff:

1. If it ends in `Options`/`OptionsArg` or is a child of one, expect no match (request params differ).
2. Strip a trailing `Verbose{Zero|One|Two|Three}` decoration: verbosity-0 maps to the bare corepc
   type (`GetRawTransactionVerboseZero` -> `GetRawTransaction`), higher levels to corepc's `Verbose`
   / `WithPrevout` word.
3. Strip the parent-path prefix and the `Item`/`V{n}` suffix from a nested name, then map the
   remaining leaf role to corepc's shared noun (`...VinItemScriptSig` -> `ScriptSig`,
   `...InputsItem` under decodepsbt -> `PsbtInput`).
4. For a fallible type, the `{Type}Error` enum is in the generated **model** module, not the raw one.
5. If it is a trivial-return newtype (`Stop`, `Uptime`, ...), corepc has no type; the client returns
   the primitive.
