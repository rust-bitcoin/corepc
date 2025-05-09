// The version specific client and json types.
//
// **THIS IS AVAILABLE FOR ALL VERSION NUMBER FEATURES** (eg `25_2`, `28_0` etc). This crate is
// unusual in that it expects exactly one version number feature to be selected, docs.rs is not set
// up to handle such oddity.

#![allow(unused_imports)] // Not all users need the json types.

#[cfg(feature = "28_0")]
pub use corepc_client::{client_sync::v28::{Client, AddressType, TemplateRequest, TemplateRules}, types::v28 as vtype};

#[cfg(all(feature = "27_2", not(feature = "28_0")))]
pub use corepc_client::{client_sync::v27::{Client, AddressType, TemplateRequest, TemplateRules}, types::v27 as vtype};

#[cfg(all(feature = "27_1", not(feature = "27_2")))]
pub use corepc_client::{client_sync::v27::{Client, AddressType, TemplateRequest, TemplateRules}, types::v27 as vtype};

#[cfg(all(feature = "27_0", not(feature = "27_1")))]
pub use corepc_client::{client_sync::v27::{Client, AddressType, TemplateRequest, TemplateRules}, types::v27 as vtype};

#[cfg(all(feature = "26_2", not(feature = "27_0")))]
pub use corepc_client::{client_sync::v26::{Client, AddressType, TemplateRequest, TemplateRules}, types::v26 as vtype};

#[cfg(all(feature = "26_1", not(feature = "26_2")))]
pub use corepc_client::{client_sync::v26::{Client, AddressType, TemplateRequest, TemplateRules}, types::v26 as vtype};

#[cfg(all(feature = "26_0", not(feature = "26_1")))]
pub use corepc_client::{client_sync::v26::{Client, AddressType, TemplateRequest, TemplateRules}, types::v26 as vtype};

#[cfg(all(feature = "25_2", not(feature = "26_0")))]
pub use corepc_client::{client_sync::v25::{Client, AddressType, TemplateRequest, TemplateRules}, types::v25 as vtype};

#[cfg(all(feature = "24_2", not(feature = "25_2")))]
pub use corepc_client::{client_sync::v24::{Client, AddressType, TemplateRequest, TemplateRules}, types::v24 as vtype};

#[cfg(all(feature = "23_2", not(feature = "24_2")))]
pub use corepc_client::{client_sync::v23::{Client, AddressType, TemplateRequest, TemplateRules}, types::v23 as vtype};

#[cfg(all(feature = "22_1", not(feature = "23_2")))]
pub use corepc_client::{client_sync::v22::{Client, AddressType, TemplateRequest, TemplateRules}, types::v22 as vtype};

#[cfg(all(feature = "0_21_2", not(feature = "22_1")))]
pub use corepc_client::{client_sync::v21::{Client, AddressType, TemplateRequest, TemplateRules}, types::v21 as vtype};

#[cfg(all(feature = "0_20_2", not(feature = "0_21_2")))]
pub use corepc_client::{client_sync::v20::{Client, AddressType, TemplateRequest, TemplateRules}, types::v20 as vtype};

#[cfg(all(feature = "0_19_1", not(feature = "0_20_2")))]
pub use corepc_client::{client_sync::v19::{Client, AddressType, TemplateRequest, TemplateRules}, types::v19 as vtype};

#[cfg(all(feature = "0_18_1", not(feature = "0_19_1")))]
pub use corepc_client::{client_sync::v18::{Client, AddressType, TemplateRequest, TemplateRules}, types::v18 as vtype};

#[cfg(all(feature = "0_17_2", not(feature = "0_18_1")))]
pub use corepc_client::{client_sync::v17::{Client, AddressType, TemplateRequest, TemplateRules}, types::v17 as vtype};

/// This is meaningless but we need it otherwise we can't get far enough into
/// the build process to trigger the `compile_error!` in `./versions.rs`.
#[cfg(all(
    not(feature = "28_0"),
    not(feature = "27_2"),
    not(feature = "27_1"),
    not(feature = "27_0"),
    not(feature = "26_2"),
    not(feature = "26_1"),
    not(feature = "26_0"),
    not(feature = "25_2"),
    not(feature = "24_2"),
    not(feature = "23_2"),
    not(feature = "22_1"),
    not(feature = "0_21_2"),
    not(feature = "0_20_2"),
    not(feature = "0_19_1"),
    not(feature = "0_18_1"),
    not(feature = "0_17_2"),
))]
pub use corepc_client::{client_sync::v28::{Client, AddressType, TemplateRequest, TemplateRules}, types::v28 as vtype};
