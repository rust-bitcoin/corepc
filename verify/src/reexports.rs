// SPDX-License-Identifier: CC0-1.0

//! Checks the re-exports in `corepc-types` are complete.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use syn::{Fields, GenericArgument, Item, PathArguments, Type, UseTree, Visibility};
use walkdir::WalkDir;

use crate::Version;

type VersionedDeps = HashMap<String, BTreeMap<String, BTreeSet<String>>>;
type ParsedTypeFiles = (Vec<(String, PathBuf)>, HashSet<String>);

/// The original version/type behind a public re-export.
#[derive(Clone, Debug)]
struct ExportInfo {
    source_version: String,
    source_ident: String,
    exported_ident: String,
}

/// A flattened path entry gathered from a `use` tree.
#[derive(Debug)]
struct UseEntry {
    path: Vec<String>,
    rename: Option<String>,
}

/// Checks that every type is re-exported for the requested version.
pub fn check_type_reexports(version: Version) -> Result<()> {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src_dir = crate_dir.join("../types/src");
    let all_versions = collect_version_dirs(&src_dir)?;
    let (files, known_names) = collect_type_files_and_names(&src_dir, &all_versions)?;
    let definitions = collect_type_definitions(&files, &known_names)?;
    let version_name = version.to_string();
    let export_map = collect_exports(&src_dir, &version_name)?;

    let mut missing = Vec::new();

    // Checks every type defined in this version is publicly re-exported.
    let version_defs = match definitions.get(&version_name) {
        Some(defs) => defs,
        None => {
            let msg = format!("no definitions found for version {}", version_name);
            return Err(anyhow::anyhow!(msg));
        }
    };

    for type_name in version_defs.keys() {
        let exported = export_map
            .values()
            .any(|info| info.source_version == version_name && type_name == &info.source_ident);
        if !exported {
            missing
                .push(format!("{} defines {} but does not re-export it", version_name, type_name));
        }
    }

    // Checks all auxiliary types are re-exported.
    for (exported_name, export) in &export_map {
        if let Some(deps) =
            definitions.get(&export.source_version).and_then(|map| map.get(&export.source_ident))
        {
            for dep in deps {
                if !export_map.contains_key(dep) {
                    missing.push(format!(
                        "{} re-exports {} from {} but does not re-export auxiliary type {}",
                        version_name, exported_name, export.source_version, dep
                    ));
                }
            }
        }
    }

    if missing.is_empty() {
        return Ok(());
    }
    let msg = format!("Missing re-exports:\n{}", missing.join("\n"));
    Err(anyhow!(msg))
}

/// Returns all the types version root directories `types/src/vXX`.
fn collect_version_dirs(src_dir: &Path) -> Result<Vec<String>> {
    let mut versions = Vec::new();
    for entry in fs::read_dir(src_dir)
        .with_context(|| format!("reading version directory listing in {}", src_dir.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if is_version_dir_name(&name) {
            versions.push(name.into_owned());
        }
    }
    versions.sort();
    Ok(versions)
}

/// Parses all versioned source files and records every public struct/enum name.
fn collect_type_files_and_names(src_dir: &Path, versions: &[String]) -> Result<ParsedTypeFiles> {
    let mut files = Vec::new();
    let mut names = HashSet::new();

    for version in versions {
        let dir = src_dir.join(version);
        for entry in WalkDir::new(&dir).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            if entry.path().extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let content = fs::read_to_string(entry.path())
                .with_context(|| format!("reading source file {}", entry.path().display()))?;
            let syntax = syn::parse_file(&content)
                .with_context(|| format!("parsing source file {}", entry.path().display()))?;
            for item in &syntax.items {
                match item {
                    Item::Struct(item_struct) if is_public(&item_struct.vis) => {
                        names.insert(item_struct.ident.to_string());
                    }
                    Item::Enum(item_enum) if is_public(&item_enum.vis) => {
                        names.insert(item_enum.ident.to_string());
                    }
                    _ => {}
                }
            }
            files.push((version.clone(), entry.into_path()));
        }
    }

    Ok((files, names))
}

/// Builds a per-version dependency map for every public type.
fn collect_type_definitions(
    files: &[(String, PathBuf)],
    known_names: &HashSet<String>,
) -> Result<VersionedDeps> {
    let mut defs: VersionedDeps = HashMap::new();

    for (version, path) in files {
        let content = fs::read_to_string(path)
            .with_context(|| format!("reading source file {}", path.display()))?;
        let syntax = syn::parse_file(&content)
            .with_context(|| format!("parsing source file {}", path.display()))?;
        for item in syntax.items {
            match item {
                Item::Struct(item_struct) if is_public(&item_struct.vis) => {
                    let deps = collect_deps_from_fields(&item_struct.fields, known_names);
                    defs.entry(version.clone())
                        .or_default()
                        .insert(item_struct.ident.to_string(), deps);
                }
                Item::Enum(item_enum) if is_public(&item_enum.vis) => {
                    let mut deps = BTreeSet::new();
                    for variant in item_enum.variants {
                        deps.extend(collect_deps_from_fields(&variant.fields, known_names));
                    }
                    defs.entry(version.clone())
                        .or_default()
                        .insert(item_enum.ident.to_string(), deps);
                }
                _ => {}
            }
        }
    }

    Ok(defs)
}

