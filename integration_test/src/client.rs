// SPDX-License-Identifier: CC0-1.0

//! A single dual client for the integration tests.
//!
//! [`TestClient`] is a concrete client the tests call directly. Which backend it is, is chosen at
//! compile time: without `test-async` it wraps the sync client; with `test-async` it drives the
//! async production client on a private current-thread runtime. A test gets one from [`test_client`]
//! / [`test_client_for_wallet`] and calls its methods, so the *same* test body runs against both.
//!
//! Each method returns the version-nonspecific model type and runs its own `into_model`, so a test
//! exercises the curated conversion normally and the generated one under `test-async`. The two
//! backends share nothing, so a bug in one cannot mask the other (the whole point of running both).
//!
//! The method set is intentionally **partial** and grows as tests migrate: add the calls a test
//! needs to both `impl` blocks below, and the test body carries no client scaffolding.

use bitcoin::{Address, BlockHash};
use bitcoind::{mtype, AddressType};

use crate::BitcoinD;

/// A [`TestClient`] for `node`'s base RPC endpoint (node-level RPCs, or a node with no wallet).
pub fn test_client(node: &BitcoinD) -> TestClient<'_> { TestClient::new(node) }

/// A [`TestClient`] for `node`'s `wallet`-scoped RPC endpoint (wallet RPCs).
pub fn test_client_for_wallet<'a>(node: &'a BitcoinD, wallet: &str) -> TestClient<'a> {
    TestClient::for_wallet(node, wallet)
}

// ---------------------------------------------------------------------------------------------
// Sync backend: wraps the sync client. The wallet argument is ignored because the harness has
// already pointed `node.client` at the right (wallet-scoped or base) endpoint.
// ---------------------------------------------------------------------------------------------

/// The dual test client (sync client, the default).
#[cfg(not(feature = "test-async"))]
pub struct TestClient<'a>(&'a bitcoind::Client);

