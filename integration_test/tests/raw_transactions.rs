// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Rawtransactions ==` section of the API docs.

#[allow(unused_imports)] // Just being lazy, some tests are currently broken for some versions.
use bitcoin::{absolute, transaction, Amount, Transaction, TxOut};
use bitcoin::opcodes::all::*;
use bitcoin::hex::FromHex as _;
use bitcoin::script::{self, ScriptBuf};
use client::client_sync::{Input, Output};
use client::types::model;
use node::types::*;
use integration_test::{Node, NodeExt as _, Wallet};

// Note, this RPC call implements the combiner role.
#[test]
fn combine_psbt() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let psbt = create_a_psbt(&node);

    // Quick and dirty test, just combine the same PSBT with itself.
    let psbts = vec![psbt.clone(), psbt];

    let _ = node.client.combine_psbt(&psbts).expect("combinepsbt");
}

#[test]
fn combine_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let (_, txid) = node.create_mempool_transaction();
    let tx = node.client.get_raw_transaction(txid).expect("getrawtransaction").transaction().expect("GetRawTransaction into model");

    // Quick and dirty test, just combine the same tx with itself.
    let txs = vec![tx.clone(), tx.clone()];
    let json: CombineRawTransaction = node.client.combine_raw_transaction(&txs).expect("combinerawtransaction");
    let model: Result<model::CombineRawTransaction, _> = json.into_model();
    let combined = model.expect("CombineRawTransaction into model");
    assert_eq!(combined.0, tx)
}

#[test]
fn convert_to_psbt() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let tx = create_a_raw_transaction(&node);

    let json: ConvertToPsbt = node.client.convert_to_psbt(&tx).expect("converttopsbt");
    let model: Result<model::ConvertToPsbt, _> = json.into_model();
    let _ = model.expect("ConvertToPsbt into model");
}

#[test]
fn create_psbt() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    let _ = create_a_psbt(&node);
}

#[test]
fn create_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    raw_transaction_create_sign_send(&node);
}

#[test]
fn decode_psbt() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    let psbt = create_a_psbt(&node);
    let encoded = psbt.to_string();

    let _json: DecodePsbt = node.client.decode_psbt(&encoded).expect("decodepsbt");
    // TODO: Test implementation of `into_model` for `DecodePsbt`.
    //
    // let res: Result<DecodePsbt, _> = json.into_model();
    // let _ = res.expect("DecodePsbt into model");
}

#[test]
fn decode_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let (_, txid) = node.create_mempool_transaction();

    let tx = node.client.get_raw_transaction(txid).expect("getrawtransaction").transaction().expect("GetRawTransaction into model");
    let json = node.client.decode_raw_transaction(&tx).expect("decoderawtransaction");
    let model: Result<model::DecodeRawTransaction, RawTransactionError> = json.into_model();
    model.expect("DecodeRawTransaction into model");
}

#[test]
// FIXME: Seems the returned fields are  different depending on the script. Needs more thorough testing.
fn decode_script() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let p2pkh = arbitrary_p2pkh_script();
    let multi = arbitrary_multisig_script();

    for script in &[p2pkh, multi] {
        let hex = script.to_hex_string();

        let json: DecodeScript = node.client.decode_script(&hex).expect("decodescript");
        let model: Result<model::DecodeScript, DecodeScriptError> = json.into_model();
        let _ = model.expect("DecodeScript into model");
    }
}

// Script builder code copied from rust-bitcoin script unit tests.
fn arbitrary_p2pkh_script() -> ScriptBuf {
    let pubkey_hash = <[u8; 20]>::from_hex("16e1ae70ff0fa102905d4af297f6912bda6cce19").unwrap();

    script::Builder::new()
        .push_opcode(OP_DUP)
        .push_opcode(OP_HASH160)
        .push_slice(&pubkey_hash)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_CHECKSIG)
        .into_script()
}

fn arbitrary_multisig_script() -> ScriptBuf {
    let pk1 = <[u8; 33]>::from_hex("022afc20bf379bc96a2f4e9e63ffceb8652b2b6a097f63fbee6ecec2a49a48010e").unwrap();
    let pk2 = <[u8; 33]>::from_hex("03a767c7221e9f15f870f1ad9311f5ab937d79fcaeee15bb2c722bca515581b4c0").unwrap();

    script::Builder::new()
        .push_opcode(OP_PUSHNUM_1)
        .push_opcode(OP_PUSHBYTES_33)
        .push_slice(&pk1)
        .push_opcode(OP_PUSHBYTES_33)
        .push_slice(&pk2)
        .push_opcode(OP_PUSHNUM_2)
        .push_opcode(OP_CHECKMULTISIG)
        .into_script()
}

