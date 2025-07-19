// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Wallet ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

#[cfg(feature = "TODO")]
use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, FeeRate, PrivateKey, PublicKey};
use integration_test::{Node, NodeExt as _, Wallet};
use node::{mtype, AddressType, ImportMultiRequest, ImportMultiScriptPubKey, ImportMultiTimestamp};
use node::vtype::*;             // All the version specific types.
use std::fs;

#[test]
fn wallet__abandon_transaction() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let mining_addr = node.client.new_address().expect("newaddress");
    let json = node.client.generate_to_address(101, &mining_addr).expect("generatetoaddress");
    let block_hashes = json.into_model();

    let block_hash = block_hashes.expect("blockhash").0[0];

    let dest_addr = node.client.new_address().expect("newaddress");
    let amount = bitcoin::Amount::from_sat(1_000_000);

    let txid = node
        .client
        .send_to_address_rbf(&dest_addr, amount)
        .expect("sendtoaddressrbf")
        .txid()
        .expect("txid");

    node.client.invalidate_block(block_hash).expect("invalidateblock");

    let _: () = node.client.abandon_transaction(txid).expect("abandontransaction");
}

#[test]
fn wallet__abort_rescan() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let json: AbortRescan = node.client.abort_rescan().expect("abortrescan");
    assert!(!json.0); // No rescan running, abort should return false
}

#[test]
#[cfg(feature = "TODO")]
fn wallet__add_multisig_address__modelled() {
    let nrequired = 1; // 1-of-2 multisig.

    let add1: Address<NetworkChecked> =
        "32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf".parse::<Address<_>>().unwrap().assume_checked();
    let add2: Address<NetworkChecked> =
        "132F25rTsvBdp9JzLLBHP5mvGY66i1xdiM".parse::<Address<_>>().unwrap().assume_checked();

    let node = Node::with_wallet(Wallet::Default, &[]);
    let json: AddMultisigAddress = node
        .client
        .add_multisig_address_with_addresses(nrequired, vec![add1, add2])
        .expect("addmultisigaddress");
    let model: Result<mtype::AddMultisigAddress, AddMultisigAddressError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__backup_wallet() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let file_path = integration_test::random_tmp_file();

    let _: () = node.client.backup_wallet(&file_path).expect("backupwallet");
    assert!(file_path.exists(), "Backup file should exist at destination");
    assert!(file_path.is_file(), "Backup destination should be a file");

    fs::remove_file(&file_path).expect("removefile");
}

#[test]
fn wallet__bump_fee__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to create new address");
    let _ = node.client.generate_to_address(101, &address).expect("generatetoaddress");

    let txid = node
        .client
        .send_to_address_rbf(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json: BumpFee = node.client.bump_fee(txid).expect("bumpfee");
    let model: Result<mtype::BumpFee, BumpFeeError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__create_wallet__modelled() {
    // Implicitly tests `createwallet` because we create the default wallet.
    let _ = Node::with_wallet(Wallet::Default, &[]);
}

#[test]
fn wallet__dump_priv_key__modelled() {
    // As of Core v23 the default wallet is an native descriptor wallet which does not
    // support dumping private keys. Legacy wallets are supported upto v25 it seems.
    #[cfg(all(feature = "v25_and_below", not(feature = "v22_and_below")))]
    {
        let node = Node::with_wallet(Wallet::None, &[]);

        node.client.create_legacy_wallet("legacy_wallet").expect("legacy create_wallet");
        let address = node.client.get_new_address(Some("label"), Some(AddressType::Legacy)).expect("legacy get_new_address");
        let address = address.into_model().unwrap().0.assume_checked();

        let json: DumpPrivKey = node.client.dump_priv_key(&address).expect("dumpprivkey");
        let model: Result<mtype::DumpPrivKey, _> = json.into_model();
        model.unwrap();
    }

    #[cfg(feature = "v22_and_below")]
    {
        let node = Node::with_wallet(Wallet::Default, &[]);
        let address = node.client.new_address().expect("failed to get new address");

        let json: DumpPrivKey = node.client.dump_priv_key(&address).expect("dumpprivkey");
        let model: Result<mtype::DumpPrivKey, _> = json.into_model();
        model.unwrap();
    }
}

#[test]
fn wallet__dump_wallet() {
    // As of Core v23 the default wallet is an native descriptor wallet which does not
    // support dumping private keys. Legacy wallets are supported upto v25 it seems.
    #[cfg(all(feature = "v25_and_below", not(feature = "v22_and_below")))]
    {
        let node = Node::with_wallet(Wallet::None, &[]);

        node.client.create_legacy_wallet("legacy_wallet").expect("legacy create_wallet");
        let out = integration_test::random_tmp_file();

        let _: DumpWallet = node.client.dump_wallet(&out).expect("dumpwallet");
    }

    #[cfg(feature = "v22_and_below")]
    {
        let node = Node::with_wallet(Wallet::Default, &[]);
        let out = integration_test::random_tmp_file();

        let _: DumpWallet = node.client.dump_wallet(&out).expect("dumpwallet");
    }
}

#[test]
fn wallet__encrypt_wallet() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let _: EncryptWallet = node.client.encrypt_wallet("test-passphrase").expect("encryptwallet");
}

