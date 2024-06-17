//! Test the JSON-RPC API against `bitcoind v23.2`.

#![cfg(feature = "v23")]

use integration_test::*;

// == Blockchain ==
mod blockchain {
    use super::*;

    impl_test_v17__getblockchaininfo!();
    impl_test_v17__getbestblockhash!();
    impl_test_v22__getblock!();
}

// == Control ==
mod control {
    use super::*;

    impl_test_v17__stop!();
}

// == Generating ==
mod generating {
    use super::*;

    impl_test_v17__generatetoaddress!();
}

// == Network ==
mod network {
    use super::*;

    impl_test_v17__getnetworkinfo!();
}

// == Rawtransactions ==
mod raw_transactions {
    use super::*;

    impl_test_v17__sendrawtransaction!();
}

// == Wallet ==
mod wallet {
    use super::*;

    impl_test_v17__createwallet!();
    impl_test_v17__loadwallet!();

    impl_test_v17__getnewaddress!();
    impl_test_v17__getbalance!();
    impl_test_v19__getbalances!();
    impl_test_v17__sendtoaddress!();
    impl_test_v17__gettransaction!();
}