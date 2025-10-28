use anyhow::Result;

use midnight_base_crypto::hash::{HashOutput, PERSISTENT_HASH_BYTES};
use midnight_ledger::structure::{Transaction, TransactionHash};
use midnight_node_ledger_helpers::{
	NetworkId, PureGeneratorPedersen, SerdeTransaction, deserialize,
};
use midnight_storage::DefaultDB;

use serde_json::Value;

/// Deserialize all transactions in a block
pub async fn deserialize_transactions(
	block_value: Value,
) -> Result<
	Vec<(
		TransactionHash,
		SerdeTransaction<midnight_base_crypto::signatures::Signature, (), DefaultDB>,
	)>,
	anyhow::Error,
> {
	let raw_tx_data = block_value.get("transactions_index");

	let mut txs = Vec::new();

	if let Some(arr) = raw_tx_data.and_then(|v| v.as_array()) {
		for item in arr {
			if let Some([hash, raw_tx_data]) = item.as_array().and_then(|a| a.get(0..2)) {
				let (hash, tx) = parse_tx_index_item(
					// These strings have a prefix of "0x"
					hash.as_str().unwrap(),
					raw_tx_data.as_str().unwrap(),
					NetworkId::TestNet,
				)
				.await?;

				txs.push((hash, tx));
			}
		}
	}

	Ok(txs)
}

/// Parse a transaction index item
pub async fn parse_tx_index_item(
	hash_with_prefix: &str,
	raw_tx_data_with_prefix: &str,
	_network_id: NetworkId,
) -> Result<
	(
		TransactionHash,
		SerdeTransaction<midnight_base_crypto::signatures::Signature, (), DefaultDB>,
	),
	anyhow::Error,
> {
	// Remove the "0x" prefix from the hash and raw tx data
	let (_hex_prefix, hash_without_prefix) = hash_with_prefix.split_at(2);
	let (_hex_prefix, raw_tx_data_without_prefix) = raw_tx_data_with_prefix.split_at(2);

	let hash = hex::decode(hash_without_prefix)
		.map_err(|e| anyhow::anyhow!("TransactionHashDecodeError: {}", e))?;

	if hash.len() != PERSISTENT_HASH_BYTES {
		return Err(anyhow::anyhow!(
			"hash length ({}) != {PERSISTENT_HASH_BYTES}",
			hash.len()
		));
	}
	let hash = TransactionHash(HashOutput(
		hash.try_into()
			.map_err(|_| anyhow::anyhow!("Invalid hash length"))?,
	));

	let body = hex::decode(raw_tx_data_without_prefix)
		.map_err(|e| anyhow::anyhow!("TransactionBodyDecodeError: {}", e))?;

	// Deserialize as Option<Transaction> since that's what the RPC returns
	let tx_opt: Option<
		Transaction<
			midnight_base_crypto::signatures::Signature,
			(),
			PureGeneratorPedersen,
			DefaultDB,
		>,
	> = deserialize(&mut body.as_slice())
		.map_err(|e| anyhow::anyhow!("TransactionDeserializeError: {}", e))?;

	// Wrap in SerdeTransaction::Midnight if present
	let tx = match tx_opt {
		Some(tx) => SerdeTransaction::Midnight(tx),
		None => return Err(anyhow::anyhow!("Transaction data is None")),
	};

	Ok((hash, tx))
}
