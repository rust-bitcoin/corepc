// SPDX-License-Identifier: CC0-1.0

//! Paths to files used by the verify tool.

use std::path::PathBuf;

use crate::Version;

/// Returns the path to the `verify` crate root directory.
fn crate_dir() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")) }

/// Returns the path to `types/src`.
pub fn types_src_dir() -> PathBuf { crate_dir().join("../types/src") }

/// Path to the RPC SSOT file.
pub fn ssot(version: Version) -> PathBuf { crate_dir().join(format!("rpc-api-{}.txt", version)) }

/// Path to the version specific module file.
pub fn versioned_mod(version: Version) -> PathBuf {
    types_src_dir().join(format!("{}/mod.rs", version))
}

/// Path to the model module file.
pub fn model_mod() -> PathBuf { types_src_dir().join("model/mod.rs") }
