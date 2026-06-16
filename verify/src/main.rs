// SPDX-License-Identifier: CC0-1.0

//! Tool to help verify that what we claim in the rustdocs is true.
//!
//! Currently verifies:
//!
//! - That the correct set of methods is documented.
//! - That an expected return type is provided if the method is supported.
//! - That there is a `model` type if required.
//! - That the method has an integration test.
//! - That re-exports in `corepc-types` are complete.

use std::process;

use anyhow::Result;
use clap::{arg, Command};
use verify::method::{Method, Return};
use verify::versioned::{self, Status};
use verify::{method, model, reexports, ssot, Version};

// TODO: Enable running from any directory, currently errors if run from `src/`.
// TODO: Add a --quiet option.

const VERSIONS: [Version; 15] = [
    Version::V17,
    Version::V18,
    Version::V19,
    Version::V20,
    Version::V21,
    Version::V22,
    Version::V23,
    Version::V24,
    Version::V25,
    Version::V26,
    Version::V27,
    Version::V28,
    Version::V29,
    Version::V30,
    Version::V31,
];

fn main() -> Result<()> {
    let cmd = Command::new("verify").args([
        arg!([version] "Verify specific version of Core (use \"all\" for all versions)")
            .required(true),
        arg!(-t --tests <TEST_OUTPUT> "Optionally check claimed status of tests").required(false),
        arg!(-q --quiet ... "Run tests in quiet mode").required(false),
    ]);

    let matches = cmd.clone().get_matches();
    let version = matches.get_one::<String>("version").unwrap();
    let test_output = matches.get_one::<String>("tests");
    let quiet = matches.get_one::<u8>("quiet") == Some(&1);

    if version == "all" {
        verify_all_versions(test_output, quiet)?;
    } else if let Ok(v) = version.parse::<Version>() {
        verify_version(v, test_output, quiet)?;
    } else {
        eprint!("Unrecognised version: {} (supported versions: ", version);
        eprint!("{} - {}", VERSIONS[0], VERSIONS[VERSIONS.len() - 1]);
        eprintln!(")");
        process::exit(1);
    }
    Ok(())
}

fn verify_all_versions(test_output: Option<&String>, quiet: bool) -> Result<()> {
    let mut any_failed = false;
    for version in VERSIONS {
        println!("\nVerifying for Bitcoin Core version {} ...", version);
        if verify_version(version, test_output, quiet).is_err() {
            any_failed = true;
        }
    }
    if any_failed {
        return Err(anyhow::anyhow!("verification failed for one or more versions"));
    }
    Ok(())
}

fn verify_version(version: Version, test_output: Option<&String>, quiet: bool) -> Result<()> {
    let mut failures = 0;

    let s = format!("{}::METHOD data", version);
    let msg = format!("Checking that the {} list is correct", s);
    check(&msg, quiet);
    match verify_correct_methods(version, method::all_methods(version), &s) {
        Ok(()) => close(true, quiet),
        Err(e) => {
            if !quiet {
                eprintln!("{}", e);
            }
            close(false, quiet);
            failures += 1;
        }
    }

    let s = "rustdoc version specific rustdocs";
    let msg = format!("Checking that the {} list is correct", s);
    check(&msg, quiet);
    match verify_correct_methods(version, versioned::all_methods(version)?, s) {
        Ok(()) => close(true, quiet),
        Err(e) => {
            if !quiet {
                eprintln!("{}", e);
            }
            close(false, quiet);
            failures += 1;
        }
    }

    let msg = "Checking that the status claimed in the version specific rustdocs is correct";
    check(msg, quiet);
    match verify_status(version, test_output) {
        Ok(()) => close(true, quiet),
        Err(e) => {
            if !quiet {
                eprintln!("{}", e);
            }
            close(false, quiet);
            failures += 1;
        }
    }

    let msg = "Checking that 'Returns' column matches model requirements";
    check(msg, quiet);
    match verify_returns_method(version) {
        Ok(()) => close(true, quiet),
        Err(e) => {
            if !quiet {
                eprintln!("{}", e);
            }
            close(false, quiet);
            failures += 1;
        }
    }

    let msg = "Checking that corepc-types re-exports are complete";
    check(msg, quiet);
    match reexports::check_type_reexports(version) {
        Ok(()) => close(true, quiet),
        Err(e) => {
            if !quiet {
                eprintln!("{}", e);
            }
            close(false, quiet);
            failures += 1;
        }
    }

    if failures > 0 {
        return Err(anyhow::anyhow!("verification failed ({} check(s) failed)", failures));
    }
    Ok(())
}

