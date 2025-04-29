//! Provides a macro that implements the tests.

use std::path::PathBuf;

use node::{Conf, P2P};
use rand::distributions::Alphanumeric;
use rand::Rng;

#[rustfmt::skip]    // Keep public re-exports separate.
pub use node::Node; // Re-export this to make test imports more terse.
pub use client::client_sync::{AddNodeCommand, SetBanCommand, LockUnspentOutput};

/// Initialize a logger (configure with `RUST_LOG=trace cargo test`).
#[allow(dead_code)] // Not all tests use this function.
pub fn init_logger() { let _ = env_logger::try_init(); }

/// Controls the loaded wallet.
#[derive(Debug, PartialEq, Eq)]
pub enum Wallet {
    /// Load the default wallet.
    Default,
    /// Load a wallet with custom name.
    Load(String),
    /// Do not load a wallet.
    None,
}

pub trait NodeExt {
    /// Returns a handle to a `bitcoind` instance after leading wallet if present.
    fn with_wallet(wallet: Wallet, args: &[&str]) -> Node;

    /// Generates 101 blocks to an address controlled by the loaded wallet.
    fn fund_wallet(&self);

    /// Mines a block.
    ///
    /// Should send mining reward to a new address for the loaded wallet.
    fn mine_a_block(&self);

    /// Creates a transaction in the mempool.
    ///
    /// # Returns
    ///
    /// The receive address and the transaction.
    fn create_mempool_transaction(&self) -> (bitcoin::Address, bitcoin::Txid);

    /// Creates a transaction and mines a block that includes it in the chain.
    ///
    /// # Returns
    ///
    /// The receive address and the transaction.
    fn create_mined_transaction(&self) -> (bitcoin::Address, bitcoin::Transaction);

    /// Returns the number of peers connected to this node.
    fn peers_connected(&self) -> usize;
}

impl NodeExt for Node {
    fn with_wallet(wallet: Wallet, args: &[&str]) -> Node {
        let exe = node::exe_path().expect("failed to get bitcoind executable");

        let mut conf = node::Conf::default();
        match wallet {
            Wallet::Default => {} // conf.wallet = Some("default")
            Wallet::Load(w) => conf.wallet = Some(w.to_owned()),
            Wallet::None => conf.wallet = None,
        }

        for arg in args {
            conf.args.push(arg);
        }

        Node::with_conf(exe, &conf).expect("failed to create node")
    }

    fn fund_wallet(&self) {
        let address = self.client.new_address().expect("failed to get new address");
        self.client.generate_to_address(101, &address).expect("failed to generate to address");
    }

    fn mine_a_block(&self) {
        let address = self.client.new_address().expect("failed to get new address");
        self.client.generate_to_address(1, &address).expect("failed to generate to address");
    }

    fn create_mempool_transaction(&self) -> (bitcoin::Address, bitcoin::Txid) {
        const MILLION_SATS: bitcoin::Amount = bitcoin::Amount::from_sat(1000000);

        let address = self.client.new_address().expect("failed to get new address");

        let txid = self
            .client
            .send_to_address(&address, MILLION_SATS)
            .expect("failed to send to address")
            .txid()
            .expect("failed to convert hex to txid");
        (address, txid)
    }

    fn create_mined_transaction(&self) -> (bitcoin::Address, bitcoin::Transaction) {
        let (address, _) = self.create_mempool_transaction();
        self.mine_a_block();

        let best_block_hash = self.client.best_block_hash().expect("best_block_hash");
        let best_block = self.client.get_block(best_block_hash).expect("best_block");
        let tx = best_block.txdata[1].clone();

        (address, tx)
    }

    fn peers_connected(&self) -> usize {
        let json = self.client.get_peer_info().expect("get_peer_info");
        json.0.len()
    }
}

/// Return a temporary file path.
pub fn random_tmp_file() -> PathBuf {
    let file: String =
        rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect();
    let mut tmp = std::env::temp_dir();
    tmp.push(&file);
    tmp
}

/// Creates a Bitcoin network with three connected nodes.
pub fn three_node_network() -> (Node, Node, Node) {
    let exe = node::exe_path().expect("failed to get bitcoind executable");

    // Create Node 1 and listen for p2p connections.
    let mut conf_node1 = Conf::default();
    conf_node1.p2p = P2P::Yes;
    let node1 = Node::with_conf(&exe, &conf_node1).unwrap();
    assert_eq!(node1.peers_connected(), 0);

    // Create Node 2 connected Node 1.
    let mut conf_node2 = Conf::default();
    conf_node2.p2p = node1.p2p_connect(true).unwrap();
    let node2 = Node::with_conf(&exe, &conf_node2).unwrap();
    assert_eq!(node2.peers_connected(), 1);

    // For some reason using only two nodes still errors.

    // Create Node 3 connected Node 2.
    let mut conf_node3 = Conf::default();
    conf_node3.p2p = node2.p2p_connect(true).unwrap();
    let node3 = Node::with_conf(&exe, &conf_node3).unwrap();
    assert!(node3.peers_connected() >= 1); // FIXME: Why not 2?

    (node1, node2, node3)
}
