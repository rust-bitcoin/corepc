// The version specific client and json types.
//
// **THIS IS AVAILABLE FOR ALL VERSION NUMBER FEATURES** (eg `25_2`, `28_0` etc). This crate is
// unusual in that it expects exactly one version number feature to be selected, docs.rs is not set
// up to handle such oddity.

#![allow(unused_imports)] // Not all users need the json types.

#[cfg(feature = "31_0")]
pub use corepc_client::{client_sync::v31::*, types::v31 as vtype};

#[cfg(all(feature = "30_2", not(feature = "31_0"), not(feature = "client-async")))]
pub use corepc_client::{client_sync::v30::*, types::v30 as vtype};

// With `client-async`, `Client` is the blocking facade over the async production client. It exposes
// the identical `v30` method surface (same names, args, curated return types), so the integration
// tests run unchanged but exercise the async transport.
//
// The `not(feature = "31_0")` guard is what makes `--all-features` resolve to the sync client: under
// `--all-features` every version feature is on (including `31_0`), so this branch is disabled and the
// `31_0` sync re-export above wins. `client-async` is meant to be paired with `30_2` alone; combining
// it with a higher version silently falls back to that version's sync client rather than erroring.
#[cfg(all(feature = "30_2", not(feature = "31_0"), feature = "client-async"))]
pub use corepc_client::{client_async::blocking::*, types::v30 as vtype};

#[cfg(all(feature = "29_0", not(feature = "30_2")))]
pub use corepc_client::{client_sync::v29::*, types::v29 as vtype};

#[cfg(all(feature = "28_2", not(feature = "29_0")))]
pub use corepc_client::{client_sync::v28::*, types::v28 as vtype};

#[cfg(all(feature = "27_2", not(feature = "28_2")))]
pub use corepc_client::{client_sync::v27::*, types::v27 as vtype};

#[cfg(all(feature = "26_2", not(feature = "27_2")))]
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
#[cfg(not(feature = "0_17_2"))] // Remember: later version features enable earlier ones.
pub use corepc_client::{client_sync::v28::*, types::v28 as vtype};

// Guards the `--all-features` => sync-client contract above. This combination of features is only
// active under `--all-features` (every version on, so `31_0` wins and disables the `client-async`
// re-export). The body never runs; the assignment only has to type-check, which it does iff
// `Client` is the sync `v31` client. Inert in every single-version build.
#[cfg(all(test, feature = "31_0", feature = "client-async"))]
#[test]
fn all_features_surfaces_the_sync_client() {
    #[allow(unreachable_code, unused_variables)]
    fn _assert() {
        let sync: corepc_client::client_sync::v31::Client = unimplemented!();
        let surfaced: crate::Client = sync;
    }
}
