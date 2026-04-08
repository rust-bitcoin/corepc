// An explicit version of Bitcoin Core must be selected by enabling some feature.
// We check this here instead of in `lib.rs` because this file is included in `build.rs`.
#[cfg(all(
    not(feature = "30_2"),
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
    not(feature = "0_17_2")
))]
compile_error!("enable a feature in order to select the version of Bitcoin Core to use");

#[cfg(feature = "30_2")]
#[allow(dead_code)] // Triggers in --all-features builds.
pub const VERSION: &str = "30.2";

#[cfg(all(feature = "30_0", not(feature = "30_2")))]
pub const VERSION: &str = "30.0";

#[cfg(all(feature = "29_0", not(feature = "30_0")))]
pub const VERSION: &str = "29.0";

#[cfg(all(feature = "28_2", not(feature = "29_0")))]
pub const VERSION: &str = "28.2";

#[cfg(all(feature = "28_1", not(feature = "28_2")))]
pub const VERSION: &str = "28.1";

#[cfg(all(feature = "28_0", not(feature = "28_1")))]
pub const VERSION: &str = "28.0";

#[cfg(all(feature = "27_2", not(feature = "28_0")))]
pub const VERSION: &str = "27.2";

#[cfg(all(feature = "27_1", not(feature = "27_2")))]
pub const VERSION: &str = "27.1";

#[cfg(all(feature = "27_0", not(feature = "27_1")))]
pub const VERSION: &str = "27.0";

#[cfg(all(feature = "26_2", not(feature = "27_0")))]
pub const VERSION: &str = "26.2";

#[cfg(all(feature = "25_2", not(feature = "26_2")))]
pub const VERSION: &str = "25.2";

#[cfg(all(feature = "24_2", not(feature = "25_2")))]
pub const VERSION: &str = "24.2";

#[cfg(all(feature = "23_2", not(feature = "24_2")))]
pub const VERSION: &str = "23.2";

#[cfg(all(feature = "22_1", not(feature = "23_2")))]
pub const VERSION: &str = "22.1";

#[cfg(all(feature = "0_21_2", not(feature = "22_1")))]
pub const VERSION: &str = "0.21.2";

#[cfg(all(feature = "0_20_2", not(feature = "0_21_2")))]
pub const VERSION: &str = "0.20.2";

#[cfg(all(feature = "0_19_1", not(feature = "0_20_2")))]
pub const VERSION: &str = "0.19.1";

#[cfg(all(feature = "0_18_1", not(feature = "0_19_1")))]
pub const VERSION: &str = "0.18.1";

#[cfg(all(feature = "0_17_2", not(feature = "0_18_1")))]
pub const VERSION: &str = "0.17.2";

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
    not(feature = "0_17_2")
))]
pub const VERSION: &str = "never-used";
