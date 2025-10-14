// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Rawtransactions ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.
#![allow(unused_imports)] // Because of feature gated tests.
use bitcoin::address::NetworkUnchecked;
use bitcoin::consensus::encode;
use bitcoin::hashes::{hash160, sha256, Hash};
use bitcoin::hex::FromHex as _;
use bitcoin::key::{Secp256k1, XOnlyPublicKey};
use bitcoin::opcodes::all::*;
use bitcoin::script::Builder;
use bitcoin::{
    absolute, consensus, hex, psbt, script, secp256k1, transaction, Address, Amount, Network,
    PublicKey, ScriptBuf, Transaction, TxOut, WPubkeyHash, WScriptHash,
};
use integration_test::{Node, NodeExt as _, Wallet};
use node::vtype::*;
use node::{mtype, Input, Output}; // All the version specific types.
use rand::Rng;

#[test]
#[cfg(not(feature = "v17"))] // analyzepsbt was added in v0.18.
fn raw_transactions__analyze_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let psbt = create_a_psbt(&node);
    let json: AnalyzePsbt = node.client.analyze_psbt(&psbt).expect("analyzepsbt");
    let model: Result<mtype::AnalyzePsbt, AnalyzePsbtError> = json.into_model();
    model.unwrap();
}

#[test]
fn raw_transactions__combine_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let (_addr, txid) = node.create_mempool_transaction(); // A million sats.
    node.mine_a_block();

    let tx_out = node
        .client
        .get_tx_out(txid, 0) // Might be previous spend or might be change.
        .expect("gettxout")
        .into_model()
        .expect("GetTxOut into model")
        .tx_out;
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    // Calculate the change because we do not know the value of the UTXO.
    let change_amount = tx_out.value - spend_amount - fee;

    let inputs = vec![Input { txid, vout: 0, sequence: None }];

    let mut outputs = vec![];

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to create new address");
    outputs.push(Output::new(spend_address, spend_amount));

    let change_address = node
        .client
        .get_raw_change_address()
        .expect("getrawchangeaddress")
        .into_model()
        .expect("GetRawChangeAddress into model")
        .0
        .assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreatePsbt = node.client.create_psbt(&inputs, &outputs).expect("createpsbt");
    let psbt: Result<mtype::CreatePsbt, psbt::PsbtParseError> = json.clone().into_model();
    let psbt = psbt.unwrap();
    let psbt = psbt.0;

    // Quick and dirty test, just combine the same PSBT with itself.
    let psbts = vec![psbt.clone(), psbt.clone()];

    let json: CombinePsbt = node.client.combine_psbt(&psbts).expect("combinepsbt");
    let model: Result<mtype::CombinePsbt, psbt::PsbtParseError> = json.into_model();
    let combined = model.unwrap();
    // Just for giggles.
    assert_eq!(combined.0, psbt)
}

#[test]
fn raw_transactions__combine_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let (_, txid) = node.create_mempool_transaction();
    let tx =
        node.client.get_raw_transaction(txid).expect("getrawtransaction").transaction().unwrap();

    // Quick and dirty test, just combine the same tx with itself.
    let txs = vec![tx.clone(), tx.clone()];

    let json: CombineRawTransaction =
        node.client.combine_raw_transaction(&txs).expect("combinerawtransaction");
    let model: Result<mtype::CombineRawTransaction, encode::FromHexError> = json.into_model();

    let combined = model.unwrap();
    // Just for giggles.
    assert_eq!(combined.0, tx)
}

#[test]
fn raw_transactions__convert_to_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let tx = create_a_raw_transaction(&node);

    let json: ConvertToPsbt = node.client.convert_to_psbt(&tx).expect("converttopsbt");
    let model: Result<mtype::ConvertToPsbt, psbt::PsbtParseError> = json.into_model();
    model.unwrap();
}

#[test]
fn raw_transactions__create_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    let _ = create_a_psbt(&node);
}

#[test]
fn raw_transactions__create_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    create_sign_send(&node);
}

