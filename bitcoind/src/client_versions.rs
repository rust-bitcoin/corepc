// The version specific client and json types.
//
// **THIS IS AVAILABLE FOR ALL VERSION NUMBER FEATURES** (eg `25_2`, `28_0` etc). This crate is
// unusual in that it expects exactly one version number feature to be selected, docs.rs is not set
// up to handle such oddity.

#![allow(unused_imports)] // Not all users need the json types.

#[cfg(feature = "30_0")]
pub use corepc_client::{client_sync::v30::*, types::v30 as vtype};

#[cfg(all(feature = "29_0", not(feature = "30_0")))]
pub use corepc_client::{client_sync::v29::*, types::v29 as vtype};

#[cfg(all(feature = "28_2", not(feature = "29_0")))]
pub use corepc_client::{client_sync::v28::*, types::v28 as vtype};

#[cfg(all(feature = "28_1", not(feature = "28_2")))]
pub use corepc_client::{client_sync::v28::*, types::v28 as vtype};

#[cfg(all(feature = "28_0", not(feature = "28_1")))]
pub use corepc_client::{client_sync::v28::*, types::v28 as vtype};

#[cfg(all(feature = "27_2", not(feature = "28_0")))]
pub use corepc_client::{client_sync::v27::*, types::v27 as vtype};

#[cfg(all(feature = "27_1", not(feature = "27_2")))]
pub use corepc_client::{client_sync::v27::*, types::v27 as vtype};

#[cfg(all(feature = "27_0", not(feature = "27_1")))]
pub use corepc_client::{client_sync::v27::*, types::v27 as vtype};

#[cfg(all(feature = "26_2", not(feature = "27_0")))]
pub use corepc_client::{client_sync::v26::*, types::v26 as vtype};

#[cfg(all(feature = "25_2", not(feature = "26_2")))]
pub use corepc_client::{client_sync::v25::*, types::v25 as vtype};

#[cfg(all(feature = "24_2", not(feature = "25_2")))]
pub use corepc_client::{client_sync::v24::*, types::v24 as vtype};

#[cfg(all(feature = "23_2", not(feature = "24_2")))]
pub use corepc_client::{client_sync::v23::*, types::v23 as vtype};

#[cfg(all(feature = "22_1", not(feature = "23_2")))]
pub use corepc_client::{client_sync::v22::*, types::v22 as vtype};

#[cfg(all(feature = "0_21_2", not(feature = "22_1")))]
pub use corepc_client::{client_sync::v21::*, types::v21 as vtype};

#[cfg(all(feature = "0_20_2", not(feature = "0_21_2")))]
pub use corepc_client::{client_sync::v20::*, types::v20 as vtype};

#[cfg(all(feature = "0_19_1", not(feature = "0_20_2")))]
pub use corepc_client::{client_sync::v19::*, types::v19 as vtype};

#[cfg(all(feature = "0_18_1", not(feature = "0_19_1")))]
pub use corepc_client::{client_sync::v18::*, types::v18 as vtype};

#[cfg(all(feature = "0_17_2", not(feature = "0_18_1")))]
pub use corepc_client::{client_sync::v17::*, types::v17 as vtype};

/// This is meaningless but we need it otherwise we can't get far enough into
/// the build process to trigger the `compile_error!` in `./versions.rs`.
#[cfg(all(
    not(feature = "30_0"),
    not(feature = "29_0"),
    not(feature = "28_2"),
    not(feature = "28_1"),
    not(feature = "28_0"),
    not(feature = "27_2"),
    not(feature = "27_1"),
    not(feature = "27_0"),
    not(feature = "26_2"),
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
pub use corepc_client::{client_sync::v28::*, types::v28 as vtype};
