// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Rawtransactions ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.
#![allow(unused_imports)] // Because of feature gated tests.
use integration_test::{Node, NodeExt as _, Wallet};
use node::{mtype, Input, Output};
use node::vtype::*;             // All the version specific types.
use bitcoin::{hex::FromHex as _,
    absolute, transaction, consensus,Amount, TxOut, Transaction,
    Address, Network, ScriptBuf,script, hashes::{hash160,sha256,Hash},
    WPubkeyHash, WScriptHash, secp256k1,
    PublicKey,
    script::Builder,
    opcodes::all::*,
    key::{Secp256k1, XOnlyPublicKey},
    address::NetworkUnchecked,
};
use rand::Rng;


#[test]
#[cfg(not(feature = "v17"))]    // analyzepsbt was added in v0.18.
fn raw_transactions__analyze_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let psbt = create_a_psbt(&node);
    let json: AnalyzePsbt = node.client.analyze_psbt(&psbt).expect("analyzepsbt");
    let model: Result<mtype::AnalyzePsbt, _> = json.into_model();
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
        .get_tx_out(txid, 0)    // Might be previous spend or might be change.
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
    let res: Result<mtype::CreatePsbt, _> = json.clone().into_model();
    let psbt = res.expect("CreatePsbt into model");
    let psbt = psbt.0;

    // Quick and dirty test, just combine the same PSBT with itself.
    let psbts = vec![psbt.clone(), psbt.clone()];

    let json: CombinePsbt = node.client.combine_psbt(&psbts).expect("combinepsbt");
    let model: Result<mtype::CombinePsbt, _> = json.into_model();
    let combined = model.expect("CombinePsbt into model");
    // Just for giggles.
    assert_eq!(combined.0, psbt)
}

#[test]
fn raw_transactions__combine_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let (_, txid) = node.create_mempool_transaction();
    let tx = node
        .client
        .get_raw_transaction(txid)
        .expect("getrawtransaction")
        .transaction()
        .expect("GetRawTransaction into model");

    // Quick and dirty test, just combine the same tx with itself.
    let txs = vec![tx.clone(), tx.clone()];

    let json: CombineRawTransaction = node.client.combine_raw_transaction(&txs).expect("combinerawtransaction");
    let model: Result<mtype::CombineRawTransaction, _> = json.into_model();

    let combined = model.expect("CombineRawTransaction into model");
    // Just for giggles.
    assert_eq!(combined.0, tx)
}

#[test]
fn raw_transactions__convert_to_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let tx = create_a_raw_transaction(&node);

    let json: ConvertToPsbt = node.client.convert_to_psbt(&tx).expect("converttopsbt");
    let model: Result<mtype::ConvertToPsbt, _> = json.into_model();
    let _ = model.expect("ConvertToPsbt into model");
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
        use bitcoin::bip32::{Fingerprint, DerivationPath, Xpub};

        let mut map = BTreeMap::default();
        // Some arbitrary xpub I grabbed from rust-bitcoin.
        let xpub = "xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL";
        let xpub = xpub.parse::<Xpub>().expect("failed to parse xpub");
        let fp = Fingerprint::from([1u8, 2, 3, 42]);
        let path = "m/84'/0'/0'/0/1".parse::<DerivationPath>().expect("failed to parse derivation path");
        map.insert(xpub, (fp, path));

        psbt.xpub = map;
    }

    let encoded = psbt.to_string();

    let json: DecodePsbt = node.client.decode_psbt(&encoded).expect("decodepsbt");
    let res: Result<mtype::DecodePsbt, DecodePsbtError> = json.into_model();

    #[allow(unused_variables)]
    let decoded = res.expect("DecodePsbt into model");

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

    let tx = node
        .client
        .get_raw_transaction(txid)
        .expect("getrawtransaction")
        .transaction()
        .expect("GetRawTransaction into model");
    let json = node.client.decode_raw_transaction(&tx).expect("decoderawtransaction");
    let model: Result<mtype::DecodeRawTransaction, RawTransactionError> = json.into_model();
    model.expect("DecodeRawTransaction into model");
}

