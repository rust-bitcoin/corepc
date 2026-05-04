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
```
