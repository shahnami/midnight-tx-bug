use anyhow::Result;
use midnight_ledger::{
	base_crypto::hash::{HashOutput, PERSISTENT_HASH_BYTES},
	serialize::{NetworkId, deserialize},
	storage::DefaultDB,
	structure::{Proof, Proofish, Transaction, TransactionHash},
};
use serde_json::Value;

/// Deserialize all transactions in a block
pub async fn deserialize_transactions(
	block_value: Value,
) -> Result<Vec<(TransactionHash, Option<Transaction<Proof, DefaultDB>>)>> {
	let raw_tx_data = block_value.get("transactions_index");

	let mut txs = Vec::new();

	if let Some(arr) = raw_tx_data.and_then(|v| v.as_array()) {
		for item in arr {
			if let Some([hash, raw_tx_data]) = item.as_array().and_then(|a| a.get(0..2)) {
				let (hash, tx) = parse_tx_index_item::<Proof>(
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
pub async fn parse_tx_index_item<P: Proofish<DefaultDB>>(
	hash_with_prefix: &str,
	raw_tx_data_with_prefix: &str,
	network_id: NetworkId,
) -> Result<(TransactionHash, Option<Transaction<P, DefaultDB>>), anyhow::Error> {
	// Remove the "0x" prefix from the hash and raw tx data
	let (_hex_prefix, hash_without_prefix) = hash_with_prefix.split_at(2);
	let (_hex_prefix, raw_tx_data_without_prefix) = raw_tx_data_with_prefix.split_at(2);

	let hash = hex::decode(hash_without_prefix)
		.map_err(|e| anyhow::anyhow!("TransactionHashDecodeError: {}", e))?;

	// When testing, we don't have the raw tx data, so we just return the hash
	if raw_tx_data_without_prefix.is_empty() {
		return Ok((
			TransactionHash(HashOutput(
				hash.try_into()
					.map_err(|_| anyhow::anyhow!("Invalid hash length"))?,
			)),
			None,
		));
	}

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

	let tx = deserialize(body.as_slice(), network_id)
		.map_err(|e| anyhow::anyhow!("TransactionDeserializeError: {}", e))?;

	Ok((hash, tx))
}
