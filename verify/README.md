# Verify

TL;DR: `cargo run -- --help`

This crate provides a tool to help verify the rest of the repository.
Specifically the Single Source Of Truth for a specific Core version is
the version module in `types` e.g., `types/src/v17/mod.rs`.

Verification is warranted because there are many pieces involved and
during development much cut'n'paste. We want to catch our mistakes and
have some confidence that our claims are valid.

Please see `types/README.md` for more information.


