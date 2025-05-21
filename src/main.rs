mod client;
mod parser;

use client::RpcClient;
use parser::deserialize_transactions;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	let node_url = "https://rpc.testnet-02.midnight.network";

	// Replace with the block number you want to test
	let block_number = 13915;
	let client = RpcClient::new(node_url);

	let block_value = client.get_block_at_number(block_number).await?;

	let deserialized_txs = deserialize_transactions(block_value).await?;

	// If we reach here, that's a good sign!
	println!("Deserialized txs: {:#?}", deserialized_txs);

	Ok(())
}