#[test]
#[cfg(feature = "TODO")]
fn finalize_psbt() {
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

    let change_address = node.client.get_raw_change_address().expect("getrawchangeaddress").into_model().expect("GetRawChangeAddress into model").0.assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreatePsbt = node.client.create_psbt(&inputs, &outputs).expect("createpsbt");
    let res: Result<model::CreatePsbt, _> = json.clone().into_model();
    let psbt = res.expect("CreatePsbt into model");
    let psbt = psbt.0;

    let json: DumpPrivKey = node.client.dump_priv_key(&addr).expect("dumpprivkey");
    let model: model::DumpPrivKey = json.into_model().expect("DumpPrivKey");
    let key = model.0;

    let json: SignRawTransaction = node.client.sign_raw_transaction_with_key(&psbt.unsigned_tx, &[key]).expect("signrawtransactionwithkey");
    let res: Result<model::SignRawTransaction, SignRawTransactionError> = json.into_model();
    let model = res.expect("SignRawTransaction into model");

    // FIXME: Core errors here with: code: -22, message: "TX decode failed"
    let json: ConvertToPsbt = node.client.convert_to_psbt(&model.tx).expect("converttopsbt");
    let model: Result<model::ConvertToPsbt, _> = json.into_model();
    let psbt = model.expect("ConvertToPsbt into model").0;

    let json: FinalizePsbt = node.client.finalize_psbt(&psbt).expect("finalizepsbt");
    let model: Result<model::FinalizePsbt, _> = json.into_model();
    let _ = model.expect("FinalizePsbt into model");
}

#[test]
#[cfg(feature = "v17",)]        // FIXME: Why does this not work for v18 onwards?
fn fund_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    raw_transaction_fund_sign_send(&node);
}

#[test]
fn get_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    // Get raw transaction using a mined transaction.
    let (_, tx) = node.create_mined_transaction();
    let json: GetRawTransaction = node.client.get_raw_transaction(tx.compute_txid()).expect("getrawtransaction");
    let model: Result<model::GetRawTransaction, _> = json.into_model();
    model.expect("GetRawTransaction into model");

    // Get raw transaction using an un-mined transaction.
    let (_, txid) = node.create_mempool_transaction();
    let _ = node.client.get_raw_transaction(txid).expect("getrawtransaction").into_model().expect("GetRawTransaction into model");
}

#[test]
fn get_raw_transaction_verbose() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    // Get raw transaction using a mined transaction.
    let (_, tx) = node.create_mined_transaction();
    let json = node.client.get_raw_transaction_verbose(tx.compute_txid()).expect("getrawtransaction verbose");
    let model: Result<model::GetRawTransactionVerbose, GetRawTransactionVerboseError> = json.into_model();
    model.expect("GetRawTransactionVerbose into model");

    // Get raw transaction using an un-mined transaction.
    let (_, txid) = node.create_mempool_transaction();
    let _ = node.client.get_raw_transaction_verbose(txid).expect("getrawtransaction verbose").into_model().expect("GetRawTransactionVerbose into model");
}

#[test]
fn send_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    raw_transaction_create_sign_send(&node);
}

#[test]
fn sign_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    raw_transaction_create_sign_send(&node);
}

#[test]
#[cfg(any(
    feature = "v17",
    feature = "v18",
    feature = "v19",
    feature = "v20",
    feature = "v21",
    feature = "v22",
))]                             // In v23 dumpprivkey no longer works.
fn sign_raw_transaction_with_key() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();
    raw_transaction_create_sign_with_key_send(&node);
}

#[test]
#[cfg(feature = "TODO")]
fn test_mempool_accept() {}

// FIXME: Doesn't work for v26 for some reason.
#[test]
#[cfg(feature = "v27")]
fn submitpackage() {
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
    for (_, tx_result) in &res.tx_results {
        assert!(tx_result.error.is_some());
    }
    assert!(res.replaced_transactions.is_empty());
}

// In Core v28 submitpackage has additional optional featurse.
#[test]
#[cfg(feature = "v28")]
fn submitpackage() {
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
    for (_, tx_result) in &res.tx_results {
        assert!(tx_result.error.is_some());
    }
    assert!(res.replaced_transactions.is_empty());
}

// Manipulates raw transactions.
//
// Calls the following RPC methods:
// - create_raw_transaction
// - sign_raw_transaction_with_wallet
// - send_raw_transaction
fn raw_transaction_create_sign_send(node: &Node) {
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

    let change_address = node.client.get_raw_change_address().expect("getrawchangeaddress").into_model().expect("GetRawChangeAddress into model").0.assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreateRawTransaction = node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let res: Result<model::CreateRawTransaction, _> = json.clone().into_model();
    let _ = res.expect("CreateRawTransaction into model");
    let tx = json.transaction().unwrap();

    let json: SignRawTransaction = node
        .client
        .sign_raw_transaction_with_wallet(&tx)
        .expect("signrawtransactionwithwallet");

    let res: Result<model::SignRawTransaction, SignRawTransactionError> = json.into_model();
    let model = res.expect("SignRawTransactionWithWallet into model");

    // The proves we did everything correctly.
    let json: SendRawTransaction = node.client.send_raw_transaction(&model.tx).expect("sendrawtransaction");
    let res: Result<model::SendRawTransaction, _> = json.into_model();
    let _ = res.expect("SendRawTransaction into model");
}

