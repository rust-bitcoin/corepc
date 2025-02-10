// SPDX-License-Identifier: CC0-1.0

//! Helper methods.

pub mod method;
pub mod model;
pub mod ssot;
pub mod versioned;

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

/// Supported Bitcoin Core versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Version {
    /// Bitcoin Core v17.
    V17,
    /// Bitcoin Core v18.
    V18,
    /// Bitcoin Core v19.
    V19,
    /// Bitcoin Core v20.
    V20,
}

impl Version {
    /// Creates a new `Version` from string.
    pub fn new(v: &str) -> Result<Version> {
        match v {
            "v17" | "17" => Ok(Version::V17),
            "v18" | "18" => Ok(Version::V18),
            "v19" | "19" => Ok(Version::V19),
            "v20" | "20" => Ok(Version::V20),
            other => Err(anyhow::Error::msg(format!("unknown version: '{}'", other))),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Version::*;
        let s = match *self {
            V17 => "v17",
            V18 => "v18",
            V19 => "v19",
            V20 => "v20",
        };
        fmt::Display::fmt(&s, f)
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> { Version::new(s) }
}

/// Checks that `got` contains all methods from `want` and no additional methods.
///
/// # Returns
///
/// `true` if all methods are correct, `false` otherwise.
pub fn correct_methods(got: &[&str], want: &[&str], msg: &str) -> bool {
    let mut correct = true;
    let ret = has_all_expected(got, want);
    if !ret.is_empty() {
        eprintln!("\nMissing methods ({}):", msg);
        correct = false;
        for method in ret {
            eprintln!(" - {}", method);
        }
        eprintln!();
    }

    let ret = has_no_additional(got, want);
    if !ret.is_empty() {
        eprintln!("Unexpected additional methods ({}):", msg);
        correct = false;
        for method in ret {
            eprintln!(" - {}", method);
        }
        eprintln!();
    }
    correct
}

/// Checks that all methods in `want` exist in `got`.
///
/// # Returns
///
/// A list of any methods found to be missing.
fn has_all_expected<'b>(got: &[&str], want: &[&'b str]) -> Vec<&'b str> {
    let mut missing = vec![];
    for method in want {
        if !got.contains(method) {
            missing.push(*method);
        }
    }
    missing
}

/// Checks that no methods in `got` exist in `want`.
///
/// # Returns
///
/// A list of any methods found to exist when they should not.
fn has_no_additional<'a>(got: &[&'a str], want: &[&str]) -> Vec<&'a str> {
    let mut additional = vec![];
    // We did not get any additional methods we didn't expect.
    for method in got {
        if !want.contains(method) {
            additional.push(*method);
        }
    }
    additional
}

/// Opens file at `path` and greps for `s`.
pub fn grep_for_string(path: &Path, s: &str) -> Result<bool> {
    let file = File::open(path)
        .with_context(|| format!("Failed to grep for string in {}", path.display()))?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(s)?;

    for line in reader.lines() {
        let line = line?;

        if re.is_match(&line) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Opens file at `path` and greps for `s,`.
///
/// Note the `,` appended to `s`. This is to stop false positives `grep_for_string(Foo)`
/// will match `FooBar`. Re-exports always have a comma after them.
pub fn grep_for_re_export(path: &Path, s: &str) -> Result<bool> {
    let file = File::open(path)
        .with_context(|| format!("Failed to grep for string in {}", path.display()))?;
    let reader = io::BufReader::new(file);

    let s = format!("{}[,}}]", &s);
    let re = Regex::new(&s)?;

    for line in reader.lines() {
        let line = line?;

        if re.is_match(&line) {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        let got = vec!["one", "two", "three"];
        let want = vec!["one", "two", "three"];

        assert!(has_all_expected(&got, &want).is_empty());
        assert!(has_no_additional(&got, &want).is_empty());
    }

    #[test]
    fn missing_from_beginning() {
        let got = vec!["two", "three"];
        let want = vec!["one", "two", "three"];
        assert_eq!(has_all_expected(&got, &want), &["one"]);
    }

    #[test]
    fn missing_from_middle() {
        let got = vec!["one", "three"];
        let want = vec!["one", "two", "three"];
        assert_eq!(has_all_expected(&got, &want), &["two"]);
    }

    #[test]
    fn missing_from_end() {
        let got = vec!["one", "two"];
        let want = vec!["one", "two", "three"];
        assert_eq!(has_all_expected(&got, &want), &["three"]);
    }

    #[test]
    fn has_additional_at_beginning() {
        let got = vec!["one", "two", "three"];
        let want = vec!["two", "three"];

        assert_eq!(has_no_additional(&got, &want), &["one"]);
    }

    #[test]
    fn has_additional_in_middle() {
        let got = vec!["one", "two", "three"];
        let want = vec!["one", "three"];

        assert_eq!(has_no_additional(&got, &want), &["two"]);
    }

    #[test]
    fn has_additional_at_end() {
        let got = vec!["one", "two", "three"];
        let want = vec!["one", "two"];

        assert_eq!(has_no_additional(&got, &want), &["three"]);
    }
}