#[test]
fn wallet__get_addresses_by_label__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let label = "some-label";
    let addr = node.client.new_address_with_label(label).expect("failed to get new address");

    let json: GetAddressesByLabel = node.client.get_addresses_by_label(label).expect("getaddressesbylabel");
    let model: Result<mtype::GetAddressesByLabel, _> = json.into_model();
    let map = model.unwrap();

    // sanity checks.
    assert!(!map.0.is_empty());
    assert!(map.0.contains_key(&addr));
}

#[test]
#[cfg(feature = "TODO")]        // FIXME: The types are broken.
// TODO: Consider testing a few different address types.
fn wallet__get_address_info__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to create new address");

    let json: GetAddressInfo = node.client.get_address_info(&address).expect("getaddressinfo");
    let model: Result<mtype::GetAddressInfo, GetAddressInfoError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__get_balance__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let json: GetBalance = node.client.get_balance().expect("getbalance");
    let model: Result<mtype::GetBalance, _> = json.into_model();
    model.unwrap();

    // Check non-zero balance just for giggles.
    node.fund_wallet();
    let json = node.client.get_balance().expect("getbalance");
    json.into_model().unwrap();
}

#[test]
#[cfg(not(feature = "v18_and_below"))]
fn wallet__get_balances() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: GetBalances = node.client.get_balances().expect("getbalances");
    let model: Result<mtype::GetBalances, _> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__get_new_address__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    // Implicitly tests `getnewaddress`.
    let _ = node.client.new_address().unwrap();

    // Exhaustively test address types with helper.
    let _ = node.client.new_address_with_type(AddressType::Legacy).unwrap();
    let _ = node.client.new_address_with_type(AddressType::P2shSegwit).unwrap();
    let _ = node.client.new_address_with_type(AddressType::Bech32).unwrap();
}

#[test]
fn wallet__get_raw_change_address__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let json: GetRawChangeAddress = node.client.get_raw_change_address().expect("getrawchangeaddress");
    let model: Result<mtype::GetRawChangeAddress, _> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__get_received_by_address__modelled() {
    let amount = Amount::from_sat(10_000);

    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let _txid =
        node.client.send_to_address(&address, amount).expect("sendtoaddress").txid().unwrap();
    node.mine_a_block();

    let json: GetReceivedByAddress = node.client.get_received_by_address(&address).expect("getreceivedbyaddress");
    let model: Result<mtype::GetReceivedByAddress, _> = json.into_model();
    let model = model.unwrap();

    assert_eq!(model.0, amount);
}

#[cfg(not(feature = "v17"))]
#[test]
fn wallet__get_received_by_label__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let label = "test-label";

    // Send some coins to the label
    let amount = Amount::from_sat(10_000);
    let address = node.client.new_address_with_label(label).unwrap().assume_checked();
    let _ = node.client.send_to_address(&address, amount).unwrap();
    node.mine_a_block();

    let json: GetReceivedByLabel = node.client.get_received_by_label(label).expect("getreceivedbylabel");
    let model: Result<mtype::GetReceivedByLabel, _> = json.into_model();
    assert_eq!(model.unwrap().0, amount);
}