// Notes on testing decoding of PBST.
//
// - `bip32_derivs` field in the input list of the decoded PSBT changes shape a bunch of times.
// - In v23 a bunch of additional fields are added.
// - In v24 taproot fields are added.
//
// All this should still be handled by `into_model` because `bitcoin::Psbt` has all optional fields.
#[test]
fn raw_transactions__decode_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let mut psbt = create_a_psbt(&node);

    // A bunch of new fields got added in v23.
    //
    // Add an arbitrary global xpub to see if it decodes. Before v23 this will end up in `unknown`,
    // from v23 onwards in should be in its own field.
    {
        use std::collections::BTreeMap;

        use bitcoin::bip32::{DerivationPath, Fingerprint, Xpub};

        let mut map = BTreeMap::default();
        // Some arbitrary xpub I grabbed from rust-bitcoin.
        let xpub = "xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL";
        let xpub = xpub.parse::<Xpub>().expect("failed to parse xpub");
        let fp = Fingerprint::from([1u8, 2, 3, 42]);
        let path =
            "m/84'/0'/0'/0/1".parse::<DerivationPath>().expect("failed to parse derivation path");
        map.insert(xpub, (fp, path));

        psbt.xpub = map;
    }

    let encoded = psbt.to_string();

    let json: DecodePsbt = node.client.decode_psbt(&encoded).expect("decodepsbt");
    let model: Result<mtype::DecodePsbt, DecodePsbtError> = json.into_model();

    #[allow(unused_variables)]
    let decoded = model.unwrap();

    // Before Core v23 global xpubs was not a known keypair.
    #[cfg(feature = "v22_and_below")]
    assert_eq!(decoded.psbt.unknown.len(), 1);

    #[cfg(not(feature = "v22_and_below"))]
    assert_eq!(decoded.psbt.xpub.len(), 1);

    // TODO: Add a taproot field and test it with v24
}

#[test]
fn raw_transactions__decode_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let (_, txid) = node.create_mempool_transaction();

    let tx =
        node.client.get_raw_transaction(txid).expect("getrawtransaction").transaction().unwrap();
    let json: DecodeRawTransaction =
        node.client.decode_raw_transaction(&tx).expect("decoderawtransaction");
    let model: Result<mtype::DecodeRawTransaction, RawTransactionError> = json.into_model();
    model.unwrap();
}

