// All features uses 26_2
/// The version specific client and json types.
///
/// **THIS IS AVAILABLE FOR ALL VERSION NUMBER FEATURES** (eg `25_0`, `24_2` etc). This crate is
/// unusual in that it expects exactly one version number feature to be selected, docs.rs is not set
/// up to handle such oddity.
///

#[cfg(feature = "28_0")]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v28::{Client, AddressType}, types::v28 as types};

#[cfg(all(feature = "27_2", not(feature = "28_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v27::{Client, AddressType}, types::v27 as types};

#[cfg(all(feature = "27_1", not(feature = "27_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v27::{Client, AddressType}, types::v27 as types};

#[cfg(all(feature = "27_0", not(feature = "27_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v27::{Client, AddressType}, types::v27 as types};

#[cfg(all(feature = "26_2", not(feature = "27_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v26::{Client, AddressType}, types::v26 as types};

#[cfg(all(feature = "26_1", not(feature = "26_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v26::{Client, AddressType}, types::v26 as types};

#[cfg(all(feature = "26_0", not(feature = "26_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v26::{Client, AddressType}, types::v26 as types};

#[cfg(all(feature = "25_2", not(feature = "26_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v25::{Client, AddressType}, types::v25 as types};

#[cfg(all(feature = "25_1", not(feature = "25_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v25::{Client, AddressType}, types::v25 as types};

#[cfg(all(feature = "25_0", not(feature = "25_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v25::{Client, AddressType}, types::v25 as types};

#[cfg(all(feature = "24_2", not(feature = "25_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v24::{Client, AddressType}, types::v24 as types};

#[cfg(all(feature = "24_1", not(feature = "24_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v24::{Client, AddressType}, types::v24 as types};

#[cfg(all(feature = "24_0_1", not(feature = "24_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v24::{Client, AddressType}, types::v24 as types};

#[cfg(all(feature = "23_2", not(feature = "24_0_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v23::{Client, AddressType}, types::v23 as types};

#[cfg(all(feature = "23_1", not(feature = "23_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v23::{Client, AddressType}, types::v23 as types};

#[cfg(all(feature = "23_0", not(feature = "23_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v23::{Client, AddressType}, types::v23 as types};

#[cfg(all(feature = "22_1", not(feature = "23_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v22::{Client, AddressType}, types::v22 as types};

#[cfg(all(feature = "22_0", not(feature = "22_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v22::{Client, AddressType}, types::v22 as types};

#[cfg(all(feature = "0_21_2", not(feature = "22_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v21::{Client, AddressType}, types::v21 as types};

#[cfg(all(feature = "0_20_2", not(feature = "0_21_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v20::{Client, AddressType}, types::v20 as types};

#[cfg(all(feature = "0_19_1", not(feature = "0_20_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v19::{Client, AddressType}, types::v19 as types};

#[cfg(all(feature = "0_18_1", not(feature = "0_19_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v18::{Client, AddressType}, types::v18 as types};

#[cfg(all(feature = "0_17_1", not(feature = "0_18_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v17::{Client, AddressType}, types::v17 as types};

// To make --no-default-features work we have to re-export a the types, use most recent version same as we do for all features.
#[cfg(all(not(feature = "28_0"), not(feature = "27_1"), not(feature = "27_0"), not(feature = "26_2"), not(feature = "26_1"), not(feature = "26_0"), not(feature = "25_2"), not(feature = "25_1"), not(feature = "25_0"), not(feature = "24_2"),not(feature = "24_1"), not(feature = "24_0_1"), not(feature = "23_2"), not(feature = "23_1"), not(feature = "23_0"), not(feature = "22_1"), not(feature = "22_0"), not(feature = "0_21_2"), not(feature = "0_20_2"), not(feature = "0_19_1"), not(feature = "0_18_1"), not(feature = "0_17_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use corepc_client::{client_sync::v28::{Client, AddressType}, types::v28 as types};
