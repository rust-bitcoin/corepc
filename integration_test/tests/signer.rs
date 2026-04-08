// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Signer ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.
#![allow(unused_imports)] // Because of feature gated tests.

use bitcoin::address::{NetworkUnchecked, ParseError};
use bitcoin::Address;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};
use bitcoind::vtype::*;
use bitcoind::{mtype, Input, Output}; // All the version specific types.

#[test]
#[cfg(unix)]
#[cfg(not(feature = "v21_and_below"))]
fn signer__enumerate_signers() {
    use std::os::unix::fs::PermissionsExt;

    let script_path = integration_test::random_tmp_file();
    // `script_body` is minimal JSON array expected by `enumeratesigners` RPC: an array
    // of signer objects with at least a fingerprint and name. Using a hard-coded
    // dummy signer (fingerprint "deadbeef").
    let script_body =
        "#!/bin/sh\necho '[{\"fingerprint\":\"deadbeef\",\"name\":\"TestSigner\"}]'\n";
    std::fs::write(&script_path, script_body).expect("write signer script");

    // Script needs to be executable so bitcoind can invoke it via the -signer arg.
    std::fs::set_permissions(&script_path, std::fs::Permissions::from_mode(0o755)).expect("chmod");

    let signer_arg = format!("-signer={}", script_path.to_str().unwrap());
    let node = BitcoinD::with_wallet(Wallet::None, &[&signer_arg]);
    let json: EnumerateSigners = node.client.enumerate_signers().expect("enumeratesigners");
    let first_tx = json.signers.first().expect("no signers found");

    assert_eq!(first_tx.fingerprint, "deadbeef");
}

#[test]
#[cfg(unix)]
#[cfg(not(feature = "v21_and_below"))]
fn signer__wallet_display_address__modelled() {
    use std::os::unix::fs::PermissionsExt;

    // MOCK address and xpub from Bitcoin Core's `wallet_signer.py`.
    let address = "bcrt1qm90ugl4d48jv8n6e5t9ln6t9zlpm5th68x4f8g";
    let xpub = "tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B";

    // Mock signer script that handles `enumerate`, `getdescriptors`, and
    // `displayaddress` sub-commands — the three commands bitcoind invokes when
    // creating an external-signer wallet and then displaying an address.
    let script_path = integration_test::random_tmp_file();
    let script_body = format!(
        r#"#!/bin/sh
CMD=""
while [ $# -gt 0 ]; do
    case "$1" in enumerate|getdescriptors|displayaddress) CMD="$1" ;; esac
    shift
done
case "$CMD" in
    enumerate)     echo '[{{"fingerprint":"00000001","type":"trezor","model":"trezor_t"}}]' ;;
    getdescriptors) echo '{{"receive":["wpkh([00000001/84h/1h/0h]{xpub}/0/*)"],"internal":[]}}' ;;
    displayaddress) echo '{{"address":"{address}"}}' ;;
    *)             echo '{{"error":"unknown command"}}'; exit 1 ;;
esac
"#
    );
    std::fs::write(&script_path, script_body).expect("write signer script");
    std::fs::set_permissions(&script_path, std::fs::Permissions::from_mode(0o755)).expect("chmod");

    let signer_arg = format!("-signer={}", script_path.to_str().unwrap());
    let node = BitcoinD::with_wallet(Wallet::None, &[&signer_arg]);

    let _: CreateWallet = node
        .client
        .create_wallet_external_signer("hww")
        .expect("createwallet with external signer");

    let json: WalletDisplayAddress =
        node.client.wallet_display_address(address).expect("walletdisplayaddress");

    let address: Address<NetworkUnchecked> = address.parse().unwrap();
    let model: Result<mtype::WalletDisplayAddress, ParseError> = json.into_model();
    let model = model.unwrap();
    assert_eq!(model.address, address);
}