/// Tests the `decodescript` RPC method by verifying it correctly decodes various standard script types.
#[test]
fn raw_transactions__decode_script__modelled() {
    // Initialize test node with graceful handling for missing binary
    let node = match std::panic::catch_unwind(|| Node::with_wallet(Wallet::Default, &["-txindex"]))
    {
        Ok(n) => n,
        Err(e) => {
            let err_msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown initialization error".to_string()
            };
            if err_msg.contains("No such file or directory") {
                println!("[SKIPPED] Bitcoin Core binary not found: {}", err_msg);
                return;
            }
            panic!("Node initialization failed: {}", err_msg);
        }
    };

    node.fund_wallet();
    // Version detection
    let version = node.client.get_network_info().map(|info| info.version).unwrap_or(0);
    let supports_taproot = version >= 210000;
    let is_legacy_version = version < 180000;

    // Basic test cases that work on all versions
    let mut test_cases: Vec<(&str, ScriptBuf, &str, Option<bool>)> = vec![
        ("p2pkh", arbitrary_p2pkh_script(), "pubkeyhash", Some(true)),
        ("multisig", arbitrary_multisig_script(), "multisig", None),
        ("p2sh", arbitrary_p2sh_script(), "scripthash", Some(true)),
        ("bare", arbitrary_bare_script(), "nulldata", Some(false)),
        ("p2wpkh", arbitrary_p2wpkh_script(), "witness_v0_keyhash", Some(true)),
        ("p2wsh", arbitrary_p2wsh_script(), "witness_v0_scripthash", Some(true)),
    ];

    // Check if Taproot is supported (version 0.21.0+)
    if supports_taproot {
        test_cases.push(("p2tr", arbitrary_p2tr_script(), "witness_v1_taproot", Some(true)));
    }
    for (label, script, expected_type, expect_address) in test_cases {
        let hex = script.to_hex_string();
        let json: DecodeScript = match node.client.decode_script(&hex) {
            Ok(j) => j,
            Err(e) if e.to_string().contains("Invalid Taproot script") && !supports_taproot => {
                println!("[SKIPPED] Taproot not supported in this version");
                continue;
            }
            Err(e) => panic!("Failed to decode script for {}: {}", label, e),
        };
        // Handle version-specific type expectations
        let expected_type =
            if label == "p2tr" && !supports_taproot { "witness_unknown" } else { expected_type };
        let model: Result<mtype::DecodeScript, DecodeScriptError> = json.into_model();
        let decoded = match model {
            Ok(d) => d,
            Err(DecodeScriptError::Addresses(_)) if is_legacy_version => {
                println!("[SKIPPED] Segwit address validation not supported in this version");
                continue;
            }
            Err(e) => panic!("Failed to convert to model for {}: {}", label, e),
        };
        // Use script_pubkey field if available, otherwise rely on segwit.hex
        if let Some(script_pubkey) = &decoded.script_pubkey {
            assert_eq!(script_pubkey, &script, "Script hex mismatch for {}", label);
        } else if let Some(segwit) = &decoded.segwit {
            assert_eq!(segwit.hex, &script, "Segwit hex mismatch for {}", label);
        } else {
            println!("[WARNING] Script hex not available for {}", label);
        }

        assert_eq!(decoded.type_, expected_type, "Type mismatch for {}", label);
        if let Some(expected) = expect_address {
            // Version address check
            let has_address = if is_legacy_version && (label == "p2wpkh" || label == "p2wsh") {
                expected
            } else {
                !decoded.addresses.is_empty()
                    || decoded.address.is_some()
                    || (expect_address.unwrap_or(false)
                        && decoded.segwit.as_ref().and_then(|s| s.address.as_ref()).is_some())
            };
            assert_eq!(has_address, expected, "Address mismatch for {}", label);
        }
    }
}
fn arbitrary_p2sh_script() -> ScriptBuf {
    let redeem_script = arbitrary_multisig_script();
    let redeem_script_hash = hash160::Hash::hash(redeem_script.as_bytes());

    script::Builder::new()
        .push_opcode(OP_HASH160)
        .push_slice(redeem_script_hash.as_byte_array())
        .push_opcode(OP_EQUAL)
        .into_script()
}
fn arbitrary_bare_script() -> ScriptBuf {
    script::Builder::new().push_opcode(OP_RETURN).push_slice(b"hello").into_script()
}
fn arbitrary_pubkey() -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(&[1u8; 32]).unwrap();
    PublicKey::new(secp256k1::PublicKey::from_secret_key(&secp, &secret_key))
}
fn arbitrary_p2pkh_script() -> ScriptBuf {
    let pubkey_hash = <[u8; 20]>::from_hex("16e1ae70ff0fa102905d4af297f6912bda6cce19").unwrap();

    script::Builder::new()
        .push_opcode(OP_DUP)
        .push_opcode(OP_HASH160)
        .push_slice(pubkey_hash)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_CHECKSIG)
        .into_script()
}
fn arbitrary_multisig_script() -> ScriptBuf {
    let pk1 =
        <[u8; 33]>::from_hex("022afc20bf379bc96a2f4e9e63ffceb8652b2b6a097f63fbee6ecec2a49a48010e")
            .unwrap();
    let pk2 =
        <[u8; 33]>::from_hex("03a767c7221e9f15f870f1ad9311f5ab937d79fcaeee15bb2c722bca515581b4c0")
            .unwrap();

    script::Builder::new()
        .push_opcode(OP_PUSHNUM_1)
        .push_slice(pk1)
        .push_slice(pk2)
        .push_opcode(OP_PUSHNUM_2)
        .push_opcode(OP_CHECKMULTISIG)
        .into_script()
}
fn arbitrary_p2wpkh_script() -> ScriptBuf {
    let pubkey = arbitrary_pubkey();
    let pubkey_hash = hash160::Hash::hash(&pubkey.to_bytes());

    Builder::new().push_int(0).push_slice(pubkey_hash.as_byte_array()).into_script()
}
fn arbitrary_p2wsh_script() -> ScriptBuf {
    let redeem_script = arbitrary_multisig_script();
    let script_hash = sha256::Hash::hash(redeem_script.as_bytes());

    Builder::new().push_int(0).push_slice(script_hash.as_byte_array()).into_script()
}
fn arbitrary_p2tr_script() -> ScriptBuf {
    let secp = Secp256k1::new();
    let sk = secp256k1::SecretKey::from_slice(&[2u8; 32]).unwrap();
    let internal_key = secp256k1::PublicKey::from_secret_key(&secp, &sk);
    let x_only = XOnlyPublicKey::from(internal_key);

    Builder::new().push_int(1).push_slice(x_only.serialize()).into_script()
}