#[test]
fn wallet__get_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let txid = node
        .client
        .send_to_address(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json: GetTransaction = node.client.get_transaction(txid).expect("gettransaction");
    let model: Result<mtype::GetTransaction, GetTransactionError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__import_address() {
    let node = match () {
        #[cfg(feature = "v22_and_below")]
        () => Node::with_wallet(Wallet::Default, &[]),
        #[cfg(not(feature = "v22_and_below"))]
        () => {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            node.client.create_legacy_wallet("wallet_name").expect("createlegacywallet");
            node
        }
    };

    let privkey =
        PrivateKey::from_wif("cVt4o7BGAig1UXywgGSmARhxMdzP5qvQsxKkSsc1XEkw3tDTQFpy").unwrap();

    // Derive the address from the private key
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let pubkey = privkey.public_key(&secp);
    let addr = bitcoin::Address::p2pkh(pubkey, privkey.network);

    let _: () = node.client.import_address(&addr).expect("importaddress");
}

#[test]
#[cfg(not(feature = "v20_and_below"))]
fn wallet__import_descriptors() {
    use node::{serde_json, ImportDescriptorsRequest};

    let node = Node::with_wallet(Wallet::None, &[]);
    let wallet_name = "desc_wallet";
    node.client.create_wallet_with_descriptors(wallet_name).expect("create descriptor wallet");

    let address = node.client.new_address().expect("failed to get new address");
    let descriptor = format!("addr({})", address);

    let request = ImportDescriptorsRequest {
        descriptor,
        timestamp: serde_json::Value::String("now".to_string()),
    };

    let _: ImportDescriptors = node.client.import_descriptors(&[request]).expect("importdescriptors");
}

#[test]
fn wallet__import_pruned_funds() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let (_, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let raw_tx = node.client.get_raw_transaction(txid).expect("getrawtransaction");
    let tx_out_proof = node.client.get_tx_out_proof(&[txid]).expect("gettxoutproof");

    let _: () = node.client.import_pruned_funds(&raw_tx.0, &tx_out_proof).expect("importprunedfunds");
}

#[test]
fn wallet__import_wallet() {
    let node = match () {
        #[cfg(feature = "v22_and_below")]
        () => Node::with_wallet(Wallet::Default, &[]),
        #[cfg(not(feature = "v22_and_below"))]
        () => {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            node.client.create_legacy_wallet("wallet_name").expect("createlegacywallet");
            node
        }
    };

    node.client.new_address().expect("newaddress");
    let dump_file_path = integration_test::random_tmp_file();

    node.client.dump_wallet(&dump_file_path).expect("dumpwallet");
    assert!(dump_file_path.exists());

    let _: () = node.client.import_wallet(&dump_file_path).expect("importwallet");
}

#[test]
fn wallet__keypool_refill() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let _: () = node.client.key_pool_refill().expect("keypoolrefill");
}

#[cfg(not(feature = "v17"))]
#[test]
fn wallet__list_received_by_label__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let label = "test-label";

    // Send some coins to the label
    let amount = Amount::from_sat(10_000);
    let address = node.client.new_address_with_label(label).unwrap().assume_checked();
    let _ = node.client.send_to_address(&address, amount).unwrap();
    node.mine_a_block();

    let json: ListReceivedByLabel = node.client.list_received_by_label().expect("listreceivedbylabel");
    let model: Result<mtype::ListReceivedByLabel, ListReceivedByLabelError> = json.into_model();
    let model = model.unwrap();
    assert!(model.0.iter().any(|item| item.label == label));
}

#[test]
fn wallet__import_multi() {
    let node = match () {
        #[cfg(feature = "v22_and_below")]
        () => Node::with_wallet(Wallet::Default, &[]),
        #[cfg(not(feature = "v22_and_below"))]
        () => {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            node.client.create_legacy_wallet("wallet_name").expect("createlegacywallet");
            node
        }
    };

    let dummy_script_hex = "76a914aabbccddeeff00112233445566778899aabbccdd88ac";
    let addr = node.client.new_address().expect("newaddress");
    let dummy_desc = "pkh(02c6047f9441ed7d6d3045406e95c07cd85a2a0e5c1e507a7a7e3d2f0d6c3d8ef8)#tp9h0863";

    // Uses scriptPubKey (valid): success - true, without warnings nor error.
    // NOTE: On v17, use a wallet-generated address (not raw script)
    // to ensure import succeeds, since the wallet already knows the key.
    let req1 = ImportMultiRequest {
        descriptor: None,
        script_pubkey: Some(ImportMultiScriptPubKey::Script(dummy_script_hex.to_string())),
        timestamp: ImportMultiTimestamp::Now,
    };

    // Uses an address (valid): success - false, with JSON-RPC error.
    let req2 = ImportMultiRequest {
        descriptor: None,
        script_pubkey: Some(ImportMultiScriptPubKey::Address {
            address: addr.to_string(),
        }),
        timestamp: ImportMultiTimestamp::Now,
    };

    // Uses descriptor (valid): success - true
    // on v18 onwards, it will return a watch-only warning.
    // NOTE: Works only for v18 onwards, as v17 doesn't support descriptors.
    let req3 = ImportMultiRequest {
        descriptor: Some(dummy_desc.to_string()),
        script_pubkey: None,
        timestamp: ImportMultiTimestamp::Time(1_700_000_000),
    };

    let json: ImportMulti = node.client.import_multi(&[req1, req2, req3]).expect("importmulti");

    #[cfg(not(feature = "v17"))]
    {
        // result of req1: should succeed, no error, no warning.
        // just any random script doesn't work with v17.
        assert!(json.0[0].success);
        assert!(json.0[0].error.is_none());

        // result of req3: should succeed, with warning for v18 onwards
        assert!(json.0[2].success);
        assert!(json.0[2].error.is_none());
        assert!(json.0[2].warnings.is_some());
    }

    // result of req2: should fail with error (wallet already contains privkey for address/script)
    assert!(!json.0[1].success);
    assert!(json.0[1].error.is_some());
}

