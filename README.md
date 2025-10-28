# midnight-tx-bug

This project is a **minimal reproducible example** demonstrating a bug in the transaction deserialization process when using a minimal Rust async RPC client to fetch and parse block and transaction data from a Midnight node using JSON-RPC.

## Bug Demonstrated

You may encounter the following error during transaction deserialization:

```
TransactionDeserializeError: expected header tag 'midnight:option(transaction[v6](signature[v1],(),pedersen-schnorr[v1])):', got '������������������o������Ni�S���u�,��)�s�(Y������]�����X��������tP���'
```

This error occurs when attempting to deserialize transaction data from the Midnight node's `transactions_index` field. The issue is that the data from the RPC is hex-encoded binary data without the `midnight:` tag prefix, but the `deserialize` function expects tagged data.

## Prerequisites

- Rust (https://rustup.rs/)
- Internet access to connect to a Midnight node (default: testnet)

## Run the example

```
cargo run
```

The example will:

1. Connect to a Midnight node (see `src/main.rs` for the URL).
2. Fetch a block by number using JSON-RPC.
3. Attempt to parse and print the block and transaction data, demonstrating the deserialization bug.

You can change the node URL or block number in `src/main.rs` as needed.

---

**Note:** If you want to use a different node or block, edit the `node_url` and `block_number` variables in `src/main.rs`.