/// Tests the decoding of Segregated Witness (SegWit) scripts via the `decodescript` RPC.
///
/// This test specifically verifies P2WPKH (Pay-to-Witness-PublicKeyHash) script decoding,
/// ensuring compatibility across different Bitcoin Core versions
#[test]
fn raw_transactions__decode_script_segwit__modelled() {
    // Initialize test node with graceful handling for missing binary
    let node = match std::panic::catch_unwind(|| Node::with_wallet(Wallet::Default, &["-txindex"]))
    {
        Ok(n) => n,
        Err(e) => {
            let err_msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown initialization error".to_string()
            };

            if err_msg.contains("No such file or directory") {
                println!("[SKIPPED] Bitcoin Core binary not found: {}", err_msg);
                return;
            }
            panic!("Node initialization failed: {}", err_msg);
        }
    };

    // Version detection
    let version = node.client.get_network_info().map(|info| info.version).unwrap_or(0);
    let is_legacy_version = version < 180000;
    // Load and fund wallet
    node.client.load_wallet("default").ok();
    node.fund_wallet();

    // Create a P2WPKH script
    let script = arbitrary_p2wpkh_script();
    let hex = script.to_hex_string();

    let json: DecodeScript = node.client.decode_script(&hex).expect("decodescript failed");
    let model: Result<mtype::DecodeScript, DecodeScriptError> = json.into_model();

    let decoded = match model {
        Ok(d) => d,
        Err(DecodeScriptError::Segwit(_)) if is_legacy_version => {
            println!("[SKIPPED] Segwit address validation not supported in this version");
            return;
        }
        Err(DecodeScriptError::Addresses(_)) if is_legacy_version => {
            println!("[SKIPPED] Address validation not fully supported in this version");
            return;
        }
        Err(e) => panic!("Failed to convert to model: {}", e),
    };
    // Validate segwit-specific fields if present
    if let Some(segwit) = &decoded.segwit {
        // Use the hex field from segwit struct
        assert_eq!(segwit.hex, script, "Segwit hex does not match script");

        if let Some(addr) = &segwit.address {
            let checked_addr = addr.clone().assume_checked();
            assert!(
                checked_addr.script_pubkey().is_witness_program(),
                "Invalid witness address: {:?}",
                checked_addr
            );
        }

        if let Some(desc) = &segwit.descriptor {
            assert!(
                desc.starts_with("addr(") || desc.starts_with("wpkh("),
                "Invalid descriptor format: {}",
                desc
            );
        }

        if let Some(p2sh_segwit) = &segwit.p2sh_segwit {
            let p2sh_spk = p2sh_segwit.clone().assume_checked().script_pubkey();
            assert!(p2sh_spk.is_p2sh(), "Invalid P2SH-SegWit address");
        }
    } else {
        // For legacy versions, skip some validations
        if is_legacy_version {
            println!(
                "[NOTE] Segwit field not present in legacy version - skipping detailed validation"
            );
            // use script_pubkey instead of hex field
            if let Some(script_pubkey) = &decoded.script_pubkey {
                assert_eq!(script_pubkey, &script, "Script hex mismatch");
            }
            assert!(!decoded.type_.is_empty(), "Script type should not be empty");
            return;
        }
        // validation for modern versions that have segwit field
        if let Some(script_pubkey) = &decoded.script_pubkey {
            assert_eq!(script_pubkey, &script, "Script hex mismatch in script_pubkey field");
        } else {
            println!(
                "[WARNING] Script hex not returned in decode_script response for segwit script"
            );
        }
        if let Some(addr) = &decoded.address {
            let checked_addr = addr.clone().assume_checked();
            // For P2WPKH,expect a witness program
            assert!(
                checked_addr.script_pubkey().is_witness_program(),
                "Invalid witness address: {:?}",
                checked_addr
            );
        } else {
            println!("[NOTE] Address not returned in decode_script response");
        }

        println!(
            "[NOTE] Segwit field not present in decode_script response - using fallback validation"
        );
    }
    // Use script_pubkey field if available, otherwise rely on segwit.hex
    if let Some(script_pubkey) = &decoded.script_pubkey {
        assert_eq!(script_pubkey, &script, "Script does not match");
    } else if let Some(segwit) = &decoded.segwit {
        assert_eq!(segwit.hex, script, "Segwit script does not match");
    }
    assert!(!decoded.type_.is_empty(), "Script type should not be empty");
    assert!(
        decoded.type_.contains("witness") || decoded.type_ == "witness_v0_keyhash",
        "Expected witness script type, got: {}",
        decoded.type_
    );
}

