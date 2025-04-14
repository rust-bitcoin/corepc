// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Wallet ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

#[cfg(feature = "TODO")]
use bitcoin::address::{Address, NetworkChecked};
use bitcoin::Amount;
use integration_test::{Node, NodeExt as _, Wallet};
use node::AddressType;
use node::vtype::*;             // All the version specific types.
use node::mtype;
use std::fs;

use bitcoin::{
    Address,
    Network,
    secp256k1::{SecretKey, PublicKey},
    key::{CompressedPublicKey, Secp256k1, PrivateKey}
};

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
    let model: Result<AddMultisigAddress, AddMultisigAddressError> = json.into_model();
    model.unwrap();
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
    #[cfg(any(
        feature = "v23",
        feature = "v24",
        feature = "v25",
    ))]
    {
        let node = Node::with_wallet(Wallet::None, &[]);

        node.client.create_legacy_wallet("legacy_wallet").expect("legacy create_wallet");
        let address = node.client.get_new_address(Some("label"), Some(AddressType::Legacy)).expect("legacy get_new_address");
        let address = address.into_model().unwrap().0.assume_checked();

        let json: DumpPrivKey = node.client.dump_priv_key(&address).expect("dumpprivkey");
        let model: Result<mtype::DumpPrivKey, _> = json.into_model();
        model.unwrap();
    }

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
        feature = "v20",
        feature = "v21",
        feature = "v22",
    ))]
    {
        let node = Node::with_wallet(Wallet::Default, &[]);
        let address = node.client.new_address().expect("failed to get new address");

        let json: DumpPrivKey = node.client.dump_priv_key(&address).expect("dumpprivkey");
        let model: Result<mtype::DumpPrivKey, _> = json.into_model();
        model.unwrap();
    }
}

#[test]
fn wallet__dump_wallet__modelled() {
    // As of Core v23 the default wallet is an native descriptor wallet which does not
    // support dumping private keys. Legacy wallets are supported upto v25 it seems.
    #[cfg(any(
        feature = "v23",
        feature = "v24",
        feature = "v25",
    ))]
    {
        let node = Node::with_wallet(Wallet::None, &[]);

        node.client.create_legacy_wallet("legacy_wallet").expect("legacy create_wallet");
        let out = integration_test::random_tmp_file();

        let json: DumpWallet = node.client.dump_wallet(&out).expect("dumpwallet");
        let _: mtype::DumpWallet = json.into_model();
    }

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
        feature = "v20",
        feature = "v21",
        feature = "v22",
    ))]
    {
        let node = Node::with_wallet(Wallet::Default, &[]);
        let out = integration_test::random_tmp_file();

        let json: DumpWallet = node.client.dump_wallet(&out).expect("dumpwallet");
        let _: mtype::DumpWallet = json.into_model();
    }
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
    assert!(map.0.get(&addr).is_some());
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
#[cfg(all(not(feature = "v17"), not(feature = "v18")))]
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
fn wallet__load_wallet__modelled() {
    create_load_unload_wallet();
}

#[test]
fn wallet__unload_wallet() {
    create_load_unload_wallet();
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

fn create_load_unload_wallet() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
    node.client.create_wallet(&wallet).expect("failed to create wallet");

    // Upto version 20 Core returns null for `unloadwallet`.
    #[cfg(any(feature = "v17", feature = "v18", feature = "v19", feature = "v20"))]
    let _ = node.client.unload_wallet(&wallet).expect("unloadwallet");

    // From version 21 Core returns warnings for `unloadwallet`.
    #[cfg(all(not(feature = "v17"), not(feature = "v18"), not(feature = "v19"), not(feature = "v20")))]
    {
        let json: UnloadWallet = node.client.unload_wallet(&wallet).expect("unloadwallet");
        let _: mtype::UnloadWallet = json.into_model();
    }

    let _: LoadWallet = node.client.load_wallet(&wallet).expect("loadwallet");
}

#[test]
fn wallet__abandon_transaction() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let (_, txid) = node.create_mempool_transaction();
    let _ = node.client.abandon_transaction(txid);
}

#[test]
fn wallet__abort_rescan() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let result = node.client.abort_rescan();

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
    ))] {
        result.expect("abortrescan RPC call failed (v17-v19)");
    }

    #[cfg(not(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
    )))] {
        let success = result.expect("abortrescan RPC call failed (v20+");
        assert!(!success, "abortrescan should return false when no scan is active (v20+)");
    }
}

#[test]
fn wallet__backup_wallet() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let backup_dest = integration_test::random_tmp_file();

    if backup_dest.exists() {
        fs::remove_file(&backup_dest).expect("Failed to remove pre-existing temp file");
    }

    node.client.backup_wallet(&backup_dest).expect("backupwallet RPC call failed");
    assert!(backup_dest.exists(), "Backup file should exist at destination");
    assert!(backup_dest.is_file(), "Backup destination should be a file");

    fs::remove_file(&backup_dest).expect("Failed to remove backup file during cleanup");
}

