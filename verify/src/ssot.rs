// SPDX-License-Identifier: CC0-1.0

//! Things related to parsing the JSON RPC Single Source Of Truth (SSOT) file.
//!
//! The SSOT used by this tool is a file that contains the output of
//!
//! `bitcoin-cli --help`
//!
//! Run against the version of Core that we are verifying.

use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{Context, Result};
use regex::Regex;

use crate::{paths, Version};

/// Parses the Bitcoin Core docs (from SSOT file) and gets all the method names.
pub fn all_methods(version: Version) -> Result<Vec<String>> {
    let path = paths::ssot(version);
    let file = File::open(&path)
        .with_context(|| format!("Failed to grep for method names in {}", path.display()))?;
    let reader = io::BufReader::new(file);

    let header_re = Regex::new(r"==").unwrap();
    let empty_re = Regex::new(r"^$").unwrap();

    let mut methods = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if header_re.is_match(&line) || empty_re.is_match(&line) {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        // We know this is not empty because of `empty_re`.
        methods.push(parts[0].to_string());
    }

    Ok(methods)
}