// Manipulates raw transactions.
//
// Calls the following RPC methods:
// - create_raw_transaction
// - sign_raw_transaction_with_key (sign_raw_transaction was deprecated in v17).
// - send_raw_transaction
//
// TODO: Work out how to get a private key without using `dumpprivkey`.
#[cfg(any(
    feature = "v17",
    feature = "v18",
    feature = "v19",
    feature = "v20",
    feature = "v21",
    feature = "v22",
))]                             // In v23 dumpprivkey no longer works.
fn raw_transaction_create_sign_with_key_send(node: &Node) {
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

    let change_address = node.client.get_raw_change_address().expect("getrawchangeaddress").into_model().expect("GetRawChangeAddress into model").0.assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreateRawTransaction = node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let res: Result<model::CreateRawTransaction, _> = json.clone().into_model();
    let _ = res.expect("CreateRawTransaction into model");
    let tx = json.transaction().unwrap();

    let json: DumpPrivKey = node.client.dump_priv_key(&addr).expect("dumpprivkey");
    let model: model::DumpPrivKey = json.into_model().expect("DumpPrivKey");
    let key = model.0;

    let json: SignRawTransaction = node.client.sign_raw_transaction_with_key(&tx, &[key]).expect("signrawtransactionwithkey");
    let res: Result<model::SignRawTransaction, SignRawTransactionError> = json.into_model();
    let model = res.expect("SignRawTransaction into model");

    // The proves we did everything correctly.
    let json: SendRawTransaction = node.client.send_raw_transaction(&model.tx).expect("sendrawtransaction");
    let res: Result<model::SendRawTransaction, _> = json.into_model();
    let _ = res.expect("SendRawTransaction into model");
}

// Manipulates raw transactions.
//
// Calls the following RPC methods:
// - fund_raw_transaction
// - sign_raw_transaction_with_wallet (sign_raw_transaction was deprecated in v17).
// - send_raw_transaction
#[cfg(feature = "v17",)]        // FIXME: Why does this not work for v18 onwards?
fn raw_transaction_fund_sign_send(node: &Node) {
    let (_addr, _tx, _txid, tx_out, _vout) = create_utxo(node);

    // Assumes tx_out has a million sats in it.
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    let change_amount = tx_out.value - spend_amount - fee;

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to get new address");
    let change_address = node
        .client
        .get_raw_change_address()
        .expect("getrawchangeaddress")
        .into_model()
        .expect("GetRawChangeAddress into model")
        .0
        .assume_checked();

    let spend = TxOut { value: spend_amount, script_pubkey: spend_address.script_pubkey() };
    let change = TxOut { value: change_amount, script_pubkey: change_address.script_pubkey() };

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![],
        output: vec![spend, change],
    };

    let json: FundRawTransaction = node.client.fund_raw_transaction(&tx).expect("fundrawtransaction");
    let res: Result<model::FundRawTransaction, FundRawTransactionError> = json.clone().into_model();
    let _ = res.expect("FundRawTransaction into model");
    let funded = json.transaction().unwrap();

    let json: SignRawTransaction = node
        .client
        .sign_raw_transaction_with_wallet(&funded)
        .expect("signrawtransactionwithwallet");

    let res: Result<model::SignRawTransaction, SignRawTransactionError> = json.into_model();
    let model = res.expect("SignRawTransactionWithWallet into model");

    // The proves we did everything correctly.
    let _ = node.client.send_raw_transaction(&model.tx).expect("createrawtransaction");
}

// Creates a transaction using client to do RPC call `create_raw_transaction`.
fn create_a_raw_transaction(node: &Node) -> Transaction {
    let (_addr, _tx, txid, tx_out, vout) = create_utxo(&node);

    // Assumes tx_out has a million sats in it.
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    let change_amount = tx_out.value - spend_amount - fee;

    let inputs = vec![Input { txid, vout, sequence: None }];

    let mut outputs = vec![];

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to create new address");
    outputs.push(Output::new(spend_address, spend_amount));

    let change_address = node.client.get_raw_change_address().expect("getrawchangeaddress").into_model().expect("GetRawChangeAddress into model").0.assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreateRawTransaction = node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let res: Result<model::CreateRawTransaction, _> = json.clone().into_model();
    let _ = res.expect("CreateRawTransaction into model");
    json.transaction().unwrap()
}

// Sends a transaction, mines a block then grabs a million sat UTXO from the mined transaction.
fn create_utxo(node: &Node) -> (bitcoin::Address, bitcoin::Transaction, bitcoin::Txid, bitcoin::TxOut, u64){
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

    let change_address = node.client.get_raw_change_address().expect("getrawchangeaddress").into_model().expect("GetRawChangeAddress into model").0.assume_checked();
    outputs.push(Output::new(change_address, change_amount));

    let json: CreatePsbt = node.client.create_psbt(&inputs, &outputs).expect("createpsbt");
    let res: Result<model::CreatePsbt, _> = json.clone().into_model();
    let psbt = res.expect("CreatePsbt into model");
    psbt.0
}