#[test]
fn wallet__encrypt_wallet() {
    let wallet_name = format!("test_encrypt_{}", rand::random::<u32>());
    let node = Node::with_wallet(Wallet::None, &[]);
    let _ = node.client.create_wallet(&wallet_name);

    let passphrase = "my_secret_test_passphrase";
    let encrypt_result = node.client.encrypt_wallet(passphrase);

    #[cfg(any(
        feature="v17",
        feature="v18",
        feature="v19"))]
    {
        encrypt_result.expect("encrypt_wallet RPC call failed (v17-v19)");
    }

    #[cfg(not(any(
        feature="v17",
        feature="v18",
        feature="v19")))]
    {
        let return_msg = encrypt_result.expect("encrypt_wallet RPC call failed (v20+)");
        assert!(!return_msg.is_empty(), "encrypt_wallet should return a non-empty string (v20+)");
        assert!(return_msg.contains("wallet encrypted"), "Return message should mention encryption");
    }
}

#[test]
fn wallet__import_address() {
    let node = {
        #[cfg(any(
            feature = "v17",
            feature = "v18",
            feature = "v19",
        ))] {
            Node::with_wallet(Wallet::Default, &[])
        }

        #[cfg(not(any(
            feature = "v17",
            feature = "v18",
            feature = "v19",
        )))] {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            let wallet_name = format!("legacy_import_{}", rand::random::<u32>());
            node.client.create_legacy_wallet(&wallet_name).expect("Failed to create legacy wallet for v20+ test");

            node
        }
    };

        let secp = Secp256k1::new();
        let mut rng = rand::thread_rng();

        // Test Case 1: Import with default label and rescan
        let secret_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let compressed_public_key = CompressedPublicKey(public_key);

        let ext_addr = Address::p2wpkh(&compressed_public_key, Network::Regtest);
        let ext_addr_str = ext_addr.to_string();

        node.client.import_address(&ext_addr_str, None, None, None).expect("importaddress with defaults failed");

        // Test Case 2" Import with label, no rescan, no p2sh
        let secret_key2 = SecretKey::new(&mut rng);
        let public_key2 = PublicKey::from_secret_key(&secp, &secret_key2);
        let compressed_public_key2 = CompressedPublicKey(public_key2);

        let ext_addr2 = Address::p2wpkh(&compressed_public_key2, Network::Regtest);
        let ext_addr_str2 = ext_addr2.to_string();
        let label = "imported_watchonly";

        node.client.import_address(&ext_addr_str2, Some(label), Some(false), Some(false)).expect("importaddress with options failed");
}

#[test]
fn wallet__import_priv_key() {
    let node = {
        #[cfg(any(
            feature = "v17",
            feature = "v18",
            feature = "v19",
        ))] {
            Node::with_wallet(Wallet::Default, &[])
        }

        #[cfg(not(any(
            feature = "v17",
            feature = "v18",
            feature = "v19",
        )))] {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            let wallet_name = format!("legacy_importprivkey_{}", rand::random::<u32>());
            node.client.create_legacy_wallet(&wallet_name).expect("Failed to create legacy wallet for v20+ test");
            node
        }
    };

    let mut rng = rand::thread_rng();

    // Test Case 1: Import key with label, no rescan
    let secret_key = SecretKey::new(&mut rng);
    let private_key = PrivateKey::new(secret_key, Network::Regtest);

    let label = "imported_privkey";
    let _ = node.client.import_priv_key(&private_key, Some(label), Some(false)).expect("importprivkey failed");
}

#[test]
fn wallet__import_pruned_funds() {
    let node = {
        #[cfg(any(
            feature = "v17",
            feature = "v18",
            feature = "v19",
        ))] {
            Node::with_wallet(Wallet::Default, &[])
        }

        #[cfg(not(any(
            feature = "v17",
            feature = "v18",
            feature = "v19",
        )))] {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            let wallet_name = format!("legacy_pruned_{}", rand::random::<u32>());
            node.client.create_legacy_wallet(&wallet_name).expect("Failed to create legacy wallet for v20+ test");

            node
        }
    };

    let dummy_raw_tx = "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff01e8030000000000001976a914000000000000000000000000000000000000000088ac00000000";
    let dummy_tx_proof = "00";

    let _ = node.client.import_pruned_funds(dummy_raw_tx, dummy_tx_proof);
}

#[test]
fn wallet__import_pubkey() {
    let node = {
        #[cfg(any(
            feature = "v17",
            feature = "v18",
            feature = "v19",
        ))] {
            Node::with_wallet(Wallet::Default, &[])
        }

        #[cfg(not(any(
            feature = "v17",
            feature = "v18",
            feature = "v19",
        )))] {
            let node = Node::with_wallet(Wallet::None, &["-deprecatedrpc=create_bdb"]);
            let wallet_name = format!("legacy_importpubkey_{}", rand::random::<u32>());
            node.client.create_legacy_wallet(&wallet_name).expect("Failed to create legacy wallet for v20+");
            node
        }
    };

    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();

    let secret_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let pub_key = bitcoin::PublicKey::new(public_key);

    // Test Case 1: Import with default label and rescan
    let label = "imported_pubkey";
    node.client.import_pubkey(&pub_key, Some(label), Some(false)).expect("importpubkey failed");
}
