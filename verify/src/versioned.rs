// SPDX-License-Identifier: CC0-1.0

//! Things related to parsing the version specific module file.
//!
//! The "version specific module file" is for example `types/src/v17/mod.rs`.

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

use crate::method::{self, Return};
use crate::{paths, Version};

/// Parses module rustdocs and gets all the method names.
pub fn all_methods(version: Version) -> Result<Vec<String>> {
    let methods = methods_and_status(version)?;
    Ok(methods.iter().map(|m| m.name.to_string()).collect())
}

/// Parses module rustdocs and grabs each method and its current status.
pub fn methods_and_status(version: Version) -> Result<Vec<Method>> {
    let path = paths::versioned_mod(version);
    let file = File::open(&path)
        .with_context(|| format!("Failed to grep rustdocs in {}", path.display()))?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"\/\/\! \| ([a-z]+) .* \| ([a-z +]+?) \|.*\|").unwrap();

    let mut methods = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let override_status = if line.contains("UNTESTED") {
            Some(Status::Untested)
        } else if line.contains("TODO") {
            Some(Status::Todo)
        } else {
            None
        };

        if let Some(caps) = re.captures(&line) {
            let method_name = caps.get(1).unwrap().as_str();
            let returns_column = caps.get(2).unwrap().as_str();

            let status = match override_status {
                Some(status) => status,
                None => returns_column.trim().parse::<Status>()?,
            };

            methods.push(Method { name: method_name.to_string(), status });
        }
    }
    Ok(methods)
}

/// Returns `true` if this method requires a type to exist.
pub fn requires_type(version: Version, method_name: &str) -> Result<bool> {
    let method = match method::Method::from_name(version, method_name) {
        Some(m) => m,
        None =>
            return Err(anyhow::Error::msg(format!(
                "return type for method not found: {}",
                method_name
            ))),
    };

    Ok(matches!(method.ret, Some(Return::Type(_))))
}

/// Checks that a type exists in version specific module.
pub fn type_exists(version: Version, method_name: &str) -> Result<bool> {
    let path = paths::versioned_mod(version);
    let method = match method::Method::from_name(version, method_name) {
        Some(m) => m,
        None =>
            return Err(anyhow::Error::msg(format!(
                "return type for method not found: {}",
                method_name
            ))),
    };

    if let Some(Return::Type(s)) = method.ret {
        return crate::grep_for_re_export(&path, s);
    }
    Ok(false)
}

/// The value from the `Returns` column in the versioned rustdoc table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnsDoc {
    /// The table says the method is implemented for this version and no model is required.
    Version,
    /// The table says the method is implemented for this version and a model is required.
    VersionPlusModel,
    /// Any other entry (omitted, TODO, returns nothing/numeric/bool/string, etc.).
    Other(String),
}

/// Parses the version-specific module rustdocs and returns a mapping to `ReturnsDoc`.
pub fn returns_map(version: Version) -> Result<std::collections::BTreeMap<String, ReturnsDoc>> {
    use std::collections::BTreeMap;

    let path = paths::versioned_mod(version);
    let file = File::open(&path)
        .with_context(|| format!("Failed to grep rustdocs in {}", path.display()))?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"//! \| ([a-z]+) .* \| ([a-z +]+?) \|.*\|").unwrap();

    let mut map: BTreeMap<String, ReturnsDoc> = BTreeMap::new();
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            let method_name = caps.get(1).unwrap().as_str();
            let returns_column = caps.get(2).unwrap().as_str().trim();

            let entry = match returns_column {
                "version" => ReturnsDoc::Version,
                "version + model" => ReturnsDoc::VersionPlusModel,
                other => ReturnsDoc::Other(other.to_string()),
            };
            map.insert(method_name.to_string(), entry);
        }
    }

    Ok(map)
}

/// A list item from rustdocs (e.g. in in `types/src/v17/mod.rs`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// TODO: This name is overloaded (`method::Method`).
pub struct Method {
    /// The JSON RPC method name.
    pub name: String,
    /// The current implementation status for this method.
    pub status: Status,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (status: {})", self.name, self.status)
    }
}

/// Possible status for a method.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    /// Done - implemented and tested.
    Done,
    /// Intentionally omitted (likely because deprecated).
    Omitted,
    /// Implemented but not yet tested.
    Untested,
    /// Still to do.
    Todo,
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "version" => Ok(Status::Done),
            "version + model" => Ok(Status::Done),
            "omitted" => Ok(Status::Omitted),
            "returns nothing" | "returns numeric" | "returns boolean" | "returns string" =>
                Ok(Status::Done),
            other => Err(anyhow::Error::msg(format!("unknown status: '{}'", other))),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Status::*;
        let s = match self {
            Done => "done",
            Omitted => "omitted",
            Untested => "done (untested)",
            Todo => "todo",
        };
        fmt::Display::fmt(&s, f)
    }
}