#[test]
fn wallet__import_privkey() {
    let node = match () {
        #[cfg(feature = "v22_and_below")]
        () => Node::with_wallet(Wallet::Default, &[]),
        #[cfg(not(feature = "v22_and_below"))]
        () => {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            node.client.create_legacy_wallet("wallet_name").expect("createlegacywallet");
            node
        }
    };

    let privkey =
        PrivateKey::from_wif("cVt4o7BGAig1UXywgGSmARhxMdzP5qvQsxKkSsc1XEkw3tDTQFpy").unwrap();

    let _: () = node.client.import_privkey(&privkey).expect("importprivkey");
}

#[test]
fn wallet__import_pubkey() {
    let node = match () {
        #[cfg(feature = "v22_and_below")]
        () => Node::with_wallet(Wallet::Default, &[]),
        #[cfg(not(feature = "v22_and_below"))]
        () => {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            node.client.create_legacy_wallet("wallet_name").expect("createlegacywallet");
            node
        }
    };

    let pubkey = "02ff12471208c14bd580709cb2358d98975247d8765f92bc25eab3b2763ed605f8"
        .parse::<PublicKey>()
        .unwrap();

    let _: () = node.client.import_pubkey(&pubkey).expect("importpubkey");
}

#[test]
#[cfg(not(feature = "v21_and_below"))]
fn wallet__list_descriptors() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let wallet_name = "desc_wallet";

    #[cfg(feature = "v22_and_below")]
    node.client.create_wallet_with_descriptors(wallet_name).expect("create descriptor wallet");

    // v23 onwards uses descriptor wallets by default.
    #[cfg(not(feature = "v22_and_below"))]
    node.client.create_wallet(wallet_name).expect("create wallet");

    let json: ListDescriptors = node.client.list_descriptors().expect("listdescriptors");

    let has_descriptor = json.descriptors.iter().any(|desc_info| desc_info.descriptor.starts_with("wpkh(") || desc_info.descriptor.starts_with("pkh("));
    assert!(has_descriptor, "No standard descriptors found in listdescriptors result");
}

#[test]
fn wallet__list_unspent__modelled() {
    let node = match () {
        #[cfg(feature = "v17")]
        () => Node::with_wallet(Wallet::Default, &["-deprecatedrpc=accounts"]),
        #[cfg(not(feature = "v17"))]
        () => Node::with_wallet(Wallet::Default, &[]),
    };

    node.fund_wallet();

    let json: ListUnspent = node.client.list_unspent().expect("listunspent");
    let model: Result<mtype::ListUnspent, ListUnspentItemError> = json.into_model();
    model.unwrap();
}

#[cfg(not(feature = "v17"))]
#[test]
fn wallet__list_wallet_dir() {
    let wallet_name = "test-wallet";
    let node = Node::with_wallet(Wallet::None, &[]);
    node.client.create_wallet(wallet_name).expect("failed to create wallet");

    let wallet_dir = node.client.list_wallet_dir().expect("listwalletdir");
    let wallet_names: Vec<_> = wallet_dir.wallets.iter().map(|w| &w.name).collect();

    assert!(wallet_names.iter().any(|w| *w == wallet_name));
}

#[test]
fn wallet__load_wallet__modelled() {
    create_load_unload_wallet();
}

#[test]
fn wallet__lock_unspent() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: LockUnspent = node.client.lock_unspent().expect("lockunspent");
    assert!(json.0);

    let json: LockUnspent = node.client.unlock_unspent().expect("unlockunspent");
    assert!(json.0);
}

