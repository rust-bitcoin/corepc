// SPDX-License-Identifier: CC0-1.0

//! Generate Rust bindings for the Bitcoin Core JSON-RPC API from its OpenRPC spec.
//!
//! Usage:
//!
//! ```text
//! btc-codegen <version>      # For example, `30`, regenerates one version
//! btc-codegen all            # Regenerates every spec under specs/
//! ```
//!
//! Output is split across the two sibling crates: the response types and their model
//! conversions go to `../types/src/v{N}/generated/` and the call surface (request structs +
//! method wrappers) to `../client/src/client_async/v{N}/` (relative to the manifest dir).

use std::path::{Path, PathBuf};
use std::{env, fs, process};

use btc_codegen::generate;

/// Parse the version argument and regenerate the bindings for each selected version.
fn main() {
    let mut args = env::args().skip(1);
    let version = args.next().unwrap_or_else(|| {
        eprintln!("usage: btc-codegen <version|all>");
        process::exit(2)
    });

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let specs_dir = manifest_dir.join("specs");
    let types_root = manifest_dir.join("../types/src");
    let client_async_root = manifest_dir.join("../client/src/client_async");

    let versions = if version == "all" {
        list_versions(&specs_dir).unwrap_or_else(|e| exit_with_msg(&e))
    } else {
        vec![version]
    };

    if versions.is_empty() {
        exit_with_msg(&format!("no specs found under {}", specs_dir.display()));
    }

    for v in versions {
        let spec_path = match find_spec(&specs_dir, &v) {
            Some(p) => p,
            None => {
                eprintln!("[codegen] no spec for v{v}, skipping");
                continue;
            }
        };
        let types_dir = types_root.join(format!("v{v}/generated"));
        let client_dir = client_async_root.join(format!("v{v}"));
        if let Err(e) = generate(&spec_path, &types_dir, &client_dir, &v) {
            exit_with_msg(&format!("[codegen] v{v}: {e}"));
        }
    }
}

/// Print `msg` to stderr and exit with a failure code.
fn exit_with_msg(msg: &str) -> ! {
    eprintln!("{msg}");
    process::exit(1)
}

/// Find the spec file for one major version (`v{version}_*_openrpc.json`), the lowest match if several.
fn find_spec(specs_dir: &Path, version: &str) -> Option<PathBuf> {
    let prefix = format!("v{version}_");
    let mut matches: Vec<_> = fs::read_dir(specs_dir)
        .ok()?
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|n| n.starts_with(&prefix) && n.ends_with("_openrpc.json"))
                .unwrap_or(false)
        })
        .map(|e| e.path())
        .collect();
    matches.sort();
    matches.into_iter().next()
}

/// List the distinct major versions that have a spec file under `specs_dir`. Entries such as `v30_2_0_openrpc.json`.
fn list_versions(specs_dir: &Path) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let entries =
        fs::read_dir(specs_dir).map_err(|e| format!("read {}: {e}", specs_dir.display()))?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if !name.starts_with('v') || !name.ends_with("_openrpc.json") {
            continue;
        }
        if let Some(major) = name[1..].split('_').next() {
            if !out.contains(&major.to_owned()) {
                out.push(major.to_owned());
            }
        }
    }
    out.sort();
    Ok(out)
}
