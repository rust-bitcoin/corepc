// SPDX-License-Identifier: CC0-1.0

//! Provides a data structure that describes each JSON RPC method.

pub mod v17;
pub mod v18;
pub mod v19;
pub mod v20;
pub mod v21;
pub mod v22;

use crate::Version;

/// Returns a list of all the method names.
pub fn all_methods(version: Version) -> Vec<String> {
    use Version::*;

    let list = match version {
        V17 => v17::METHODS,
        V18 => v18::METHODS,
        V19 => v19::METHODS,
        V20 => v20::METHODS,
        V21 => v21::METHODS,
        V22 => v22::METHODS,
    };

    list.iter().map(|m| m.name.to_string()).collect()
}

/// Describes a single JSON RPC method.
// TODO: This name is overloaded (`versioned::Method`).
#[derive(Debug)]
pub struct Method {
    /// The method name.
    pub name: &'static str,
    /// The type it is expected to return (method name in snake case).
    ///
    /// `None` if the method is either intentionally omitted or not yet done.
    pub ret: Option<Return>,
    /// `true` if this method requires a type to exist in `model`.
    ///
    /// This is true if the return type includes types that can be
    /// more strongly typed using `rust-bitcoin`.
    pub requires_model: bool,
    /// The function name (snake case) for this method.
    pub function: &'static str,
}

impl Method {
    /// Returns a `Method` type if one exists in the `METHODS` list for `name`.
    pub fn from_name(version: Version, name: &str) -> Option<&'static Method> {
        use Version::*;
        let list = match version {
            V17 => v17::METHODS,
            V18 => v18::METHODS,
            V19 => v19::METHODS,
            V20 => v20::METHODS,
            V21 => v21::METHODS,
            V22 => v22::METHODS,
        };

        list.iter().find(|&method| method.name == name)
    }

    /// Represents a `Method` that requires a custom type as well as a type in `model`.
    const fn new_modeled(name: &'static str, ty: &'static str, function: &'static str) -> Method {
        Method { name, ret: Some(Return::Type(ty)), requires_model: true, function }
    }

    /// Represents a method that requires a custom type as but no type in `model`.
    ///
    /// Implies this method does not return data that can be strongly type using `rust-bitcoin`.
    const fn new_no_model(name: &'static str, ty: &'static str, function: &'static str) -> Method {
        Method { name, ret: Some(Return::Type(ty)), requires_model: false, function }
    }

    const fn new_nothing(name: &'static str, function: &'static str) -> Method {
        Method { name, ret: Some(Return::Nothing), function, requires_model: false }
    }

    const fn new_numeric(name: &'static str, function: &'static str) -> Method {
        Method { name, ret: Some(Return::Numeric), function, requires_model: false }
    }

    const fn new_bool(name: &'static str, function: &'static str) -> Method {
        Method { name, ret: Some(Return::Bool), function, requires_model: false }
    }

    const fn new_string(name: &'static str, function: &'static str) -> Method {
        Method { name, ret: Some(Return::String), function, requires_model: false }
    }

    const fn new_none(name: &'static str, function: &'static str) -> Method {
        Method { name, ret: None, function, requires_model: false }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Return {
    /// Method returns a type (that should exist).
    Type(&'static str),
    /// Method does not return anything.
    Nothing,
    /// Method returns a numeric type.
    Numeric,
    /// Method returns a boolean.
    Bool,
    /// Method returns a string.
    String,
}
