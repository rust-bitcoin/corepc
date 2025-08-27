// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Signer ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.
#![allow(unused_imports)] // Because of feature gated tests.

use integration_test::{Node, NodeExt as _, Wallet};
use node::{mtype, Input, Output};
use node::vtype::*;             // All the version specific types.

#[test]
#[cfg(not(feature = "v21_and_below"))]
fn signer__enumerate_signers() {
	let script_path = integration_test::random_tmp_file();
	let script_body = "#!/bin/sh\necho '[{\"fingerprint\":\"deadbeef\",\"name\":\"TestSigner\"}]'\n";
	std::fs::write(&script_path, script_body).expect("write signer script");
    
	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		std::fs::set_permissions(&script_path, std::fs::Permissions::from_mode(0o755))
			.expect("chmod");
	}

	let signer_arg = format!("-signer={}", script_path.to_str().unwrap());
	let node = Node::with_wallet(Wallet::None, &[&signer_arg]);
	let json: EnumerateSigners = node.client.enumerate_signers().expect("enumeratesigners");

    assert_eq!(json.signers[0].fingerprint, "deadbeef");
}
