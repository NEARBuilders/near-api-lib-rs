
# near-transactions

The `near-transactions` crate is a Rust crate designed to facilitate the creation, manipulation, and signing of transactions on the NEAR blockchain. Leveraging the builder pattern, this crate provides a fluent and flexible way to construct transactions programmatically, simplifying the process of interacting with the blockchain.

## Features

- Transaction Builder: At the heart of this crate is the TransactionBuilder, a struct that guides users through the construction of transactions. Starting with essential details like signer and receiver IDs, it incrementally builds up a transaction by adding actions such as token transfers, contract deployments, and function calls.

- Comprehensive Action Support: Supports a wide array of actions including creating accounts, deploying contracts, transferring tokens, staking tokens, adding keys, deleting keys, and deleting accounts. This allows for the execution of complex operations on the NEAR blockchain.

- Signing Transactions: Once a transaction is fully constructed, it can be signed using a Signer.

- Extensibility: Designed with extensibility in mind, enabling the addition of more actions and features in the future without breaking existing implementations.

  

## Getting Started

Add near-transactions to your Cargo.toml to start building and signing transactions:

```toml
[dependencies]
near-transactions = "0.1.0-alpha"
```

### Example Usage

```rust
use near_transactions::TransactionBuilder;
use near_crypto::{InMemorySigner, KeyType};
use near_primitives::types::{AccountId, Balance, Gas};

fn main() {
    // Initialize a signer
    let signer = InMemorySigner::from_seed("example.signer", KeyType::ED25519, "seed");

    // Build a transaction
    let transaction = TransactionBuilder::new(
            "example.signer.near".parse().unwrap(),
            signer.public_key(),
            "example.receiver.near".parse().unwrap(),
            1, // nonce
            "e...".parse().unwrap(), // block hash
        )
        .transfer(100_000_000_000_000_000_000_000_000) // transferring 100 NEAR
        .sign_transaction(&signer); // Sign the transaction

    // Now `transaction` is ready to be sent to the blockchain
}
``` 

This example demonstrates how to construct and sign a simple transaction for transferring tokens. The process involves creating a `TransactionBuilder`, adding the desired actions, and finally signing the transaction with a signer.


## Contributing

We welcome contributions to the `near-transactions` crate! Please feel free to submit pull requests or open issues to suggest improvements or add new features.