#[test]
fn raw_transactions__finalize_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    // Create a PSBT and call finalizepsbt directly without signing.
    // This still exercises the RPC and model; it should report complete=false and return the PSBT.
    let psbt = create_a_psbt(&node);
    let json: FinalizePsbt = node.client.finalize_psbt(&psbt).expect("finalizepsbt");
    let model: Result<mtype::FinalizePsbt, FinalizePsbtError> = json.into_model();
    let finalized = model.unwrap();

    assert!(!finalized.complete);
    assert_eq!(finalized.psbt, Some(psbt));
}

#[test]
fn raw_transactions__fund_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    create_fund_sign_send(&node);
}

#[test]
fn raw_transactions__send_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    create_sign_send(&node); // Calls `sendrawtransaction`.
}

#[test]
fn raw_transactions__get_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    // Get raw transaction using a mined transaction and verbose = false.
    let (_, tx) = node.create_mined_transaction();
    let json: GetRawTransaction =
        node.client.get_raw_transaction(tx.compute_txid()).expect("getrawtransaction");
    let model: Result<mtype::GetRawTransaction, encode::FromHexError> = json.into_model();
    model.unwrap();

    // Get raw transaction using a mined transaction and verbose = true.
    let (_, tx) = node.create_mined_transaction();
    let json: GetRawTransactionVerbose = node
        .client
        .get_raw_transaction_verbose(tx.compute_txid())
        .expect("getrawtransaction verbose");
    let model: Result<mtype::GetRawTransactionVerbose, GetRawTransactionVerboseError> =
        json.into_model();
    model.unwrap();

    // Get raw transaction using an un-mined transaction.
    let (_, txid) = node.create_mempool_transaction();
    let _ = node
        .client
        .get_raw_transaction_verbose(txid)
        .expect("getrawtransaction verbose")
        .into_model()
        .unwrap();
}

#[test]
#[cfg(not(feature = "v17"))]
fn raw_transactions__join_psbts__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let psbt1 = create_a_psbt(&node);
    let psbt2 = create_a_psbt(&node);

    let json: JoinPsbts =
        node.client.join_psbts(&[psbt1.clone(), psbt2.clone()]).expect("joinpsbts");
    let model: Result<mtype::JoinPsbts, psbt::PsbtParseError> = json.into_model();
    let join_psbts = model.unwrap();

    assert_eq!(join_psbts.0.inputs.len(), psbt1.inputs.len() + psbt2.inputs.len());
}

#[test]
fn raw_transactions__sign_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    create_sign_send(&node);
}

// TODO: Work out how to test things without using dumpprivkey.
#[test]
#[cfg(feature = "v22_and_below")] // In v23 dumpprivkey no longer works.
fn raw_transactions__sign_raw_transaction_with_key__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    create_sign_with_key_send(&node)
}

