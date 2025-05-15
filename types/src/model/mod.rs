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
        ActivityEntry, Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, ChainTips,
        ChainTipsStatus, GetBestBlockHash, GetBlockCount, GetBlockFilter, GetBlockHash,
        GetBlockHeader, GetBlockHeaderVerbose, GetBlockStats, GetBlockVerboseOne,
        GetBlockVerboseZero, GetBlockchainInfo, GetChainTips, GetChainTxStats,
        GetDescriptorActivity, GetDifficulty, GetMempoolAncestors, GetMempoolAncestorsVerbose,
        GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, GetMempoolInfo,
        GetRawMempool, GetRawMempoolVerbose, GetTxOut, GetTxOutSetInfo, MempoolEntry,
        MempoolEntryFees, ReceiveActivity, ScriptPubkey, Softfork, SoftforkType, SpendActivity,
        VerifyTxOutProof,
    },
    generating::{Generate, GenerateToAddress},
    mining::{
        BlockTemplateTransaction, GetBlockTemplate, GetPrioritisedTransactions,
        PrioritisedTransaction,
    },
    network::{GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoNetwork},
    raw_transactions::{
        AnalyzePsbt, AnalyzePsbtInput, AnalyzePsbtInputMissing, CombinePsbt, CombineRawTransaction,
        ConvertToPsbt, CreatePsbt, CreateRawTransaction, DecodePsbt, DecodeRawTransaction,
        DecodeScript, DescriptorProcessPsbt, FinalizePsbt, FundRawTransaction, GetRawTransaction,
        GetRawTransactionVerbose, JoinPsbts, MempoolAcceptance, SendRawTransaction, SignFail,
        SignRawTransaction, SubmitPackage, SubmitPackageTxResult, SubmitPackageTxResultFees,
        TestMempoolAccept, UtxoUpdatePsbt,
    },
    wallet::{
        AddMultisigAddress, AddressInformation, AddressLabel, AddressPurpose, Bip125Replaceable,
        BumpFee, CreateWallet, DumpPrivKey, GetAddressInfo, GetAddressInfoEmbedded,
        GetAddressesByLabel, GetBalance, GetBalances, GetBalancesMine, GetBalancesWatchOnly,
        GetNewAddress, GetRawChangeAddress, GetReceivedByAddress, GetTransaction,
        GetTransactionDetail, GetUnconfirmedBalance, GetWalletInfo, ListAddressGroupings,
        ListAddressGroupingsItem, ListLabels, ListLockUnspent, ListLockUnspentItem,
        ListReceivedByAddress, ListReceivedByAddressItem, ListSinceBlock,
        ListSinceBlockTransaction, ListTransactions, ListTransactionsItem, ListUnspent,
        ListUnspentItem, ListWallets, LoadWallet, RescanBlockchain, ScriptType, SendMany,
        SendToAddress, SignMessage, TransactionCategory, UnloadWallet, WalletCreateFundedPsbt,
        WalletProcessPsbt,
    },
};
