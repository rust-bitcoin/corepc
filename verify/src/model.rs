// SPDX-License-Identifier: CC0-1.0

//! Things related to parsing the model files.
//!
//! The "model files" are the files in `types/src/model/`.

use std::path::PathBuf;

use anyhow::Result;

use crate::method::{self, Return};
use crate::Version;

/// Path to the model module file.
fn path() -> PathBuf { PathBuf::from("../types/src/model/mod.rs") }

/// Returns `true` if this method requires a type to exist.
pub fn requires_type(version: Version, method_name: &str) -> Result<bool> {
    let method = match method::Method::from_name(version, method_name) {
        Some(m) => m,
        None =>
            return Err(anyhow::Error::msg(format!(
                "model type for method not found: {}",
                method_name
            ))),
    };

    Ok(method.requires_model)
}

/// Checks that a type exists in `model` module.
pub fn type_exists(version: Version, method_name: &str) -> Result<bool> {
    let method = match method::Method::from_name(version, method_name) {
        Some(m) => m,
        None =>
            return Err(anyhow::Error::msg(format!(
                "model type for method not found: {}",
                method_name
            ))),
    };

    if let Some(Return::Type(s)) = method.ret {
        return crate::grep_for_re_export(&path(), s);
    }
    Ok(false)
}

/// Checks if a type exists in `model` module regardless of whether it's required.
pub fn type_physically_exists(version: Version, method_name: &str) -> Result<bool> {
    let method = match method::Method::from_name(version, method_name) {
        Some(m) => m,
        None =>
            return Err(anyhow::Error::msg(format!(
                "model type for method not found: {}",
                method_name
            ))),
    };

    if let Some(Return::Type(s)) = method.ret {
        return crate::grep_for_string(&path(), s);
    }
    Ok(false)
}
