// SPDX-License-Identifier: CC0-1.0
//! Compile-time assertions for type re-exports and auxiliary types.

#![allow(dead_code)]

use node::mtype::*;
#[allow(unused_imports)]
use std::collections::BTreeMap;

#[macro_export] macro_rules! assert_field_is_exact_type {
    ($struct:ty, $field:tt, $expected:ty) => {
        const _: fn() = || {
            fn assert_type(val: $struct) {
                let _: $expected = val.$field;
            }
        };
    };
}

#[test]
#[cfg(feature = "v17")]
fn test_aux_export_v17() {
    // blockchain
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v18_and_below", not(feature = "v17")))]
fn test_aux_export_v18() {
    // blockchain
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v19_and_below", not(feature = "v18_and_below")))]
fn test_aux_export_v19() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v20_and_below", not(feature = "v19_and_below")))]
fn test_aux_export_v20() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v21_and_below", not(feature = "v20_and_below")))]
fn test_aux_export_v21() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v22_and_below", not(feature = "v21_and_below")))]
fn test_aux_export_v22() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v23_and_below", not(feature = "v22_and_below")))]
fn test_aux_export_v23() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v24_and_below", not(feature = "v23_and_below")))]
fn test_aux_export_v24() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v25_and_below", not(feature = "v24_and_below")))]
fn test_aux_export_v25() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v26_and_below", not(feature = "v25_and_below")))]
fn test_aux_export_v26() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v27_and_below", not(feature = "v26_and_below")))]
fn test_aux_export_v27() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(Bip9SoftforkInfo, statistics, Option<Bip9SoftforkStatistics>);
    assert_field_is_exact_type!(Softfork, type_, SoftforkType);
    assert_field_is_exact_type!(Softfork, bip9, Option<Bip9SoftforkInfo>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v28_and_below", not(feature = "v27_and_below")))]
fn test_aux_export_v28() {
    // blockchain
    assert_field_is_exact_type!(GetBlockchainInfo, softforks, BTreeMap<String, Softfork>);
    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);
}

#[test]
#[cfg(all(feature = "v29_and_below", not(feature = "v28_and_below")))]
fn test_aux_export_v29() {
    // blockchain
    assert_field_is_exact_type!(GetDescriptorActivity, activity, Vec<ActivityEntry>);

    const _: fn() = || {
        let _: fn(SpendActivity) -> ActivityEntry = ActivityEntry::Spend;
        let _: fn(ReceiveActivity) -> ActivityEntry = ActivityEntry::Receive;
    };

    assert_field_is_exact_type!(GetChainTips, 0, Vec<ChainTips>);
    assert_field_is_exact_type!(ChainTips, status, ChainTipsStatus);
    assert_field_is_exact_type!(GetMempoolEntry, 0, MempoolEntry);
    assert_field_is_exact_type!(MempoolEntry, fees, MempoolEntryFees);

    // raw_transactions
    assert_field_is_exact_type!(AnalyzePsbt, inputs, Vec<AnalyzePsbtInput>);
    assert_field_is_exact_type!(AnalyzePsbtInput, missing, Option<AnalyzePsbtInputMissing>);
}