#[cfg(not(feature = "test-async"))]
impl<'a> TestClient<'a> {
    fn new(node: &'a BitcoinD) -> Self { TestClient(&node.client) }
    fn for_wallet(node: &'a BitcoinD, _wallet: &str) -> Self { TestClient(&node.client) }

    // == Wallet ==
    pub fn new_address_with_label(&self, label: &str) -> Address {
        self.0.new_address_with_label(label).unwrap().assume_checked()
    }
    pub fn new_address_with_type(&self, ty: AddressType) -> Address {
        self.0.new_address_with_type(ty).unwrap()
    }
    pub fn get_address_info(&self, address: &Address) -> mtype::GetAddressInfo {
        self.0.get_address_info(address).unwrap().into_model().unwrap()
    }

    // == Blockchain ==
    pub fn best_block_hash(&self) -> BlockHash { self.0.best_block_hash().unwrap() }
    pub fn get_block_verbose_zero(&self, hash: BlockHash) -> mtype::GetBlockVerboseZero {
        self.0.get_block_verbose_zero(hash).unwrap().into_model().unwrap()
    }
    pub fn get_block_verbose_one(&self, hash: BlockHash) -> mtype::GetBlockVerboseOne {
        self.0.get_block_verbose_one(hash).unwrap().into_model().unwrap()
    }
    #[cfg(not(feature = "v28_and_below"))]
    pub fn get_block_verbose_two(&self, hash: BlockHash) -> mtype::GetBlockVerboseTwo {
        self.0.get_block_verbose_two(hash).unwrap().into_model().unwrap()
    }
    #[cfg(not(feature = "v28_and_below"))]
    pub fn get_block_verbose_three(&self, hash: BlockHash) -> mtype::GetBlockVerboseThree {
        self.0.get_block_verbose_three(hash).unwrap().into_model().unwrap()
    }
}

// ---------------------------------------------------------------------------------------------
// Async backend: a current-thread runtime + the async production client, blocking each call so the
// methods stay synchronous. Uses the async client's own generated wrappers and `into_model`.
// ---------------------------------------------------------------------------------------------

/// The dual test client (async production client, under `test-async`).
#[cfg(feature = "test-async")]
pub struct TestClient<'a> {
    rt: tokio::runtime::Runtime,
    client: bitcoind::client_async::Client,
    // Built from a `&BitcoinD` like the sync backend so the constructors share a signature; the
    // async client owns its connection, so nothing is actually borrowed.
    _node: std::marker::PhantomData<&'a BitcoinD>,
}

#[cfg(feature = "test-async")]
impl<'a> TestClient<'a> {
    fn new(node: &'a BitcoinD) -> Self { Self::at(node, node.rpc_url()) }
    fn for_wallet(node: &'a BitcoinD, wallet: &str) -> Self {
        Self::at(node, node.rpc_url_with_wallet(wallet))
    }
    fn at(node: &'a BitcoinD, url: String) -> Self {
        use bitcoind::client_async::{Auth, Client};
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("build current-thread runtime");
        let client = Client::builder()
            .url(&url)
            .expect("valid rpc url")
            .auth(Auth::CookieFile(node.params.cookie_file.clone()))
            // Match the sync client's transport timeout; the suite runs many nodes in parallel.
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .expect("build async client");
        TestClient { rt, client, _node: std::marker::PhantomData }
    }

    /// Block on an async client call and unwrap it (the RPC error is printed on failure).
    fn run<T, E: std::fmt::Debug>(&self, fut: impl std::future::Future<Output = Result<T, E>>) -> T {
        self.rt.block_on(fut).unwrap()
    }

    fn new_address(&self, label: &str, ty: Option<AddressType>) -> Address {
        use bitcoind::client_async::v31::wallet::GetNewAddressOptions;
        let opts = GetNewAddressOptions {
            label: Some(label.to_owned()),
            address_type: ty.map(|t| t.to_string()), // `AddressType: Display` => the RPC string.
        };
        self.run(self.client.get_new_address_with(opts)).into_model().unwrap().0.assume_checked()
    }

    // == Wallet ==
    pub fn new_address_with_label(&self, label: &str) -> Address { self.new_address(label, None) }
    pub fn new_address_with_type(&self, ty: AddressType) -> Address {
        self.new_address("", Some(ty))
    }
    pub fn get_address_info(&self, address: &Address) -> mtype::GetAddressInfo {
        self.run(self.client.get_address_info(address.to_string())).into_model().unwrap()
    }

    // == Blockchain ==
    pub fn best_block_hash(&self) -> BlockHash {
        self.run(self.client.get_best_block_hash()).into_model().unwrap().0
    }
    pub fn get_block_verbose_zero(&self, hash: BlockHash) -> mtype::GetBlockVerboseZero {
        self.run(self.client.get_block_verbose_0(hash.to_string())).into_model().unwrap()
    }
    pub fn get_block_verbose_one(&self, hash: BlockHash) -> mtype::GetBlockVerboseOne {
        self.run(self.client.get_block_verbose_1(hash.to_string())).into_model().unwrap()
    }
    // verbose=2/3 go through the generated tx reconstruct, which is broken on the generated path: it
    // rebuilds the inner tx from an empty `extra` and can't find `hash` (corepc_bugs_backlog.md #7).
    // Tests gate these to the sync backend until it is fixed, so these are present but uncalled here.
    #[cfg(not(feature = "v28_and_below"))]
    pub fn get_block_verbose_two(&self, hash: BlockHash) -> mtype::GetBlockVerboseTwo {
        self.run(self.client.get_block_verbose_2(hash.to_string())).into_model().unwrap()
    }
    #[cfg(not(feature = "v28_and_below"))]
    pub fn get_block_verbose_three(&self, hash: BlockHash) -> mtype::GetBlockVerboseThree {
        self.run(self.client.get_block_verbose_3(hash.to_string())).into_model().unwrap()
    }
}