#[test]
// FIXME: Seems the returned fields are  different depending on the script. Needs more thorough testing.
fn raw_transactions__decode_script__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let test_cases: Vec<(&str, ScriptBuf, Option<&str>)> = vec![
        ("p2pkh", arbitrary_p2pkh_script(), Some("pubkeyhash")),
        ("multisig", arbitrary_multisig_script(), Some("multisig")),
        ("p2sh", arbitrary_p2sh_script(), Some("scripthash")),
        ("bare", arbitrary_bare_script(), Some("nonstandard")),
        ("p2wpkh", arbitrary_p2wpkh_script(), Some("witness_v0_keyhash")),
        ("p2wsh", arbitrary_p2wsh_script(), Some("witness_v0_scripthash")),
        ("p2tr", arbitrary_p2tr_script(), Some("witness_v1_taproot")),
    ];

    for (label, script, expected_type) in test_cases {
        let hex = script.to_hex_string();

        let json: DecodeScript = node.client.decode_script(&hex).expect("decodescript");
        let model: Result<mtype::DecodeScript, DecodeScriptError> = json.into_model();
        let decoded = model.expect("DecodeScript into model");

        println!("Decoded script ({label}): {:?}", decoded);

        if let Some(expected) = expected_type {
            assert_eq!(decoded.type_, expected, "Unexpected script type for {label}");
        } else {
            println!("Skipping type check for {}", label);
        }

        // Address should be present for standard scripts
        if expected_type != Some("nonstandard") {
            let has_any_address = !decoded.addresses.is_empty() || decoded.address.is_some();
            assert!(
                has_any_address,
                "Expected at least one address for {label}"
            );
        }
    }
}
fn arbitrary_p2sh_script() -> ScriptBuf {

    let redeem_script = arbitrary_multisig_script(); // or arbitrary_p2pkh_script()
    let redeem_script_hash = hash160::Hash::hash(redeem_script.as_bytes());

    script::Builder::new()
        .push_opcode(bitcoin::opcodes::all::OP_HASH160)
        .push_slice(redeem_script_hash.as_byte_array())  // [u8; 20]
        .push_opcode(bitcoin::opcodes::all::OP_EQUAL)
        .into_script()
}
fn arbitrary_bare_script() -> ScriptBuf {
    script::Builder::new()
        .push_opcode(OP_RETURN)
        .push_slice(b"hello")
        .into_script()
}
fn arbitrary_pubkey() -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(&[1u8; 32]).unwrap();
    PublicKey::new(secp256k1::PublicKey::from_secret_key(&secp, &secret_key))
}
// Script builder code copied from rust-bitcoin script unit tests.
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
        .push_opcode(OP_PUSHBYTES_33)
        .push_slice(pk1)
        .push_opcode(OP_PUSHBYTES_33)
        .push_slice(pk2)
        .push_opcode(OP_PUSHNUM_2)
        .push_opcode(OP_CHECKMULTISIG)
        .into_script()
}
fn arbitrary_p2wpkh_script() -> ScriptBuf {
    let pubkey = arbitrary_pubkey();
    let pubkey_hash = hash160::Hash::hash(&pubkey.to_bytes());

    // P2WPKH: 0 <20-byte pubkey hash>
    Builder::new()
        .push_int(0)
        .push_slice(pubkey_hash.as_byte_array())
        .into_script()
}

fn arbitrary_p2wsh_script() -> ScriptBuf {
    let redeem_script = arbitrary_multisig_script(); // any witness script
    let script_hash = sha256::Hash::hash(redeem_script.as_bytes());

    // P2WSH: 0 <32-byte script hash>
    Builder::new()
        .push_int(0)
        .push_slice(script_hash.as_byte_array())
        .into_script()
}

fn arbitrary_p2tr_script() -> ScriptBuf {
    let secp = Secp256k1::new();
    let sk = secp256k1::SecretKey::from_slice(&[2u8; 32]).unwrap();
    let internal_key = secp256k1::PublicKey::from_secret_key(&secp, &sk);
    let x_only = XOnlyPublicKey::from(internal_key);

    // Taproot output script: OP_1 <x-only pubkey>
    Builder::new()
        .push_int(1)
        .push_slice(&x_only.serialize())
        .into_script()
}

