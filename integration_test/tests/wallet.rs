// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Wallet ==` section of the API docs.

#![cfg(any(feature = "0_17_1", feature = "0_18_1"))]

#[cfg(feature = "TODO")]
use bitcoin::address::{Address, NetworkChecked};
use bitcoin::Amount;
use integration_test::{Node, NodeExt as _};
use node::AddressType;

#[test]
#[cfg(feature = "TODO")]
pub fn add_multisig_address() {
    let nrequired = 1;  // 1-of-2 multisig.

    let add1: Address<NetworkChecked> = "32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf".parse::<Address<_>>().unwrap().assume_checked();
    let add2: Address<NetworkChecked> = "132F25rTsvBdp9JzLLBHP5mvGY66i1xdiM".parse::<Address<_>>().unwrap().assume_checked();

    let node = Node::new_with_default_wallet();
    let json = node.client.add_multisig_address_with_addresses(nrequired, vec![add1, add2]).expect("addmultisigaddress");
    assert!(json.into_model().is_ok());
}

#[test]
pub fn bump_fee() {
    let node = Node::new_with_default_wallet();
    let address = node.client.new_address().expect("failed to create new address");
    let _ = node.client.generate_to_address(101, &address).expect("generatetoaddress");

    let txid = node
        .client
        .send_to_address_rbf(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json = node.client.bump_fee(txid).expect("bumpfee");
    assert!(json.into_model().is_ok());
}

#[test]
pub fn create_wallet() {
    // Implicitly tests `createwallet` because we create the default wallet.
    let _ = Node::new_with_default_wallet();
}

#[test]
pub fn dump_priv_key() {
    let node = Node::new_with_default_wallet();
    let address = node.client.new_address().expect("failed to create new address");
    let json = node.client.dump_priv_key(&address).expect("dumpprivkey");
    assert!(json.into_model().is_ok());
}

#[test]
pub fn dump_wallet() {
    let node = Node::new_with_default_wallet();
    let out = integration_test::random_tmp_file();
    let json = node.client.dump_wallet(&out).expect("dumpwallet");
    let _ = json.into_model();
}

#[test]
pub fn get_addresses_by_label() {
    let node = Node::new_with_default_wallet();
    let label = "some-label";
    let addr = node.client.new_address_with_label(label).expect("failed to get new address");
    let json = node.client.get_addresses_by_label(label).expect("getaddressesbylabel");
    let map = json.into_model().expect("failed to convert to model").0;
    assert!(!map.is_empty());
    assert!(map.get(&addr).is_some());
}

#[test]
#[cfg(feature = "TODO")]
// TODO: We are getting an error for missing `pubkeys` field but the field is optional?
//
// Also consider adding tests for different address types.
pub fn get_address_info() {
    let node = Node::new_with_default_wallet();
    let address = node.client.new_address().expect("failed to create new address");
    let json = node.client.get_address_info(&address).expect("getaddressinfo for new address");
    assert!(json.into_model().is_ok());

    // node.fund_wallet();
    // let _ = node
    //     .client
    //     .send_to_address(&address, Amount::from_sat(10_000))
    //     .expect("sendtoaddress");
    // node.mine_a_block();

    // let json = node.client.get_address_info(&address).expect("getaddressinfo");
    // assert!(json.into_model().is_ok());
}

#[test]
fn get_balance() {
    let node = Node::new_with_default_wallet();
    let json = node.client.get_balance().expect("getbalance");
    assert!(json.into_model().is_ok())
}

#[test]
#[cfg(feature = "v19")]
fn get_balances() {
    let node = Node::new_with_default_wallet();
    let address = node.client.new_address().expect("failed to get new address");
    let _ = node.client.generate_to_address(101, &address).expect("generatetoaddress");
    let json = node.client.get_balances().expect("getbalances");
    let model = json.into_model().expect("into_model");
    // TODO: Do more fine grained testing.
    assert!(model.mine.trusted > Amount::ZERO); 
}

#[test]
fn get_new_address() {


    let node = Node::new_with_default_wallet();

    let _ = node.client.new_address().expect("getnewaddress");

    // Test the helper as well just for good measure.
    let _ = node.client.new_address().unwrap();

    // Exhaustively test address types with helper.
    let _ = node
        .client
        .new_address_with_type(AddressType::Legacy)
        .unwrap();
    let _ = node
        .client
        .new_address_with_type(AddressType::P2shSegwit)
        .unwrap();
    let _ = node
        .client
        .new_address_with_type(AddressType::Bech32)
        .unwrap();
}

#[test]
fn get_raw_change_address() {
    let node = Node::new_with_default_wallet();
    let json = node.client.get_raw_change_address().expect("getrawchangeaddress");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_received_by_address() {
    let amount = Amount::from_sat(10_000);

    let node = Node::new_with_default_wallet();
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let _txid = node
        .client
        .send_to_address(&address, amount)
        .expect("sendtoaddress")
        .txid()
        .unwrap();
    node.mine_a_block();

    let json = node.client.get_received_by_address(&address).expect("getreceivedbyaddress");
    let model = json.into_model().expect("into_model failed");
    assert_eq!(model.0, amount);
}

#[test]
fn get_transaction() {
    let node = Node::new_with_default_wallet();
    let address = node.client.new_address().expect("failed to create new address");
    let _ = node.client.generate_to_address(101, &address).expect("generatetoaddress");

    let txid = node
        .client
        .send_to_address(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json = node.client.get_transaction(txid).expect("gettransaction");
    assert!(json.into_model().is_ok());
}

#[test]
fn load_wallet() {
    // Implicitly test loadwalled because we load the default wallet.
    let _ = Node::new_with_default_wallet();
}

#[test]
#[cfg(feature = "TODO")]
fn unload_wallet() {
    let node = Node::new_no_wallet();
    let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
    node.client.create_wallet(&wallet).expect("failed to create wallet");
    // let json = node.client.unload_wallet(&wallet).expect("unloadwallet");
    // assert!(json.into_model().is_ok())
    todo!()
}

#[test]
fn send_to_address() {
    let node = Node::new_with_default_wallet();
    let address = node.client.new_address().expect("failed to create new address");
    let _ = node.client.generate_to_address(101, &address).expect("generatetoaddress");

    let json = node
        .client
        .send_to_address(&address, Amount::from_sat(10_000))
        .expect("sendtddress");
    assert!(json.into_model().is_ok());
}
