// SPDX-License-Identifier: CC0-1.0

//! Tool to help verify that what we claim in the rustdocs is true.
//!
//! Currently verifies:
//!
//! - That the correct set of methods is documented.
//! - That an expected return type is provided if the method is supported.
//! - That there is a `model` type if required.
//! - That the method has an integration test.

use std::process;

use anyhow::Result;
use clap::{arg, Command};
use verify::method::{Method, Return};
use verify::versioned::{self, Status};
use verify::{method, model, ssot, Version};

// TODO: Enable running from any directory, currently errors if run from `src/`.
// TODO: Add a --quiet option.

const VERSIONS: [Version; 5] = [Version::V17, Version::V18, Version::V19, Version::V20, Version::V21];

fn main() -> Result<()> {
    let cmd = Command::new("verify")
        .args([
            arg!([version] "Verify specific version of Core (use \"all\" for all versions)").required(true),
            arg!(-t --tests <TEST_OUTPUT> "Optionally check claimed status of tests").required(false),
        ]);

    let matches = cmd.clone().get_matches();
    let version = matches.get_one::<String>("version").unwrap();
    let test_output = matches.get_one::<String>("tests");

    if version == "all" {
        verify_all_versions(test_output)?;
    } else if let Ok(v) = version.parse::<Version>() {
        verify_version(v, test_output)?;
    } else {
        eprint!("Unrecognised version: {} (supported versions:", version);
        for version in VERSIONS {
            eprint!(" {}", version);
        }
        eprintln!(")");
        process::exit(1);
    }
    Ok(())
}

fn verify_all_versions(test_output: Option<&String>) -> Result<()> {
    for version in VERSIONS {
        println!("Verifying for Bitcoin Core version {} ...\n", version);
        verify_version(version, test_output)?;
    }
    Ok(())
}

fn verify_version(version: Version, test_output: Option<&String>) -> Result<()> {
    let s = format!("{}::METHOD data", version);
    let msg = format!("Checking that the {} list is correct", s);
    check(&msg);
    let correct = verify_correct_methods(version, method::all_methods(version), &s)?;
    close(correct);
    if !correct {
        process::exit(1);
    }

    let s = "rustdoc version specific rustdocs";
    let msg = format!("Checking that the {} list is correct", s);
    check(&msg);
    let correct = verify_correct_methods(version, versioned::all_methods(version)?, s)?;
    close(correct);
    if !correct {
        process::exit(1);
    }

    let msg = "Checking that the status claimed in the version specific rustdocs is correct";
    check(msg);
    verify_status(version, test_output)?;
    close(correct);

    Ok(())
}

fn check(msg: &str) { println!("{} ... ", msg) }

fn close(correct: bool) {
    if correct {
        println!("Correct \u{2713} \n");
    }
}

/// Verifies that the correct set of methods are documented.
fn verify_correct_methods(version: Version, methods: Vec<String>, msg: &str) -> Result<bool> {
    let ssot = ssot::all_methods(version)?;
    let want = ssot.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let got = methods.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    Ok(verify::correct_methods(&got, &want, msg))
}

/// Verifies that the status we claim is correct.
fn verify_status(version: Version, test_output: Option<&String>) -> Result<()> {
    let methods = versioned::methods_and_status(version)?;
    for method in methods {
        let out =
            Method::from_name(version, &method.name).expect("guaranteed by methods_and_status()");
        match method.status {
            Status::Done => {
                if !versioned::return_type_exists(version, &method.name)? {
                    eprintln!("missing return type: {}", output_method(out));
                }
                if !model::type_exists(version, &method.name)? {
                    eprintln!("missing model type: {}", output_method(out));
                }
                if let Some(test_output) = test_output {
                    if !check_integration_test_crate::test_exists(version, &method.name, test_output)? {
                        eprintln!("missing integration test: {}", method.name);
                    }
                }
            }
            Status::Untested => {
                if !versioned::return_type_exists(version, &method.name)? {
                    eprintln!("missing return type: {}", output_method(out));
                }
                if !model::type_exists(version, &method.name)? {
                    eprintln!("missing model type: {}", output_method(out));
                }
                // Make sure we didn't forget to mark as tested after implementing integration test.
                if let Some(test_output) = test_output {
                    if check_integration_test_crate::test_exists(version, &method.name, test_output)? {
                        eprintln!("found integration test for untested method: {}", method.name);
                    }
                }
            }
            Status::Omitted | Status::Todo => {
                if versioned::return_type_exists(version, &method.name)? {
                    eprintln!("return type found but method is omitted or TODO: {}", output_method(out));
                }
            }
        }
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
        let file = File::open(&path).with_context(|| {
            format!("Failed to open test output file {}", path.display())
        })?;
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

    /// Checks that a type exists in `model` module.
    pub fn test_exists(version: Version, method_name: &str, test_output: &str) -> Result<bool> {
        let method = match method::Method::from_name(version, method_name) {
            Some(m) => m,
            None =>
                return Err(anyhow::Error::msg(format!(
                    "expected test method not found: {}",
                    method_name
                ))),
        };

        let tests = all_test_functions(test_output)?;
        if !tests.contains(&method.function.to_string()) {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}
