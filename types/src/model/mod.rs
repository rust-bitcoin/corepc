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
mod mining;
mod network;
mod raw_transactions;
mod util;
mod wallet;
mod zmq;

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
        ScanBlocksStart, Softfork, SoftforkType, SpendActivity, VerifyTxOutProof,
    },
    generating::{Generate, GenerateBlock, GenerateToAddress, GenerateToDescriptor},
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
        SendRawTransaction, SignFail, SignRawTransaction, SubmitPackage, SubmitPackageTxResult,
        SubmitPackageTxResultFees, TestMempoolAccept, UtxoUpdatePsbt,
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
        SimulateRawTransaction, TransactionCategory, TransactionItem, UnloadWallet,
        WalletCreateFundedPsbt, WalletDisplayAddress, WalletProcessPsbt,
    },
};
