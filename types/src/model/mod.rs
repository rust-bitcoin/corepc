// SPDX-License-Identifier: CC0-1.0

//! Models of the data returned by the JSON-RPC API of Bitcoin Core.
//!
//! The types here model the data returned by Bitcoin Core in a version nonspecific way. In other
//! words one can use a particular `bitcoind` version via the version specific module (e.g.
//! `crate::v26`) then convert the `json` types to one of the modelled types in this module using
//! `TryFrom`.

// JSON-RPC types by API section.
mod blockchain;
mod control;
mod generating;
mod hidden;
mod mining;
mod network;
mod raw_transactions;
mod util;
mod wallet;
mod zmq;

use bitcoin::address::NetworkUnchecked;
use bitcoin::{Address, ScriptBuf};
use serde::{Deserialize, Serialize};

#[doc(inline)]
pub use self::{
    blockchain::{
        ActivityEntry, Bip9Info, Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus,
        Bip9Statistics, ChainState, ChainTips, ChainTipsStatus, DeploymentInfo, DumpTxOutSet,
        GetBestBlockHash, GetBlockCount, GetBlockFilter, GetBlockHash, GetBlockHeader,
        GetBlockHeaderVerbose, GetBlockStats, GetBlockVerboseOne, GetBlockVerboseZero,
        GetBlockchainInfo, GetChainStates, GetChainTips, GetChainTxStats, GetDeploymentInfo,
        GetDescriptorActivity, GetDifficulty, GetMempoolAncestors, GetMempoolAncestorsVerbose,
        GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, GetMempoolInfo,
        GetRawMempool, GetRawMempoolVerbose, GetTxOut, GetTxOutSetInfo, GetTxSpendingPrevout,
        GetTxSpendingPrevoutItem, LoadTxOutSet, MempoolEntry, MempoolEntryFees, ReceiveActivity,
        ScanBlocksStart, Softfork, SoftforkType, SpendActivity, VerifyTxOutProof, WaitForBlock,
        WaitForBlockHeight, WaitForNewBlock,
    },
    generating::{Generate, GenerateBlock, GenerateToAddress, GenerateToDescriptor},
    hidden::{
        AddConnection, EstimateRawFee, GetOrphanTxs, GetOrphanTxsVerboseOne,
        GetOrphanTxsVerboseOneEntry, GetOrphanTxsVerboseTwo, GetOrphanTxsVerboseTwoEntry,
        RawFeeDetail, RawFeeRange,
    },
    mining::{
        BlockTemplateTransaction, GetBlockTemplate, GetMiningInfo, GetPrioritisedTransactions,
        NextBlockInfo, PrioritisedTransaction,
    },
    network::{GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoNetwork},
    raw_transactions::{
        AnalyzePsbt, AnalyzePsbtInput, AnalyzePsbtInputMissing, CombinePsbt, CombineRawTransaction,
        ConvertToPsbt, CreatePsbt, CreateRawTransaction, DecodePsbt, DecodeRawTransaction,
        DecodeScript, DescriptorProcessPsbt, FinalizePsbt, FundRawTransaction, GetRawTransaction,
        GetRawTransactionVerbose, JoinPsbts, MempoolAcceptance, MempoolAcceptanceFees,
        SendRawTransaction, SignFail, SignRawTransaction, SignRawTransactionWithKey, SubmitPackage,
        SubmitPackageTxResult, SubmitPackageTxResultFees, TestMempoolAccept, UtxoUpdatePsbt,
    },
    util::{
        CreateMultisig, DeriveAddresses, DeriveAddressesMultipath, EstimateSmartFee,
        SignMessageWithPrivKey, ValidateAddress,
    },
    wallet::{
        AddMultisigAddress, AddressInformation, AddressPurpose, Bip125Replaceable, BumpFee,
        CreateWallet, DumpPrivKey, GetAddressInfo, GetAddressInfoEmbedded, GetAddressesByLabel,
        GetBalance, GetBalances, GetBalancesMine, GetBalancesWatchOnly, GetHdKeys, GetNewAddress,
        GetRawChangeAddress, GetReceivedByAddress, GetReceivedByLabel, GetTransaction,
        GetTransactionDetail, GetUnconfirmedBalance, GetWalletInfo, GetWalletInfoScanning, HdKey,
        HdKeyDescriptor, LastProcessedBlock, ListAddressGroupings, ListAddressGroupingsItem,
        ListLockUnspent, ListLockUnspentItem, ListReceivedByAddress, ListReceivedByAddressItem,
        ListReceivedByLabel, ListReceivedByLabelItem, ListSinceBlock, ListTransactions,
        ListUnspent, ListUnspentItem, ListWallets, LoadWallet, PsbtBumpFee, RescanBlockchain,
        ScriptType, Send, SendAll, SendMany, SendManyVerbose, SendToAddress, SignMessage,
        SignRawTransactionWithWallet, SimulateRawTransaction, TransactionCategory, TransactionItem,
        UnloadWallet, WalletCreateFundedPsbt, WalletDisplayAddress, WalletProcessPsbt,
    },
};

/// Models the data returned by Core for a scriptPubKey.
///
/// This is used by methods in the blockchain section and in the raw transaction section (i.e raw
/// transaction and psbt methods).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScriptPubkey {
    /// The script_pubkey parsed from hex.
    pub script_pubkey: ScriptBuf,
    /// Number of required signatures - deprecated in Core v22.
    ///
    /// Only returned in versions prior to 22 or for version 22 onwards if
    /// config option `-deprecatedrpc=addresses` is passed.
    pub required_signatures: Option<i64>,
    /// Bitcoin address (only if a well-defined address exists).
    pub address: Option<Address<NetworkUnchecked>>,
    /// Array of bitcoin addresses - deprecated in Core v22.
    ///
    /// Only returned in versions prior to 22 or for version 22 onwards if
    /// config option `-deprecatedrpc=addresses` is passed.
    pub addresses: Option<Vec<Address<NetworkUnchecked>>>,
}
