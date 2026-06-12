# Rename map (generated vs legacy corepc)

Flat lookup of every name that differs between the generated output and the hand-written
`corepc` code. Two tables: types (`generated -> corepc-types`) and client methods
(`sync -> async`). Source of the rules is `NAMING.md`; this file is just the diff.

Anything not listed here matches byte-for-byte on both sides.

## Types: generated -> hand-written (corepc-types)

| generated                                            | hand-written                               |
| ---------------------------------------------------- | ------------------------------------------ |
| `GetMemoryInfo` (enum)                               | `GetMemoryInfoStats`                       |
| `GetRawTransactionVerboseZero`                       | `GetRawTransaction`                        |
| `GetRawTransactionVerboseOne`                        | `GetRawTransactionVerbose`                 |
| `GetRawTransactionVerboseTwo`                        | `GetRawTransactionVerboseWithPrevout`      |
| `GetBlockHeaderVerboseZero`                          | `GetBlockHeader`                           |
| `GetBlockHeaderVerboseOne`                           | `GetBlockHeaderVerbose`                    |
| `GetMempoolAncestorsVerboseZero`                     | `GetMempoolAncestors`                      |
| `GetMempoolAncestorsVerboseOne`                      | `GetMempoolAncestorsVerbose`               |
| `GetMempoolDescendantsVerboseZero`                   | `GetMempoolDescendants`                    |
| `GetMempoolDescendantsVerboseOne`                    | `GetMempoolDescendantsVerbose`             |
| `GetBlockVerboseTwoTxItem`                           | `GetBlockVerboseTwoTransaction`            |
| `GetBlockVerboseThreeTxItem`                         | `GetBlockVerboseThreeTransaction`          |
| `GetBlockVerboseThreeTxItemVinItemPrevout`           | `GetBlockVerboseThreePrevout`              |
| `GetTransactionDetailsItem`                          | `GetTransactionDetail`                     |
| `GetTxOutSetInfoBlockInfoUnspendables`               | `GetTxOutSetInfoUnspendables`              |
| `ScanBlocksV1` (arm of `ScanBlocks`)                 | `ScanBlocksStart`                          |
| `ScanBlocksV2` (arm of `ScanBlocks`)                 | `ScanBlocksStatus` / `ScanBlocksAbort`     |
| `ScanTxOutSetV0` (arm of `ScanTxOutSet`)             | `ScanTxOutSetStart`                        |
| `ScanTxOutSetV2` (arm of `ScanTxOutSet`)             | `ScanTxOutSetStatus` / `ScanTxOutSetAbort` |
| `ScanTxOutSetV0UnspentsItem`                         | `ScanTxOutSetUnspent`                      |
| `{Path}ScriptPubKey` (e.g. `GetTxOutV1ScriptPubKey`) | `ScriptPubKey`                             |
| `{Path}ScriptSig`                                    | `ScriptSig`                                |
| `{Path}WitnessUtxo`                                  | `WitnessUtxo`                              |
| `DecodePsbtInputsItem`                               | `PsbtInput`                                |
| `DecodePsbtOutputsItem`                              | `PsbtOutput`                               |
| `{Path}Bip32DerivsItem`                              | `Bip32Deriv`                               |
| `GetDeploymentInfoDeploymentsBip9`                   | `Bip9SoftforkInfo` / `Bip9Info`            |
| `GetRawMempoolV1Fees`                                | `MempoolEntryFees`                         |

Mechanical rule for the path-derived nested types: strip the parent-path prefix and the
`Item` / `V{n}` / `Verbose{Level}` decoration, then map the leaf role to corepc's shared
noun. Verbosity-0 maps to the bare corepc type; higher levels to corepc's `Verbose` /
`WithPrevout` word.

## Client methods: sync -> async

Cross-spelling (same RPC, fn name differs by spelling):

| wire RPC                 | sync                        | async                        |
| ------------------------ | --------------------------- | ---------------------------- |
| `signmessagewithprivkey` | `sign_message_with_privkey` | `sign_message_with_priv_key` |
| `getrawaddrman`          | `get_raw_addrman`           | `get_raw_addr_man`           |

