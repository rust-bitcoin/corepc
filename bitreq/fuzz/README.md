# Fuzzing bitreq

This directory contains fuzzing infrastructure for the `bitreq` crate, specifically targeting the `Url` parser.

## Structure

- `src/url_parse.rs` - Fuzz target for the URL parser
- `src/bin/target_template.txt` - Template for generating fuzz target binaries
- `src/bin/gen_target.sh` - Script to generate target binaries from template
- `ci-fuzz.sh` - CI script for running fuzzing tests

## Running Fuzz Tests

### With stdin (for quick testing)

```bash
cd fuzz
RUSTFLAGS="--cfg=fuzzing" cargo build --features stdin_fuzz --bin url_parse_target
echo "http://example.com" | ./target/debug/url_parse_target
```

### With honggfuzz (for comprehensive fuzzing)

```bash
cd fuzz
cargo install honggfuzz
export RUSTFLAGS="--cfg=fuzzing"
export HFUZZ_BUILD_ARGS="--features honggfuzz_fuzz"
cargo hfuzz build
HFUZZ_RUN_ARGS="--exit_upon_crash -v -n8 --run_time 30" cargo hfuzz run url_parse_target
```

### With AFL (alternative fuzzer)

```bash
cd fuzz
cargo install afl
export RUSTFLAGS="--cfg=fuzzing"
cargo afl build --features afl_fuzz --bin url_parse_target
cargo afl fuzz -i seeds -o findings target/debug/url_parse_target
```

### Running CI Fuzzing

The `ci-fuzz.sh` script automates the fuzzing process with honggfuzz:

```bash
cd fuzz
./ci-fuzz.sh
```

This will:
1. Regenerate fuzz targets
2. Install honggfuzz
3. Build fuzz targets
4. Run each target for 30 seconds
5. Report any crashes found

## Running Tests

The fuzz targets include unit tests that can be run with:

```bash
cd fuzz
RUSTFLAGS="--cfg=fuzzing" cargo test
```

## Adding New Fuzz Targets

1. Create a new module in `src/` (e.g., `src/my_target.rs`)
2. Export it from `src/lib.rs`
3. Add a `GEN_TEST my_target` line to `src/bin/gen_target.sh`
4. Run `cd src/bin && ./gen_target.sh` to generate the target binary
