# codegen

A self-contained Rust tool that generates Rust bindings for the Bitcoin Core JSON-RPC API from
Bitcoin Core's own OpenRPC export.

The generated files are committed in the sibling `corepc-types` and `corepc-client` crates. The
tool runs only when a maintainer regenerates bindings. The crate is `btc-codegen`, a non-published
member of the corepc workspace; it is not built or depended on by the consumer crates.

## What it produces

For a Bitcoin Core version `N` with a spec under `specs/`, the tool writes per-category modules
into the two consumer crates:

```
types/src/v{N}/generated/
- mod.rs                    module entry point + re-exports
- {category}.rs             raw response types
- model/{category}.rs       model conversions (`into_model()`) + error enums

client/src/client_async/v{N}/
- mod.rs                    module entry point + re-exports
- {category}.rs             *Options request structs + `impl Client` async method wrappers
```

Categories mirror Bitcoin Core's help sections (`blockchain`, `wallet`, `rawtransactions` as
`raw_transactions`, and so on).

## Optional arguments

Bitcoin Core's RPC takes optional arguments positionally, with `null` as the "use default"
sentinel. The tool emits two methods per RPC that has any optional parameters:

```rust
// Required-only, uses Core's defaults for everything optional.
client.send_raw_transaction(hex).await?;

// With-options, every optional field is `Option<T>`. An unset field serialises as JSON
// `null` (or is omitted from an options object), which Core treats as "use the default".
client.send_raw_transaction_with(
    hex,
    SendRawTransactionOptions { max_fee_rate: Some(0.5), ..Default::default() },
).await?;
```

## Regenerate bindings

From the corepc workspace root (this also reformats the emitted files):

```bash
just codegen 30
```

Or directly from this directory (no reformat):

```bash
just codegen 30
just test
just lint
```

## Add a new Bitcoin Core version

1. Apply Bitcoin Core's spec-export patch (a `bitcoin-cli getopenrpcinfo` extension, see
   `specs/README.md`) to a checkout matching the version you want, build it, and copy the
   resulting JSON into `specs/v{major}_{minor}_{patch}_openrpc.json`.
2. Regenerate from the corepc workspace root: `just codegen {major}`.
3. Declare the new modules in the consumer crates. `pub mod generated;` already exists in
   `types/src/v{N}/mod.rs` for generated versions, and the client version module is declared in
   `client/src/client_async/mod.rs`. Add the version feature in `client/Cargo.toml`.

## Design notes

Every spec change requires a maintainer to run `just codegen` and commit the diff. The module docs
in `src/codegen.rs` cover the architecture and `src/names.rs` documents the naming rules.

### Identifier conventions

The generator carries one curated word list (`WORDS` in `src/names.rs`) used by a longest-match
scan with a clean-boundary rule to split all-lowercase compounds (`getblockheader` into
`get_block_header` / `GetBlockHeader`). When a new Core release adds a noun the splitter doesn't
know, the output will look off. Add the noun to `WORDS` and regenerate. The `src/names.rs` module
docs cover every rule the generator applies.

### Integer vs float for `type: number`

Bitcoin Core's OpenRPC export uses `type: number` for many fields that are semantically integers
(block height, verbosity level, conf targets). `INTEGER_PARAM_NAMES` and the `classify_number`
table in `codegen.rs` recover the proper Rust type. Anything unrecognised stays a signed integer
for parameters classified as integer-like, `f64` otherwise.

A future improvement is to patch Bitcoin Core's OpenRPC generator to emit `type: integer`
directly, at which point these tables shrink.

### Doc-comment escaping

Bitcoin Core's docstrings contain bare URLs, bracketed range notation (`[begin,end]`) and
angle-bracketed placeholders (`<wallet name>`). The generator escapes brackets and wraps bare URLs
in Markdown autolinks (`esc_doc` in `codegen.rs`) so the generated docs pass rustdoc with
`-D warnings`.

### Reserved type-name collisions

A handful of method names produce PascalCase types that collide with std prelude items, notably
`send` into `Send` (which would shadow the `Send` trait). The `RESERVED_TYPE_NAMES` table in
`codegen.rs` rewrites these to non-colliding idents (`SendResult`) for both the emitted type and
the method's return-type expression.