// FIXME: Doesn't work for v26 for some reason.
#[test]
#[cfg(all(feature = "v27_and_below", not(feature = "v26_and_below")))]
fn raw_transactions__submit_package__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);

    // Submitting the empty package should simply fail.
    assert!(node.client.submit_package(&[]).is_err());

    node.fund_wallet();

    let (_, tx_0) = node.create_mined_transaction();
    let (_, tx_1) = node.create_mined_transaction();

    // The call for submitting this package should succeed, but yield an 'already known'
    // error for all transactions.
    let json: SubmitPackage =
        node.client.submit_package(&[tx_0, tx_1]).expect("failed to submit package");
    let model: Result<mtype::SubmitPackage, SubmitPackageError> = json.into_model();
    let submit_package = model.unwrap();
    for tx_result in submit_package.tx_results.values() {
        assert!(tx_result.error.is_some());
    }
    assert!(submit_package.replaced_transactions.is_empty());
}

// In Core v28 submitpackage has additional optional features.
#[test]
#[cfg(not(feature = "v27_and_below"))]
fn raw_transactions__submit_package__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);

    // Submitting the empty package should simply fail.
    assert!(node.client.submit_package(&[], None, None).is_err());

    node.fund_wallet();

    let (_, tx_0) = node.create_mined_transaction();
    let (_, tx_1) = node.create_mined_transaction();

    // The call for submitting this package should succeed, but yield an 'already known'
    // error for all transactions.
    let json: SubmitPackage =
        node.client.submit_package(&[tx_0, tx_1], None, None).expect("failed to submit package");
    let model: Result<mtype::SubmitPackage, SubmitPackageError> = json.into_model();
    let submit_package = model.unwrap();
    for tx_result in submit_package.tx_results.values() {
        assert!(tx_result.error.is_some());
    }
    assert!(submit_package.replaced_transactions.is_empty());
}

#[test]
fn raw_transactions__test_mempool_accept__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let tx = create_a_raw_transaction(&node);

    // Sign (but don't broadcast).
    let signed: SignRawTransactionWithWallet =
        node.client.sign_raw_transaction_with_wallet(&tx).expect("signrawtransactionwithwallet");
    let signed_model: mtype::SignRawTransactionWithWallet =
        signed.into_model().expect("SignRawTransaction into model");
    let signed_tx = signed_model.tx;

    // Call testmempoolaccept with the valid (not yet broadcast) transaction.
    let json: TestMempoolAccept = node
        .client
        .test_mempool_accept(std::slice::from_ref(&signed_tx))
        .expect("testmempoolaccept");
    #[cfg(feature = "v20_and_below")]
    type TestMempoolAcceptError = hex::HexToArrayError;
    let model: Result<mtype::TestMempoolAccept, TestMempoolAcceptError> = json.into_model();
    let test_mempool = model.unwrap();

    assert_eq!(test_mempool.results.len(), 1);
    let res = &test_mempool.results[0];
    assert_eq!(res.txid, signed_tx.compute_txid());
    assert!(res.allowed, "fresh signed tx should be allowed");
}

#[test]
#[cfg(not(feature = "v17"))]
fn raw_transactions__utxo_update_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let psbt = create_a_psbt(&node);
    let json: UtxoUpdatePsbt = node.client.utxo_update_psbt(&psbt).expect("utxoupdatepsbt");
    let model: Result<mtype::UtxoUpdatePsbt, psbt::PsbtParseError> = json.into_model();
    let update_psbts = model.unwrap();

    assert!(update_psbts.0.inputs.len() >= psbt.inputs.len());
}