#[cfg(not(feature = "v20_and_below"))]
#[test]
fn wallet__psbt_bump_fee__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to create new address");
    let _ = node.client.generate_to_address(101, &address).expect("generatetoaddress");

    let txid = node
        .client
        .send_to_address_rbf(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json: PsbtBumpFee = node.client.psbt_bump_fee(&txid).expect("psbtbumpfee");
    let model: Result<mtype::PsbtBumpFee, PsbtBumpFeeError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__remove_pruned_funds() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let (_, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let raw_tx = node.client.get_raw_transaction(txid).expect("getrawtransaction");
    let tx_out_proof = node.client.get_tx_out_proof(&[txid]).expect("gettxoutproof");

    let _: () = node.client.import_pruned_funds(&raw_tx.0, &tx_out_proof).expect("importprunedfunds");

    let _: () = node.client.remove_pruned_funds(txid).expect("removeprunedfunds");
}

// This is tested in raw_transactions.rs `create_sign_send()`.
#[test]
fn wallet__sign_raw_transaction_with_wallet__modelled() {}

#[test]
fn wallet__unload_wallet() {
    create_load_unload_wallet();
}

#[cfg(not(feature = "v20_and_below"))]
#[test]
fn wallet__send__modelled() {
    use std::collections::BTreeMap;

    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let mut outputs = BTreeMap::new();
    outputs.insert(address.to_string(), 0.001);

    let json: Send = node.client.send(&outputs).expect("send");
    let model: Result<mtype::Send, SendError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__send_to_address__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let json: SendToAddress =
        node.client.send_to_address(&address, Amount::from_sat(10_000)).expect("sendtddress");
    let model: Result<mtype::SendToAddress, _> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__set_tx_fee() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let fee_rate = FeeRate::from_sat_per_vb(2).expect("2 sat/vb is valid");

    let json: SetTxFee = node.client.set_tx_fee(fee_rate).expect("settxfee");
    assert!(json.0);
}

#[cfg(not(feature = "v18_and_below"))]
#[test]
fn wallet__set_wallet_flag() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let json: SetWalletFlag = node.client.set_wallet_flag("avoid_reuse").expect("setwalletflag");
    assert_eq!(json.flag_name, "avoid_reuse");
    assert!(json.flag_state);
}

#[test]
fn wallet__set_hd_seed() {
    let node = match () {
        #[cfg(feature = "v22_and_below")]
        () => Node::with_wallet(Wallet::Default, &[]),
        #[cfg(not(feature = "v22_and_below"))]
        () => {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            node.client.create_legacy_wallet("wallet_name").expect("createlegacywallet");
            node
        }
    };

    node.fund_wallet();

    let _: () = node.client.set_hd_seed().expect("sethdseed");
}

#[test]
fn wallet__sign_message__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let address = node.client.new_address_with_type(AddressType::Legacy).unwrap();
    let message = "integration test message";

    // Sign the message with the address key
    let json: SignMessage = node
        .client
        .sign_message(&address, message)
        .expect("signmessage");
    let res: Result<mtype::SignMessage, _> = json.into_model();
    let _ = res.expect("SignMessage into model");
}

#[test]
fn wallet__wallet_lock() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    node.client.create_wallet("wallet_name").expect("createwallet");
    node.client.encrypt_wallet("passphrase").expect("encryptwallet");

    let _: () = node.client.wallet_lock().expect("walletlock");
}

#[test]
fn wallet__wallet_passphrase() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    node.client.create_wallet("wallet_name").expect("createwallet");
    node.client.encrypt_wallet("passphrase").expect("encryptwallet");

    let timeout = 60u64;
    let _: () = node.client.wallet_passphrase("passphrase", timeout).expect("walletpassphrase");
}

#[test]
fn wallet__wallet_passphrase_change() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    node.client.create_wallet("wallet name").expect("createwallet");
    node.client.encrypt_wallet("old passphrase").expect("encryptwallet");

    let _: () = node.client.wallet_passphrase_change("old passphrase", "new passphrase")
        .expect("walletpassphrasechange");
}

fn create_load_unload_wallet() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
    node.client.create_wallet(&wallet).expect("failed to create wallet");

    // Upto version 20 Core returns null for `unloadwallet`.
    #[cfg(feature = "v20_and_below")]
    let _: () = node.client.unload_wallet(&wallet).expect("unloadwallet");

    // From version 21 Core returns warnings for `unloadwallet`.
    #[cfg(not(feature = "v20_and_below"))]
    {
        let json: UnloadWallet = node.client.unload_wallet(&wallet).expect("unloadwallet");
        let _: mtype::UnloadWallet = json.into_model();
    }

    let _: LoadWallet = node.client.load_wallet(&wallet).expect("loadwallet");
}

#[cfg(not(feature = "v20_and_below"))]
#[test]
fn wallet__upgrade_wallet() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let _: UpgradeWallet = node.client.upgrade_wallet().expect("upgradewallet");
}
