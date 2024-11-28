//! Provides a macro that implements the tests.

use std::path::PathBuf;

use rand::distributions::Alphanumeric;
use rand::Rng;

#[rustfmt::skip]    // Keep public re-exports separate.
pub use node::Node; // Re-export this to make test imports more terse.

/// Number of blocks to generate when funding wallet.
const NBLOCKS: usize = 101;

/// Initialize a logger (configure with `RUST_LOG=trace cargo test`).
#[allow(dead_code)] // Not all tests use this function.
pub fn init_logger() { let _ = env_logger::try_init(); }

pub trait NodeExt {
    /// Returns a handle to a `bitcoind` instance after leading wallet if present.
    fn new(conf: node::Conf, wallet: Option<String>) -> Node;

    /// Returns a handle to a `bitcoind` instance with "default" wallet loaded.
    fn new_with_default_wallet() -> Node {
        let conf = node::Conf::default();
        Self::new(conf, None)
    }

    /// Returns a handle to a `bitcoind` instance with `wallet` loaded.
    fn new_with_wallet(wallet: String) -> Node {
        let conf = node::Conf::default();
        Self::new(conf, Some(wallet))
    }

    /// Returns a handle to a `bitcoind` instance without any wallet loaded.
    fn new_no_wallet() -> Node {
        let mut conf = node::Conf::default();
        conf.wallet = None;
        Self::new(conf, None)
    }

    /// Returns a handle to a `bitcoind` instance without any wallet loaded and `-txindex` enabled.
    fn new_no_wallet_txindex() -> Node {
        let mut conf = node::Conf::default();
        conf.args.push("-txindex");
        Self::new(conf, None)
    }

    /// Generates [`NBLOCKS`] to an address controlled by the loaded wallet.
    fn fund_wallet(&self);

    /// Mine a block.
    ///
    /// Should send mining reward to a new address for the loaded wallet.
    fn mine_a_block(&self);

    /// Create a transaction and mine it.
    ///
    /// # Returns
    ///
    /// The receive address and the transaction.
    fn create_mined_transaction(&self) -> (bitcoin::Address, bitcoin::Transaction);
}

impl NodeExt for Node {
    fn new(mut conf: node::Conf, wallet: Option<String>) -> Node {
        let exe = node::exe_path().expect("failed to get bitcoind executable");

        if let Some(wallet) = wallet {
            conf.wallet = Some(wallet);
        }

        Node::with_conf(exe, &conf).expect("failed to create node")
    }

    fn fund_wallet(&self) {
        let address = self.client.new_address().expect("failed to get new address");
        self.client.generate_to_address(NBLOCKS, &address).expect("failed to generate to address");
    }

    fn create_mined_transaction(&self) -> (bitcoin::Address, bitcoin::Transaction) {
        const MILLION_SATS: bitcoin::Amount = bitcoin::Amount::from_sat(1000000);

        let address = self.client.new_address().expect("failed to get new address");

        let _ = self.client.send_to_address(&address, MILLION_SATS);
        self.mine_a_block();

        let best_block_hash = self.client.best_block_hash().expect("best_block_hash");
        let best_block = self.client.get_block(best_block_hash).expect("best_block");
        let tx = best_block.txdata[1].clone();

        (address, tx)
    }

    fn mine_a_block(&self) {
        // TODO: Consider returning the error.
        let address = self.client.new_address().expect("failed to get new address");
        self.client.generate_to_address(1, &address).expect("failed to generate to address");
    }
}

/// Return a temporary file path.
pub fn random_tmp_file() -> PathBuf {
    let file: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let mut tmp = std::env::temp_dir();
    tmp.push(&file);
    tmp
}