#[test]
fn raw_transactions__decode_script_segwit__modelled() {

    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.client.load_wallet("default").ok(); // Ensure wallet is loaded
    node.fund_wallet();

    // Get a new address and script
    let address_unc = node
        .client
        .get_new_address(None, None)
        .expect("getnewaddress")
        .address()
        .expect("valid address string");

    let address = address_unc
        .require_network(Network::Regtest)
        .expect("must be regtest");

    assert!(
        address.is_segwit(),
        "Expected SegWit address but got {:?}",
        address
    );

    let script = address.script_pubkey();
    let hex = script.to_hex_string();

    // Decode script
    let json = node.client.decode_script(&hex).expect("decodescript");
    let model: Result<mtype::DecodeScript, DecodeScriptError> = json.into_model();
    let decoded = model.expect("DecodeScript into model");

    let segwit = decoded
        .segwit
        .as_ref()
        .expect("Expected segwit field to be present");
    
    assert_eq!(
        segwit.hex, script,
        "Segwit hex does not match script"
    );

    // Extract the type field
    let script_type = decoded
        .segwit
        .as_ref()
        .map(|s| s.type_.as_str())
        .unwrap_or_else(|| decoded.type_.as_str());

    assert_eq!(
        script_type,
        "witness_v0_keyhash",
        "Expected script type to be witness_v0_keyhash"
    );

    // Compare hex from segwit
    let decoded_hex = decoded
        .segwit
        .as_ref()
        .map(|s| &s.hex)
        .unwrap_or_else(|| {
            panic!("Expected segwit hex to be present")
        });

    assert_eq!(*decoded_hex, script, "Script hex does not match");

    // Compare addresses from segwit or fallback
    let address_unc_check = address.into_unchecked();
    let segwit_addresses = decoded
        .segwit
        .as_ref()
        .map(|s| &s.addresses)
        .unwrap_or(&decoded.addresses);

    assert!(
        segwit_addresses.iter().any(|a| a == &address_unc_check),
        "Expected address {:?} in segwit.addresses or top-level addresses: {:?}",
        address_unc_check,
        segwit_addresses
    );
}

#[test]
#[cfg(feature = "TODO")]
fn raw_transactions__finalize_psbt__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let (addr, _tx, txid, tx_out, vout) = create_utxo(&node);

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
    let res: Result<mtype::CreatePsbt, _> = json.clone().into_model();
    let psbt = res.expect("CreatePsbt into model");
    let psbt = psbt.0;

    let json: DumpPrivKey = node.client.dump_priv_key(&addr).expect("dumpprivkey");
    let model: mtype::DumpPrivKey = json.into_model().expect("DumpPrivKey");
    let key = model.0;

    let json: SignRawTransaction = node
        .client
        .sign_raw_transaction_with_key(&psbt.unsigned_tx, &[key])
        .expect("signrawtransactionwithkey");
    let res: Result<mtype::SignRawTransaction, SignRawTransactionError> = json.into_model();
    let model = res.expect("SignRawTransaction into model");

    // FIXME: Core errors here with: code: -22, message: "TX decode failed"
    let json: ConvertToPsbt = node.client.convert_to_psbt(&model.tx).expect("converttopsbt");
    let model: Result<mtype::ConvertToPsbt, _> = json.into_model();
    let psbt = model.expect("ConvertToPsbt into model").0;

    let json: FinalizePsbt = node.client.finalize_psbt(&psbt).expect("finalizepsbt");
    let model: Result<mtype::FinalizePsbt, _> = json.into_model();
    let _ = model.expect("FinalizePsbt into model");
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
    create_sign_send(&node);    // Calls `sendrawtransaction`.
}

