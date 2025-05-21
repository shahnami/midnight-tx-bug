use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::{Value, json};

pub struct RpcClient {
	url: String,
	client: Client,
}

impl RpcClient {
	pub fn new(url: &str) -> Self {
		RpcClient {
			url: url.to_string(),
			client: Client::new(),
		}
	}

	pub async fn get_block_at_number(&self, block_number: u64) -> Result<Value, anyhow::Error> {
		let params = json!([format!("0x{:x}", block_number)]);

		let resp = self
			.send_raw_request("chain_getBlockHash", Some(params))
			.await
			.unwrap();

		let block_hash = resp
			.get("result")
			.and_then(|v| v.as_str())
			.ok_or_else(|| anyhow::anyhow!("Missing 'result' field"))?;

		let params = json!([block_hash]);

		let resp = self
			.send_raw_request("midnight_jsonBlock", Some(params))
			.await
			.unwrap();

		let block_data = resp
			.get("result")
			.ok_or_else(|| anyhow::anyhow!("Missing 'result' field"))?;

		// Parse the JSON string into a Value
		let block_value: serde_json::Value = serde_json::from_str(
			block_data
				.as_str()
				.ok_or_else(|| anyhow::anyhow!("Result is not a string"))?,
		)
		.with_context(|| "Failed to parse block JSON string")?;

		if block_value.is_null() {
			return Err(anyhow::anyhow!("Block not found"));
		}

		Ok(block_value)
	}

	pub async fn send_raw_request(
		&self,
		method: &str,
		params: Option<Value>,
	) -> Result<Value, anyhow::Error> {
		let req_body = json!({
			"jsonrpc": "2.0",
			"method": method,
			"params": params.unwrap_or_else(|| json!([])),
			"id": 1
		});
		let resp = self
			.client
			.post(&self.url)
			.json(&req_body)
			.send()
			.await
			.map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;
		let resp_json: Value = resp
			.json()
			.await
			.map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;
		Ok(resp_json)
	}
}