fn check(msg: &str, quiet: bool) {
    if !quiet {
        println!("{} ... ", msg);
    }
}

fn close(correct: bool, quiet: bool) {
    if quiet {
        return;
    }
    if correct {
        println!("Correct \u{2713} \n");
    } else {
        println!("\u{001b}[31mIncorrect \u{2717}\u{001b}[0m \n");
    }
}

/// Verifies that the correct set of methods are documented.
fn verify_correct_methods(version: Version, methods: Vec<String>, msg: &str) -> Result<()> {
    let ssot = ssot::all_methods(version)?;
    let want = ssot.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let got = methods.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    if !verify::correct_methods(&got, &want, msg) {
        return Err(anyhow::anyhow!("incorrect {}", msg));
    }
    Ok(())
}

/// Verifies that the status we claim is correct.
fn verify_status(version: Version, test_output: Option<&String>) -> Result<()> {
    let methods = versioned::methods_and_status(version)?;
    let mut failures = 0;
    for method in methods {
        match method.status {
            Status::Done => {
                if check_types_exist_if_required(version, &method.name).is_err() {
                    failures += 1;
                }

                if let Some(test_output) = test_output {
                    if check_integration_test_crate::test_exists(version, &method.name, test_output)
                        .is_err()
                    {
                        eprintln!("missing integration test: {}", method.name);
                        failures += 1;
                    }
                }
            }
            Status::Untested => {
                if check_types_exist_if_required(version, &method.name).is_err() {
                    failures += 1;
                }

                // Make sure we didn't forget to mark as tested after implementing integration test.
                if let Some(test_output) = test_output {
                    if check_integration_test_crate::test_exists(version, &method.name, test_output)
                        .is_ok()
                    {
                        eprintln!("found integration test for untested method: {}", method.name);
                        failures += 1;
                    }
                }
            }
            Status::Omitted | Status::Todo => {
                let out = Method::from_name(version, &method.name)
                    .expect("guaranteed by methods_and_status()");

                if versioned::type_exists(version, &method.name)?
                    && !versioned::requires_type(version, &method.name)?
                {
                    eprintln!(
                        "return type found but method is omitted or TODO: {}",
                        output_method(out)
                    );
                    failures += 1;
                }

                if model::type_exists(version, &method.name)?
                    && !model::requires_type(version, &method.name)?
                {
                    eprintln!(
                        "model type found but method is omitted or TODO: {}",
                        output_method(out)
                    );
                    failures += 1;
                }
            }
        }
    }

    if failures > 0 {
        return Err(anyhow::anyhow!("status verification failed ({} issue(s))", failures));
    }
    Ok(())
}

fn check_types_exist_if_required(version: Version, method_name: &str) -> Result<()> {
    let out = Method::from_name(version, method_name).expect("guaranteed by methods_and_status()");

    if versioned::requires_type(version, method_name)?
        && !versioned::type_exists(version, method_name)?
    {
        eprintln!("missing return type: {}", output_method(out));
        return Err(anyhow::anyhow!("missing return type"));
    }
    if model::requires_type(version, method_name)? && !model::type_exists(version, method_name)? {
        eprintln!("missing model type: {}", output_method(out));
        return Err(anyhow::anyhow!("missing model type"));
    }
    if model::type_exists(version, method_name)? && !model::requires_type(version, method_name)? {
        eprintln!("found model type when none expected: {}", output_method(out));
        return Err(anyhow::anyhow!("unexpected model type"));
    }
    Ok(())
}