// Manipulates raw transactions.
//
// Calls the following RPC methods:
// - create_raw_transaction
// - sign_raw_transaction_with_wallet
// - send_raw_transaction
fn create_sign_send(node: &Node) {
    let (_addr, _tx, txid, tx_out, vout) = create_utxo(node);

    // Assumes tx_out has a million sats in it.
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    let change_amount = tx_out.value - spend_amount - fee;

    let inputs = vec![Input { txid, vout, sequence: None }];

    let mut outputs = vec![];

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to create new address");
    outputs.push(Output::new(spend_address, spend_amount));

    let change_address = node
        .client
        .get_raw_change_address()
        .expect("getrawchangeaddress")
        .into_model()
        .expect("GetRawChangeAddress into model")
        .0
        .assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreateRawTransaction =
        node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let model: Result<mtype::CreateRawTransaction, encode::FromHexError> =
        json.clone().into_model();
    model.unwrap();
    let tx = json.transaction().unwrap();

    // wallet.rs expects this call to exist, if you change it then you'll need to update the test
    // `wallet__sign_raw_transaction_with_wallet__modelled`.
    let json: SignRawTransactionWithWallet =
        node.client.sign_raw_transaction_with_wallet(&tx).expect("signrawtransactionwithwallet");

    let model: Result<mtype::SignRawTransactionWithWallet, SignRawTransactionError> =
        json.into_model();
    let sign_raw_transaction = model.unwrap();

    // The proves we did everything correctly.
    let json: SendRawTransaction =
        node.client.send_raw_transaction(&sign_raw_transaction.tx).expect("sendrawtransaction");
    let model: Result<mtype::SendRawTransaction, hex::HexToArrayError> = json.into_model();
    model.unwrap();
}

// Manipulates raw transactions.
//
// Calls the following RPC methods:
// - create_raw_transaction
// - sign_raw_transaction_with_key (sign_raw_transaction was deprecated in v0.17).
// - send_raw_transaction
//
// TODO: Work out how to get a private key without using `dumpprivkey`.
#[cfg(feature = "v22_and_below")] // In v23 dumpprivkey no longer works.
fn create_sign_with_key_send(node: &Node) {
    let (addr, _tx, txid, tx_out, vout) = create_utxo(node);

    // Assumes tx_out has a million sats in it.
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    let change_amount = tx_out.value - spend_amount - fee;

    let inputs = vec![Input { txid, vout, sequence: None }];

    let mut outputs = vec![];

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to create new address");
    outputs.push(Output::new(spend_address, spend_amount));

    let change_address = node
        .client
        .get_raw_change_address()
        .expect("getrawchangeaddress")
        .into_model()
        .expect("GetRawChangeAddress into model")
        .0
        .assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreateRawTransaction =
        node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let model: Result<mtype::CreateRawTransaction, encode::FromHexError> =
        json.clone().into_model();
    model.unwrap();
    let tx = json.transaction().unwrap();

    let json: DumpPrivKey = node.client.dump_priv_key(&addr).expect("dumpprivkey");
    let model: mtype::DumpPrivKey = json.into_model().expect("DumpPrivKey");
    let key = model.0;

    let json: SignRawTransactionWithKey =
        node.client.sign_raw_transaction_with_key(&tx, &[key]).expect("signrawtransactionwithkey");
    let model: Result<mtype::SignRawTransactionWithKey, SignRawTransactionError> =
        json.into_model();
    let sign_raw_transaction = model.unwrap();

    // The proves we did everything correctly.
    let json: SendRawTransaction =
        node.client.send_raw_transaction(&sign_raw_transaction.tx).expect("sendrawtransaction");
    let model: Result<mtype::SendRawTransaction, hex::HexToArrayError> = json.into_model();
    model.unwrap();
}

