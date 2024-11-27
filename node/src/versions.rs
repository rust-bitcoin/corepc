#[cfg(feature = "28_0")]
pub const VERSION: &str = "28.0";

#[cfg(all(feature = "27_2", not(feature = "28_0")))]
pub const VERSION: &str = "27.1";

#[cfg(all(feature = "27_1", not(feature = "27_2")))]
pub const VERSION: &str = "27.1";

#[cfg(all(feature = "27_0", not(feature = "27_1")))]
pub const VERSION: &str = "27.0";

#[cfg(all(feature = "26_2", not(feature = "27_0")))]
pub const VERSION: &str = "26.2";

#[cfg(all(feature = "26_1", not(feature = "26_2")))]
pub const VERSION: &str = "26.1";

#[cfg(all(feature = "26_0", not(feature = "26_1")))]
pub const VERSION: &str = "26.0";

#[cfg(all(feature = "25_2", not(feature = "26_0")))]
pub const VERSION: &str = "25.2";

#[cfg(all(feature = "25_1", not(feature = "25_2")))]
pub const VERSION: &str = "25.1";

#[cfg(all(feature = "25_0", not(feature = "25_1")))]
pub const VERSION: &str = "25.0";

#[cfg(all(feature = "24_2", not(feature = "25_0")))]
pub const VERSION: &str = "24.2";

#[cfg(all(feature = "24_1", not(feature = "24_2")))]
pub const VERSION: &str = "24.1";

#[cfg(all(feature = "24_0_1", not(feature = "24_1")))]
pub const VERSION: &str = "24.0.1";

#[cfg(all(feature = "23_2", not(feature = "24_0_1")))]
pub const VERSION: &str = "23.2";

#[cfg(all(feature = "23_1", not(feature = "23_2")))]
pub const VERSION: &str = "23.1";

#[cfg(all(feature = "23_0", not(feature = "23_1")))]
pub const VERSION: &str = "23.0";

#[cfg(all(feature = "22_1", not(feature = "23_0")))]
pub const VERSION: &str = "22.1";

#[cfg(all(feature = "22_0", not(feature = "22_1")))]
pub const VERSION: &str = "22.0";

#[cfg(all(feature = "0_21_2", not(feature = "22_0")))]
pub const VERSION: &str = "0.21.2";

#[cfg(all(feature = "0_20_2", not(feature = "0_21_2")))]
pub const VERSION: &str = "0.20.2";

#[cfg(all(feature = "0_19_1", not(feature = "0_20_2")))]
pub const VERSION: &str = "0.19.1";

#[cfg(all(feature = "0_18_1", not(feature = "0_19_1")))]
pub const VERSION: &str = "0.18.1";

#[cfg(all(feature = "0_17_1", not(feature = "0_18_1")))]
pub const VERSION: &str = "0.17.1";

// To make --no-default-features work we have to enable some feature, use most recent version same as for default.
#[cfg(all(not(feature = "28_0"), not(feature = "27_1"), not(feature = "27_0"), not(feature = "26_2"), not(feature = "26_1"), not(feature = "26_0"), not(feature = "25_2"), not(feature = "25_1"), not(feature = "25_0"), not(feature = "24_2"),not(feature = "24_1"), not(feature = "24_0_1"), not(feature = "23_2"), not(feature = "23_1"), not(feature = "23_0"), not(feature = "22_1"), not(feature = "22_0"), not(feature = "0_21_2"), not(feature = "0_20_2"), not(feature = "0_19_1"), not(feature = "0_18_1"), not(feature = "0_17_1")))]
#[allow(dead_code)]         // for --no-default-features
pub const VERSION: &str = "28.0";