fn output_method(method: &Method) -> String {
    if let Some(Return::Type(s)) = method.ret {
        format!("{} {}", method.name, s)
    } else {
        method.name.to_string()
    }
}

/// Verifies that the 'Returns' table entry ("version" vs "version + model") matches the
/// method definition in verfiy.
fn verify_returns_method(version: Version) -> Result<()> {
    use verify::versioned::{returns_map, ReturnsDoc};

    let map = returns_map(version)?;
    let mut failures = 0;

    for (name, entry) in map.into_iter() {
        let Some(method) = Method::from_name(version, &name) else { continue };

        match entry {
            ReturnsDoc::Version => {
                if method.requires_model {
                    eprintln!(
                        "'Returns' says 'version' but method is marked as requiring a model: \n{}\n",
                        output_method(method)
                    );
                    failures += 1;
                }
                if matches!(method.ret, Some(Return::NoType)) {
                    eprintln!(
                        "'Returns' says 'version' but method has no custom return type: \n{}\n",
                        output_method(method)
                    );
                    failures += 1;
                }
            }
            ReturnsDoc::VersionPlusModel => {
                if !method.requires_model {
                    eprintln!(
                        "'Returns' says 'version + model' but method is marked as not requiring a model: \n{}\n",
                        output_method(method)
                    );
                    failures += 1;
                }
                if matches!(method.ret, Some(Return::NoType)) {
                    eprintln!(
                        "'Returns' says 'version + model' but method has no custom return type: \n{}\n",
                        output_method(method)
                    );
                    failures += 1;
                }
            }
            ReturnsDoc::Other(ref s) if s.starts_with("returns ") => {
                if matches!(method.ret, Some(Return::Type(_))) {
                    eprintln!(
                        "'Returns' says '{}' but method has a custom return type requiring a re-export: \n{}\n",
                        s,
                        output_method(method)
                    );
                    failures += 1;
                }
            }
            ReturnsDoc::Other(_) => {}
        }
    }

    if failures > 0 {
        return Err(anyhow::anyhow!("returns/model verification failed ({} issue(s))", failures));
    }

    Ok(())
}

// Use a module because a file with this name is confusing.
mod check_integration_test_crate {
    //! Things related to parsing the `integration_test` crate.

    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::PathBuf;

    use anyhow::{Context, Result};
    use regex::Regex;
    use verify::method;

    use crate::Version;

    fn all_test_functions(test_output: &str) -> Result<Vec<String>> {
        let mut functions = vec![];

        let path = PathBuf::from(test_output);
        let file = File::open(&path)
            .with_context(|| format!("Failed to open test output file {}", path.display()))?;
        let reader = io::BufReader::new(file);
        let test_re = Regex::new(r"test ([a-z_]+) ... ok")?;

        for line in reader.lines() {
            let line = line?;

            if let Some(caps) = test_re.captures(&line) {
                let function = caps.get(1).unwrap().as_str();
                functions.push(function.to_string());
            }
        }

        Ok(functions)
    }

    /// Checks that a test exists in the given test output.
    pub fn test_exists(version: Version, method_name: &str, test_output: &str) -> Result<bool> {
        let method = match method::Method::from_name(version, method_name) {
            Some(m) => m,
            None =>
                return Err(anyhow::Error::msg(format!(
                    "expected test method not found: {}",
                    method_name
                ))),
        };

        let test_name = if method.requires_model {
            format!("__{}__modelled", method.function)
        } else {
            format!("__{}", method.function)
        };
        for t in all_test_functions(test_output)? {
            if t.contains(&test_name) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
