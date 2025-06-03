use bitcoin::Amount;
use bitcoin_rpc_midas::test_node::test_node::BitcoinTestClient;
use node::anyhow::{anyhow, Result};
use node::serde_json::json;

#[tokio::test]
async fn midas_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Starting Midas Test ===");

    let mut client = BitcoinTestClient::new().await?;
    let info = client.getblockchaininfo().await?;
    println!("Initial blockchain state:\n{:#?}\n", info);

    let _ = client.mine_blocks(101, 2000).await?;
    let balance = client.getbalance("*".to_string(), 0, false, false).await?;
    println!("Initial wallet balance: {:.8} BTC\n", balance);

    // Generate a new address to send to
    let address = client.getnewaddress("".to_string(), "bech32m".to_string()).await?;
    let address = address.as_str().ok_or(anyhow!("Expected address to be string"))?.to_string();
    println!("Generated P2TR address: {}\n", address);

    let amount = Amount::from_sat(5500);
    println!("Preparing to send: {} satoshis\n", amount.to_sat());

    // Create and fund PSBT
    let outputs = json!({ address: amount.to_btc() });
    let psbt_obj = client
        .walletcreatefundedpsbt(
            vec![],        // Let Core auto-select UTXOs
            vec![outputs], // Outputs
            0,             // locktime as u64
            json!({
                "feeRate": 0.0001,
                "changePosition": 1,
                "includeWatching": true,
                "lockUnspents": true,
                "replaceable": false
            }),
            false, // bip32derivs as bool
        )
        .await?;

    let psbt = psbt_obj
        .get("psbt")
        .and_then(|v| v.as_str())
        .ok_or(anyhow!("Missing psbt field"))?
        .to_string();
    println!("Created PSBT: {}\n", psbt);

    // Sign the PSBT
    let signed_obj = client
        .walletprocesspsbt(
            psbt.clone(),
            true,              // sign
            "ALL".to_string(), // sighashtype
            true,              // bip32derivs
            false,             // finalize
        )
        .await?;

    let final_psbt = signed_obj
        .get("psbt")
        .and_then(|v| v.as_str())
        .ok_or(anyhow!("Missing psbt in signed object"))?;

    // Finalize and broadcast
    let finalized_obj = client.finalizepsbt(final_psbt.to_string(), true).await?;
    let hex = finalized_obj
        .get("hex")
        .and_then(|v| v.as_str())
        .ok_or(anyhow!("Missing hex from finalizepsbt"))?;
    println!("Finalized transaction: {}\n", hex);

    let txid = client
        .sendrawtransaction(hex.to_string(), Amount::from_sat(0), Amount::from_sat(0))
        .await?;
    println!("Broadcasted transaction! TXID: {}\n", txid);

    // Mine a block to confirm the transaction
    let (_, _) = client.mine_blocks(1, 2000).await?;
    println!("Block mined to confirm transaction");

    // Check final balance
    let final_balance = client.getbalance("*".to_string(), 0, false, false).await?;
    println!("Final wallet balance: {:.8} BTC", final_balance);
    println!(
        "Balance change: {:.8} BTC",
        final_balance.as_f64().unwrap() - balance.as_f64().unwrap()
    );
    println!("Expected change: {:.8} BTC", -amount.to_btc());

    println!("\n=== Sanity Test Completed Successfully ===\n");
    Ok(())
}
