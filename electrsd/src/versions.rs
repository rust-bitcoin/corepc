// An explicit version of Bitcoin Core must be selected by enabling some feature.
// We check this here instead of in `lib.rs` because this file is included in `build.rs`.
#[cfg(not(any(
    feature = "electrs_0_8_10",
    feature = "electrs_0_9_1",
    feature = "electrs_0_9_11",
    feature = "electrs_0_10_6",
    feature = "esplora_a33e97e1",
)))]
compile_error!("enable a feature in order to select the version of electrs to use");

#[cfg(target_os = "macos")]
const OS: &str = "macos";

#[cfg(target_os = "linux")]
const OS: &str = "linux";

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
const OS: &str = "undefined";

// This version is used when --all-feature tests are run.
#[cfg(feature = "electrs_0_10_6")]
const VERSION: &str = "v0.10.6";

#[cfg(all(feature = "electrs_0_9_11", not(feature = "all_features")))]
const VERSION: &str = "v0.9.11";

#[cfg(all(feature = "electrs_0_9_1", not(feature = "all_features")))]
const VERSION: &str = "v0.9.1";

#[cfg(all(feature = "electrs_0_8_10", not(feature = "all_features")))]
const VERSION: &str = "v0.8.10";

#[cfg(all(feature = "esplora_a33e97e1", not(feature = "all_features")))]
const VERSION: &str = "esplora_a33e97e1a1fc63fa9c20a116bb92579bbf43b254";

/// This is meaningless but we need it otherwise we can't get far enough into
/// the build process to trigger the `compile_error!` in `./versions.rs`.
#[cfg(not(any(
    feature = "electrs_0_8_10",
    feature = "electrs_0_9_1",
    feature = "electrs_0_9_11",
    feature = "electrs_0_10_6",
    feature = "esplora_a33e97e1",
)))]
const VERSION: &str = "never-used";

pub fn electrs_name() -> String { format!("electrs_{}_{}", OS, VERSION) }