/// Reads `mod.rs` for the chosen version and lists its public re-exports.
fn collect_exports(src_dir: &Path, version: &str) -> Result<HashMap<String, ExportInfo>> {
    let mod_path = src_dir.join(version).join("mod.rs");
    let content =
        fs::read_to_string(&mod_path).with_context(|| format!("reading {}", mod_path.display()))?;
    let syntax =
        syn::parse_file(&content).with_context(|| format!("parsing {}", mod_path.display()))?;
    let mut exports = HashMap::new();

    for item in syntax.items {
        if let Item::Use(item_use) = item {
            if !is_public(&item_use.vis) {
                continue;
            }
            let mut entries = Vec::new();
            flatten_use_tree(Vec::new(), &item_use.tree, &mut entries);
            for entry in entries {
                if let Some(info) = interpret_flat_use(version, &entry) {
                    exports.insert(info.exported_ident.clone(), info);
                }
            }
        }
    }

    Ok(exports)
}

/// Extracts referenced auxiliary types from the provided field set.
fn collect_deps_from_fields(fields: &Fields, known_names: &HashSet<String>) -> BTreeSet<String> {
    let mut deps = BTreeSet::new();
    match fields {
        Fields::Named(named) =>
            for field in &named.named {
                collect_type_dependencies(&field.ty, known_names, &mut deps);
            },
        Fields::Unnamed(unnamed) =>
            for field in &unnamed.unnamed {
                collect_type_dependencies(&field.ty, known_names, &mut deps);
            },
        Fields::Unit => {}
    }
    deps
}

/// Recursively walks a type expression to find referenced auxiliary types.
fn collect_type_dependencies(
    ty: &Type,
    known_names: &HashSet<String>,
    deps: &mut BTreeSet<String>,
) {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let ident = segment.ident.to_string();
                if known_names.contains(&ident) {
                    deps.insert(ident);
                }
            }
            for segment in &type_path.path.segments {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let GenericArgument::Type(inner) = arg {
                            collect_type_dependencies(inner, known_names, deps);
                        }
                    }
                }
            }
        }
        Type::Reference(reference) => collect_type_dependencies(&reference.elem, known_names, deps),
        Type::Paren(paren) => collect_type_dependencies(&paren.elem, known_names, deps),
        Type::Group(group) => collect_type_dependencies(&group.elem, known_names, deps),
        Type::Tuple(tuple) =>
            for elem in &tuple.elems {
                collect_type_dependencies(elem, known_names, deps);
            },
        Type::Array(array) => collect_type_dependencies(&array.elem, known_names, deps),
        Type::Slice(slice) => collect_type_dependencies(&slice.elem, known_names, deps),
        Type::Ptr(ptr) => collect_type_dependencies(&ptr.elem, known_names, deps),
        _ => {}
    }
}

/// Converts a nested `use` tree into simple path entries.
fn flatten_use_tree(prefix: Vec<String>, tree: &UseTree, acc: &mut Vec<UseEntry>) {
    match tree {
        UseTree::Name(name) => {
            let mut path = prefix;
            path.push(name.ident.to_string());
            acc.push(UseEntry { path, rename: None });
        }
        UseTree::Rename(rename) => {
            let mut path = prefix;
            path.push(rename.ident.to_string());
            acc.push(UseEntry { path, rename: Some(rename.rename.to_string()) });
        }
        UseTree::Path(path) => {
            let mut new_prefix = prefix;
            new_prefix.push(path.ident.to_string());
            flatten_use_tree(new_prefix, &path.tree, acc);
        }
        UseTree::Group(group) =>
            for item in &group.items {
                flatten_use_tree(prefix.clone(), item, acc);
            },
        UseTree::Glob(_) => {}
    }
}

/// Takes a `use` statement entry and figures out which version/module defines the type.
fn interpret_flat_use(target_version: &str, entry: &UseEntry) -> Option<ExportInfo> {
    if entry.path.is_empty() {
        return None;
    }
    let source_ident = entry.path.last()?.clone();
    let exported_ident = entry.rename.clone().unwrap_or_else(|| source_ident.clone());

    match entry.path.first()?.as_str() {
        "self" => Some(ExportInfo {
            source_version: target_version.to_string(),
            source_ident,
            exported_ident,
        }),
        "crate" => {
            if entry.path.len() < 3 {
                return None;
            }
            let source_module = &entry.path[1];
            if is_version_dir_name(source_module) {
                Some(ExportInfo {
                    source_version: source_module.clone(),
                    source_ident,
                    exported_ident,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Returns true if the type Visibility is public.
fn is_public(vis: &Visibility) -> bool { matches!(vis, Visibility::Public(_)) }

/// Checks whether the directory is the root for the version, i.e. the name fits the `vXX` pattern.
fn is_version_dir_name(name: &str) -> bool {
    name.starts_with('v') && name.chars().skip(1).all(|c| c.is_ascii_digit())
}