Verbosity / multi-method families (both sides split the RPC, level naming differs):

| wire RPC                | sync                                                          | async                                      |
| ----------------------- | ------------------------------------------------------------- | ------------------------------------------ |
| `getblock`              | `get_block_verbose_zero`                                      | `get_block`                                |
| `getblockheader`        | `get_block_header_verbose`                                    | `get_block_header_verbose_one`             |
| `getrawtransaction`     | `get_raw_transaction_verbose`                                 | `get_raw_transaction_verbose_one` / `_two` |
| `getmempoolancestors`   | `get_mempool_ancestors_verbose`                               | `get_mempool_ancestors_verbose_one`        |
| `getmempooldescendants` | `get_mempool_descendants_verbose`                             | `get_mempool_descendants_verbose_one`      |
| `getrawmempool`         | `get_raw_mempool_verbose` / `get_raw_mempool_sequence`        | `get_raw_mempool_with`                     |
| `getblockstats`         | `get_block_stats_by_height` / `get_block_stats_by_block_hash` | `get_block_stats` / `get_block_stats_with` |
| `scanblocks`            | `scan_blocks_start` / `_status` / `_abort`                    | `scan_blocks` / `scan_blocks_with`         |
| `scantxoutset`          | `scan_tx_out_set_start` / `_status` / `_abort`                | `scan_tx_out_set` / `scan_tx_out_set_with` |
| `getdeploymentinfo`     | `get_deployment_info_tip`                                     | `get_deployment_info_with`                 |
| `deriveaddresses`       | `derive_addresses_multipath`                                  | `derive_addresses_with`                    |
| `estimatesmartfee`      | `estimate_smart_fee_with_mode`                                | `estimate_smart_fee_with`                  |
| `sendtoaddress`         | `send_to_address_rbf`                                         | `send_to_address_with`                     |

Sync convenience methods with no async twin at all (async exposes the base method plus its
`_with` form instead):

| sync method                                                        | over RPC           |
| ------------------------------------------------------------------ | ------------------ |
| `best_block_hash`                                                  | `getbestblockhash` |
| `new_address` / `new_address_with_label` / `new_address_with_type` | `getnewaddress`    |
| `server_version`                                                   | `getnetworkinfo`   |
| `create_legacy_wallet` / `create_wallet_external_signer`           | `createwallet`     |
| `send_many_verbose`                                                | `sendmany`         |
| `unlock_unspent`                                                   | `lockunspent`      |
| `get_orphan_txs` / `_verbosity_1` / `_verbosity_2`                 | `getorphantxs`     |

Sync methods binding RPCs the async v30 client never binds (different wire entirely):

| sync method                            | wire RPC                                |
| -------------------------------------- | --------------------------------------- |
| `add_connection`                       | `addconnection`                         |
| `add_peer_address`                     | `addpeeraddress`                        |
| `estimate_raw_fee`                     | `estimaterawfee`                        |
| `generate_block`                       | `generateblock`                         |
| `generate_to_address`                  | `generatetoaddress`                     |
| `generate_to_descriptor`               | `generatetodescriptor`                  |
| `get_raw_addrman`                      | `getrawaddrman`                         |
| `get_zmq_notifications`                | `getzmqnotifications`                   |
| `invalidate_block`                     | `invalidateblock`                       |
| `mock_scheduler`                       | `mockscheduler`                         |
| `reconsider_block`                     | `reconsiderblock`                       |
| `sign_raw_transaction`                 | `signrawtransaction` (deprecated alias) |
| `sync_with_validation_interface_queue` | `syncwithvalidationinterfacequeue`      |

Async methods binding RPCs the sync v30 client does not (new bindings):

| async method                          | wire RPC                |
| ------------------------------------- | ----------------------- |
| `descriptor_process_psbt` (+ `_with`) | `descriptorprocesspsbt` |
| `get_open_rpc_info`                   | `getopenrpcinfo`        |
| `set_label`                           | `setlabel`              |
