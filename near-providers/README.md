# near-providers

The `near-providers` crate provides a high-level abstraction over the `near-jsonrpc-client` for interacting with the NEAR blockchain. It simplifies the process of sending transactions, querying chain status, and performing other chain-related tasks by encapsulating the JSON RPC calls into easy-to-use methods.

## Features

- Simplified interaction with the NEAR blockchain.
- Methods for querying blockchain status, sending transactions, and fetching transaction or block information.
- Support for both synchronous and asynchronous transactions.
- Implementation of the `Provider` trait to allow for easy extension with more providers in the future.

## Getting Started

### Prerequisites

This crate requires Rust and Cargo. Ensure you have the latest version of Rust installed. You can install Rust using [`rustup`](https://rustup.rs/).

### Adding `near-providers` to Your Crate

To use `near-providers`, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
near-providers = "0.1.0-alpha"
```

### Usage
Below is an example of creating a new account, deploying a contract, and querying account state:



```Rust
use near_providers::JsonRpcProvider;

#[tokio::main]
async fn main() {
    let provider = JsonRpcProvider::new("https://rpc.testnet.near.org");
    
    match provider.status().await {
        Ok(status) => println!("Chain status: {:?}", status),
        Err(e) => eprintln!("Error fetching chain status: {:?}", e),
    }
}
```

### Examples

The crate includes examples that demonstrate how to use various features. To run an example, use the following command:

`cargo run --example <example_name>` 

For instance, to test the `send_transaction` function:

`cargo run --example contract_change_method_commit`


## Contributing

We welcome contributions to the `near-providers` crate! Please feel free to submit pull requests or open issues to suggest improvements or add new features.

