// SPDX-License-Identifier: CC0-1.0

//! JSON RPC methods provided by Bitcoin Core v25.

use super::Method;

/// Data for the JSON RPC methods provided by Bitcoin Core v25.
pub const METHODS: &[Method] = &[
    Method::new_modelled("getbestblockhash", "GetBestBlockHash", "get_best_block_hash"),
    Method::new_modelled("getblock", "GetBlockVerbosityZero", "get_block"), // We only check one of the types.
    Method::new_modelled("getblockchaininfo", "GetBlockchainInfo", "get_blockchain_info"),
    Method::new_modelled("getblockcount", "GetBlockCount", "get_block_count"),
    Method::new_modelled("getblockfilter", "GetBlockFilter", "get_block_filter"),
    Method::new_modelled("getblockfrompeer", "GetBlockFromPeer", "get_block_from_peer"),
    Method::new_modelled("getblockhash", "GetBlockHash", "get_block_hash"),
    Method::new_modelled("getblockheader", "GetBlockHeader", "get_block_header"),
    Method::new_modelled("getblockstats", "GetBlockStats", "get_block_stats"),
    Method::new_modelled("getchaintips", "GetChainTips", "get_chain_tips"),
    Method::new_modelled("getchaintxstats", "GetChainTxStats", "get_chain_tx_stats"),
    Method::new_modelled("getdeploymentinfo", "GetDeploymentInfo", "get_deployment_info"),
    Method::new_modelled("getdifficulty", "GetDifficulty", "get_difficulty"),
    Method::new_modelled("getmempoolancestors", "GetMempoolAncestors", "get_mempool_ancestors"),
    Method::new_modelled(
        "getmempooldescendants",
        "GetMempoolDescendants",
        "get_mempool_descendants",
    ),
    Method::new_modelled("getmempoolentry", "GetMempoolEntry", "get_mempool_entry"),
    Method::new_modelled("getmempoolinfo", "GetMempoolInfo", "get_mempool_info"),
    Method::new_modelled("getrawmempool", "GetRawMempool", "get_raw_mempool"),
    Method::new_modelled("gettxout", "GetTxOut", "get_tx_out"),
    Method::new_string("gettxoutproof", "get_tx_out_proof"),
    Method::new_modelled("gettxoutsetinfo", "GetTxOutSetInfo", "get_tx_out_set_info"),
    Method::new_modelled("gettxspendingprevout", "GetTxSpendingPrevout", "get_tx_spending_prevout"),
    Method::new_nothing("preciousblock", "precious_block"),
    Method::new_numeric("pruneblockchain", "prune_blockchain"),
    Method::new_nothing("savemempool", "save_mempool"),
    Method::new_modelled("scanblocks", "ScanBlocks", "scan_blocks"),
    Method::new_modelled("scantxoutset", "ScanTxOutSet", "scan_tx_out_set"),
    Method::new_bool("verifychain", "verify_chain"),
    Method::new_modelled("verifytxoutproof", "VerifyTxOutProof", "verify_tx_out_proof"),
    Method::new_no_model("getrpcinfo", "GetRpcInfo", "get_rpc_info"),
    Method::new_no_model("getmemoryinfo", "GetMemoryInfoStats", "get_memory_info"),
    Method::new_string("help", "help"),
    Method::new_no_model("logging", "Logging", "logging"),
    Method::new_nothing("stop", "stop"),
    Method::new_numeric("uptime", "uptime"),
    Method::new_modelled("getblocktemplate", "GetBlockTemplate", "get_block_template"),
    Method::new_no_model("getmininginfo", "GetMiningInfo", "get_mining_info"),
    Method::new_nothing("getnetworkhashps", "get_network_hashes_per_second"),
    Method::new_bool("prioritisetransaction", "prioritise_transaction"),
    Method::new_nothing("submitblock", "submit_block"),
    Method::new_nothing("submitheader", "submit_header"),
    Method::new_nothing("addnode", "add_node"),
    Method::new_nothing("clearbanned", "clear_banned"),
    Method::new_nothing("disconnectnode", "disconnect_node"),
    Method::new_no_model("getaddednodeinfo", "GetAddedNodeInfo", "get_added_node_info"),
    Method::new_numeric("getconnectioncount", "get_connection_count"),
    Method::new_no_model("getnettotals", "GetNetTotals", "get_net_totals"),
    Method::new_modelled("getnetworkinfo", "GetNetworkInfo", "get_network_info"),
    Method::new_no_model("getnodeaddresses", "GetNodeAddresses", "get_node_addresses"),
    Method::new_no_model("getpeerinfo", "GetPeerInfo", "get_peer_info"),
    Method::new_string("listbanned", "list_banned"), // v17 docs seem wrong, says no return.
    Method::new_nothing("ping", "ping"),
    Method::new_nothing("setban", "set_ban"),
    Method::new_nothing("setnetworkactive", "set_network_active"),
    Method::new_modelled("analyzepsbt", "AnalyzePsbt", "analyze_psbt"),
    Method::new_nothing("combinepsbt", "combine_psbt"),
    Method::new_nothing("combinerawtransaction", "combine_raw_transaction"),
    Method::new_nothing("converttopsbt", "convert_to_psbt"),
    Method::new_nothing("createpsbt", "create_psbt"),
    Method::new_nothing("createrawtransaction", "create_raw_transaction"),
    Method::new_nothing("decodepsbt", "decode_psbt"),
    Method::new_nothing("decoderawtransaction", "decode_raw_transaction"),
    Method::new_nothing("decodescript", "decode_script"),
    Method::new_nothing("finalizepsbt", "finalize_psbt"),
    Method::new_nothing("fundrawtransaction", "fund_raw_transaciton"),
    Method::new_nothing("getrawtransaction", "get_raw_transaction"),
    Method::new_modelled("joinpsbts", "JoinPsbts", "join_psbts"),
    Method::new_modelled("sendrawtransaction", "SendRawTransaction", "send_raw_transaction"),
    Method::new_nothing("signrawtransactionwithkey", "sign_raw_transaction_with_key"),
    Method::new_nothing("testmempoolaccept", "test_mempool_accept"),
    Method::new_modelled("utxoupdatepsbt", "UtxoUpdatePsbt", "utxo_update_psbt"),
    Method::new_modelled("enumeratesigners", "EnumerateSigners", "enumerate_signers"),
    Method::new_modelled("createmultisig", "CreateMultisig", "create_multisig"),
    Method::new_modelled("deriveaddresses", "DeriveAddresses", "derive_addresses"),
    Method::new_nothing("estimatesmartfee", "estimate_smart_fee"),
    Method::new_no_model("getdescriptorinfo", "GetDescriptorInfo", "get_descriptor_info"),
    Method::new_no_model("getindexinfo", "GetIndexInfo", "get_index_info"),
    Method::new_string("signmessagewithprivkey", "sign_message_with_priv_key"),
    Method::new_modelled("validateaddress", "ValidateAddress", "validate_address"),
    Method::new_bool("verifymessage", "verify_message"),
    Method::new_nothing("abandontransaction", "abandon_transaction"),
    Method::new_nothing("abortrescan", "abort_rescan"),
    Method::new_modelled("addmultisigaddress", "AddMultisigAddress", "add_multisig_address"),
    Method::new_nothing("backupwallet", "backup_wallet"),
    Method::new_modelled("bumpfee", "BumpFee", "bump_fee"),
    Method::new_modelled("createwallet", "CreateWallet", "create_wallet"),
    Method::new_modelled("dumpprivkey", "DumpPrivKey", "dump_priv_key"),
    Method::new_modelled("dumpwallet", "DumpWallet", "dump_wallet"),
    Method::new_nothing("encryptwallet", "encrypt_wallet"),
    Method::new_modelled("getaddressesbylabel", "GetAddressesByLabel", "get_addresses_by_label"),
    Method::new_modelled("getaddressinfo", "GetAddressInfo", "get_address_info"),
    Method::new_modelled("getbalance", "GetBalance", "get_balance"),
    Method::new_modelled("getbalances", "GetBalances", "get_balances"),
    Method::new_modelled("getnewaddress", "GetNewAddress", "get_new_address"),
    Method::new_modelled("getrawchangeaddress", "GetRawChangeAddress", "get_raw_change_address"),
    Method::new_modelled("getreceivedbyaddress", "GetReceivedByAddress", "get_received_by_address"),
    Method::new_modelled("getreceivedbylabel", "GetReceivedByLabel", "get_received_by_label"),
    Method::new_modelled("gettransaction", "GetTransaction", "get_transaction"),
    Method::new_modelled(
        "getunconfirmedbalance",
        "GetUnconfirmedBalance",
        "get_unconfirmed_balance",
    ),
    Method::new_modelled("getwalletinfo", "GetWalletInfo", "get_wallet_info"),
    Method::new_nothing("importaddress", "import_address"),
    Method::new_no_model("importdescriptors", "ImportDescriptors", "import_descriptors"),
    Method::new_nothing("importmulti", "import_multi"),
    Method::new_nothing("importprivkey", "import_priv_key"),
    Method::new_nothing("importprunedfunds", "import_pruned_funds"),
    Method::new_nothing("importpubkey", "import_pubkey"),
    Method::new_nothing("importwallet", "import_walet"),
    Method::new_nothing("keypoolrefill", "keypool_refill"),
    Method::new_modelled("listaddressgroupings", "ListAddressGroupings", "list_address_groupings"),
    Method::new_modelled("listdescriptors", "ListDescriptors", "list_descriptors"),
    Method::new_modelled("listlabels", "ListLabels", "list_labels"),
    Method::new_modelled("listlockunspent", "ListLockUnspent", "list_lock_unspent"),
    Method::new_modelled("migratewallet", "MigrateWallet", "migrate_wallet"),
    Method::new_modelled("newkeypool", "NewKeyPool", "new_key_pool"),
    Method::new_modelled("psbtbumpfee", "PsbtBumpFee", "psbt_bump_fee"),
    Method::new_modelled(
        "listreceivedbyaddress",
        "ListReceivedByAddress",
        "list_received_by_address",
    ),
    Method::new_modelled("listreceivedbylabel", "ListReceivedByLabel", "list_received_by_label"),
    Method::new_modelled("listsinceblock", "ListSinceBlock", "list_since_block"),
    Method::new_modelled("listtransactions", "ListTransactions", "list_transactions"),
    Method::new_modelled("listunspent", "ListUnspent", "list_unspent"),
    Method::new_no_model("listwalletdir", "ListWalletDir", "list_wallet_dir"),
    Method::new_modelled("listwallets", "ListWallets", "list_wallets"),
    Method::new_modelled("loadwallet", "LoadWallet", "load_wallet"),
    Method::new_bool("lockunspent", "lock_unspent"),
    Method::new_nothing("removeprunedfunds", "remove_pruned_funds"),
    Method::new_modelled("rescanblockchain", "RescanBlockchain", "rescan_blockchain"),
    Method::new_modelled("restorewallet", "RestoreWallet", "restore_wallet"),
    Method::new_modelled("send", "Send", "send"),
    Method::new_modelled("sendall", "SendAll", "send_all"),
    Method::new_modelled("sendmany", "SendMany", "send_many"),
    Method::new_modelled("sendtoaddress", "SendToAddress", "send_to_address"),
    Method::new_nothing("sethdseed", "set_hd_seed"),
    Method::new_nothing("setlabel", "set_label"),
    Method::new_bool("settxfee", "set_tx_fee"),
    Method::new_modelled("setwalletflag", "SetWalletFlag", "set_wallet_flag"),
    Method::new_modelled("signmessage", "SignMessage", "sign_message"),
    Method::new_modelled(
        "signrawtransactionwithwallet",
        "SignRawTransactionWithWallet",
        "sign_raw_transaction_with_wallet",
    ),
    Method::new_modelled("simulaterawtransaction", "SimulateRawTransaction", "simulate_raw_transaction"),
    Method::new_nothing("unloadwallet", "unload_wallet"),
    Method::new_no_model("upgradewallet", "UpgradeWalled", "upgrade_wallet"),
    Method::new_modelled(
        "walletcreatefundedpsbt",
        "WalletCreateFundedPsbt",
        "wallet_create_funded_psbt",
    ),
    Method::new_modelled("walletdisplayaddress", "WalletDisplayAddress", "wallet_display_address"),
    Method::new_nothing("walletlock", "wallet_lock"),
    Method::new_nothing("walletpassphrase", "wallet_passphrase"),
    Method::new_nothing("walletpassphrasechange", "wallet_passphrase_change"),
    Method::new_modelled("walletprocesspsbt", "WalletProcessPsbt", "wallet_process_psbt"),
    Method::new_no_model("getzmqnotifications", "GetZmqNotifications", "get_zmq_notifications"),
];