#[test]
fn raw_transactions__get_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    // Get raw transaction using a mined transaction and verbose = false.
    let (_, tx) = node.create_mined_transaction();
    let json: GetRawTransaction =
        node.client.get_raw_transaction(tx.compute_txid()).expect("getrawtransaction");
    let model: Result<mtype::GetRawTransaction, _> = json.into_model();
    model.expect("GetRawTransaction into model");

    // Get raw transaction using a mined transaction and verbose = true.
    let (_, tx) = node.create_mined_transaction();
    let json = node
        .client
        .get_raw_transaction_verbose(tx.compute_txid())
        .expect("getrawtransaction verbose");
    let model: Result<mtype::GetRawTransactionVerbose, GetRawTransactionVerboseError> =
        json.into_model();
    model.expect("GetRawTransactionVerbose into model");

    // Get raw transaction using an un-mined transaction.
    let (_, txid) = node.create_mempool_transaction();
    let _ = node
        .client
        .get_raw_transaction_verbose(txid)
        .expect("getrawtransaction verbose")
        .into_model()
        .expect("GetRawTransactionVerbose into model");

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
    let res = node
        .client
        .submit_package(&[tx_0, tx_1])
        .expect("failed to submit package")
        .into_model()
        .expect("failed to submit package");
    for tx_result in res.tx_results.values() {
        assert!(tx_result.error.is_some());
    }
    assert!(res.replaced_transactions.is_empty());
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
    let res = node
        .client
        .submit_package(&[tx_0, tx_1], None, None)
        .expect("failed to submit package")
        .into_model()
        .expect("failed to submit package");
    for tx_result in res.tx_results.values() {
        assert!(tx_result.error.is_some());
    }
    assert!(res.replaced_transactions.is_empty());
}

#[test]
#[cfg(feature = "TODO")]
fn raw_transactions__test_mempool_accept__modelled() {}

#[test]
#[cfg(not(feature = "v17"))]    // utxoupdatepsbt was added in v0.18.
fn raw_transactions__utxo_update_psbt() {}

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
    let res: Result<mtype::CreateRawTransaction, _> = json.clone().into_model();
    let _ = res.expect("CreateRawTransaction into model");
    let tx = json.transaction().unwrap();

    // wallet.rs expects this call to exist, if you change it then you'll need to update the test
    // `wallet__sign_raw_transaction_with_wallet__modelled`.
    let json: SignRawTransaction =
        node.client.sign_raw_transaction_with_wallet(&tx).expect("signrawtransactionwithwallet");

    let res: Result<mtype::SignRawTransaction, SignRawTransactionError> = json.into_model();
    let model = res.expect("SignRawTransactionWithWallet into model");

    // The proves we did everything correctly.
    let json: SendRawTransaction =
        node.client.send_raw_transaction(&model.tx).expect("sendrawtransaction");
    let res: Result<mtype::SendRawTransaction, _> = json.into_model();
    let _ = res.expect("SendRawTransaction into model");
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
    let res: Result<mtype::CreateRawTransaction, _> = json.clone().into_model();
    let _ = res.expect("CreateRawTransaction into model");
    let tx = json.transaction().unwrap();

    let json: DumpPrivKey = node.client.dump_priv_key(&addr).expect("dumpprivkey");
    let model: mtype::DumpPrivKey = json.into_model().expect("DumpPrivKey");
    let key = model.0;

    let json: SignRawTransaction =
        node.client.sign_raw_transaction_with_key(&tx, &[key]).expect("signrawtransactionwithkey");
    let res: Result<mtype::SignRawTransaction, SignRawTransactionError> = json.into_model();
    let model = res.expect("SignRawTransaction into model");

    // The proves we did everything correctly.
    let json: SendRawTransaction =
        node.client.send_raw_transaction(&model.tx).expect("sendrawtransaction");
    let res: Result<mtype::SendRawTransaction, _> = json.into_model();
    let _ = res.expect("SendRawTransaction into model");
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
    let res: Result<mtype::CreateRawTransaction, _> = json.clone().into_model();
    let _ = res.expect("CreateRawTransaction into model");
    let tx = json.transaction().unwrap();

    let json: FundRawTransaction =
        node.client.fund_raw_transaction(&tx).expect("fundrawtransaction");
    let res: Result<mtype::FundRawTransaction, FundRawTransactionError> = json.clone().into_model();
    let _ = res.expect("FundRawTransaction into model");
    let funded = json.transaction().unwrap();

    // This method is from the wallet section.
    let json = node.client.sign_raw_transaction_with_wallet(&funded).expect("signrawtransactionwithwallet");

    // The proves we did everything correctly.
    let model = json.into_model().expect("SignRawTransactionWithWallet into model");
    let _ = node.client.send_raw_transaction(&model.tx).expect("createrawtransaction");
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
    let res: Result<mtype::CreateRawTransaction, _> = json.clone().into_model();
    let _ = res.expect("CreateRawTransaction into model");
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
    let res: Result<mtype::CreatePsbt, _> = json.clone().into_model();
    let psbt = res.expect("CreatePsbt into model");
    psbt.0
}
