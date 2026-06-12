# Integration testing

This crate is used to run tests against all the supported versions of
Core. It runs Core by using `bitcoind` with the `download` feature
enabled. However `bitcoind` allows setting the environment variable
`BITCOIND_EXE` to override downloading the Core executable. E.g.

`BITCOIND_EXE=/opt/bitcoin-28.0/bin/bitcoind cargo test --features=28_0`

## All features

Note that this crate cannot be built with `--all-features` because of
how the internal `v28_and_below` style feature gates work.

## Shell alias' for the impatient

I have all the Core versions on my machine e.g., `/opt/bitcoin-28.0`
then I use the following shell alias' to run tests

```bash
alias test17='BITCOIND_EXE=/opt/bitcoin-0.17.2/bin/bitcoind cargo test --features=0_17_2'
alias test18='BITCOIND_EXE=/opt/bitcoin-0.18.1/bin/bitcoind cargo test --features=0_18_1'
alias test19='BITCOIND_EXE=/opt/bitcoin-0.19.1/bin/bitcoind cargo test --features=0_19_1'
alias test20='BITCOIND_EXE=/opt/bitcoin-0.20.2/bin/bitcoind cargo test --features=0_20_2'
alias test21='BITCOIND_EXE=/opt/bitcoin-0.21.2/bin/bitcoind cargo test --features=0_21_2'
alias test22='BITCOIND_EXE=/opt/bitcoin-22.1/bin/bitcoind cargo test --features=22_1'
alias test23='BITCOIND_EXE=/opt/bitcoin-23.2/bin/bitcoind cargo test --features=23_2'
alias test24='BITCOIND_EXE=/opt/bitcoin-24.2/bin/bitcoind cargo test --features=24_2'
alias test25='BITCOIND_EXE=/opt/bitcoin-25.2/bin/bitcoind cargo test --features=25_2'
alias test26='BITCOIND_EXE=/opt/bitcoin-26.2/bin/bitcoind cargo test --features=26_2'
alias test27='BITCOIND_EXE=/opt/bitcoin-27.2/bin/bitcoind cargo test --features=27_2'
alias test28='BITCOIND_EXE=/opt/bitcoin-28.2/bin/bitcoind cargo test --features=28_2'
alias test29='BITCOIND_EXE=/opt/bitcoin-29.0/bin/bitcoind cargo test --features=29_0'
alias test30='BITCOIND_EXE=/opt/bitcoin-30.2/bin/bitcoind cargo test --features=30_2'
alias test31='BITCOIND_EXE=/opt/bitcoin-31.0/bin/bitcoind cargo test --features=31_0'
```

## Bitcoin Core Tests

Tests derived from Bitcoin Core's `test/functional/` folder live in
`tests/` alongside the modelled tests, with a `_core` filename suffix.

## Async Integration Tests

The async client ships with a blocking facade so the same integration
tests can be run against it without changes. Enable the `test-async`
feature alongside a single version feature. It pulls in `bitcoind`'s
`client-async` feature, which swaps the sync client for the async
client's blocking wrapper.

`test-async` is mutually exclusive with the same constraints as the
version features: pick one version, and don't combine it with
`--all-features`.

Note that a workspace `--all-features` build (which runs over `bitcoind`,
not this excluded crate) still resolves to the sync client. Enabling every
version feature turns on `31_0`, which disables the `client-async` blocking
re-export in `bitcoind`, so `--all-features` lands on the sync `v31` client
rather than the async facade. See `bitcoind/src/client_versions.rs`.

```bash
cargo test -p integration-test --no-default-features --features 30_2,test-async,download
```

As with the sync tests, set `BITCOIND_EXE` to skip the download:

```bash
BITCOIND_EXE=/opt/bitcoin-30.2/bin/bitcoind \
  cargo test -p integration-test --no-default-features --features 30_2,test-async
```
