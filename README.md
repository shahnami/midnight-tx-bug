# midnight-tx-bug

This project is a **minimal reproducible example** demonstrating a bug in the transaction deserialization process when using a minimal Rust async RPC client to fetch and parse block and transaction data from a Midnight node using JSON-RPC.

## Bug Demonstrated

You may encounter the following error during transaction deserialization:

```
TransactionDeserializeError: Invalid input data for core::option::Option<midnight_ledger::structure::Transaction<midnight_ledger::structure::Proof, midnight_storage::db::InMemoryDB>>, received version: None, maximum supported version is None. Invalid discriminant: 4.
```

This error occurs when attempting to decode certain transaction data from the Midnight node.

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