// Manipulates raw transactions.
//
// Calls the following RPC methods:
// - fund_raw_transaction
// - sign_raw_transaction_with_wallet (sign_raw_transaction was deprecated in v0.17).
// - send_raw_transaction
#[allow(clippy::inconsistent_digit_grouping)] // Sats to btc is a common use case.
fn create_fund_sign_send(node: &Node) {
    let (_addr, _tx, txid, _tx_out, vout) = create_utxo(node);

    // We need to add an input so that transaction is consensus encoded to hex correctly (because of
    // different encoding for segwit and non-segwit transactions).
    let inputs = vec![Input { txid, vout, sequence: None }];
    let mut outputs = vec![];

    let spend_amount = Amount::from_sat(50_00_000_000);
    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to create new address");
    outputs.push(Output::new(spend_address, spend_amount));

    let json: CreateRawTransaction =
        node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let model: Result<mtype::CreateRawTransaction, encode::FromHexError> =
        json.clone().into_model();
    model.unwrap();
    let tx = json.transaction().unwrap();

    let json: FundRawTransaction =
        node.client.fund_raw_transaction(&tx).expect("fundrawtransaction");
    let model: Result<mtype::FundRawTransaction, FundRawTransactionError> =
        json.clone().into_model();
    model.unwrap();
    let funded = json.transaction().unwrap();

    // This method is from the wallet section.
    let json: SignRawTransactionWithWallet = node
        .client
        .sign_raw_transaction_with_wallet(&funded)
        .expect("signrawtransactionwithwallet");
    // This proves we did everything correctly.
    let model: Result<mtype::SignRawTransactionWithWallet, SignRawTransactionError> =
        json.into_model();
    let sign_raw_transaction = model.unwrap();
    let _ =
        node.client.send_raw_transaction(&sign_raw_transaction.tx).expect("createrawtransaction");
}

// Creates a transaction using client to do RPC call `create_raw_transaction`.
fn create_a_raw_transaction(node: &Node) -> Transaction {
    let (_addr, _tx, txid, tx_out, vout) = create_utxo(node);

    // Assumes tx_out has a million sats in it.
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    let change_amount = tx_out.value - spend_amount - fee;

    let inputs = vec![Input { txid, vout, sequence: None }];

    let mut outputs = vec![];

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to create new address");
    outputs.push(Output::new(spend_address, spend_amount));

    let change_address = node
        .client
        .get_raw_change_address()
        .expect("getrawchangeaddress")
        .into_model()
        .expect("GetRawChangeAddress into model")
        .0
        .assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreateRawTransaction =
        node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let model: Result<mtype::CreateRawTransaction, encode::FromHexError> =
        json.clone().into_model();
    model.unwrap();
    json.transaction().unwrap()
}

// Sends a transaction, mines a block then grabs a million sat UTXO from the mined transaction.
fn create_utxo(
    node: &Node,
) -> (bitcoin::Address, bitcoin::Transaction, bitcoin::Txid, bitcoin::TxOut, u64) {
    // TODO: We should probably pass this into `create_mined_transaction`.
    const MILLION_SATS: bitcoin::Amount = bitcoin::Amount::from_sat(1000000);

    let (addr, tx) = node.create_mined_transaction(); // A million sat transaction.
    let txid = tx.compute_txid();

    // We don't know which output is the spend and which is the change
    // so we check for value of MILLION_SATS.
    let tx_out = node
        .client
        .get_tx_out(txid, 0)
        .expect("gettxout")
        .into_model()
        .expect("GetTxOut into model")
        .tx_out;

    let (tx_out, vout) = if tx_out.value == MILLION_SATS {
        (tx_out, 0)
    } else {
        let out = node
            .client
            .get_tx_out(txid, 1)
            .expect("gettxout")
            .into_model()
            .expect("GetTxOut into model")
            .tx_out;
        (out, 1)
    };
    (addr, tx, txid, tx_out, vout)
}

// Creates a PSBT using client to do RPC call `create_psbt`.
fn create_a_psbt(node: &Node) -> bitcoin::Psbt {
    let (_addr, _tx, txid, tx_out, vout) = create_utxo(node);

    // Assumes tx_out has a million sats in it.
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    let change_amount = tx_out.value - spend_amount - fee;

    let inputs = vec![Input { txid, vout, sequence: None }];

    let mut outputs = vec![];

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to create new address");
    outputs.push(Output::new(spend_address, spend_amount));

    let change_address = node
        .client
        .get_raw_change_address()
        .expect("getrawchangeaddress")
        .into_model()
        .expect("GetRawChangeAddress into model")
        .0
        .assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreatePsbt = node.client.create_psbt(&inputs, &outputs).expect("createpsbt");
    let model: Result<mtype::CreatePsbt, psbt::PsbtParseError> = json.clone().into_model();
    let psbt = model.unwrap();
    psbt.0
}
