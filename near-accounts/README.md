# near-accounts

The `near-accounts` is a Rust crate designed to simplify interaction with NEAR Protocol accounts. Building upon the lower-level `near-transactions` and `near-providers` crate, it provides developers with high-level abstractions for managing accounts, deploying contracts, and executing transactions on the NEAR blockchain.

  
## Features

- Account Management: Create and delete NEAR Protocol accounts.

- Access Key Management: Add and delete access keys for accounts.

- Contract Deployment: Deploy and call smart contracts.

- Transaction Handling: Send NEAR and call contract functions with customizable gas and attached deposit.

- Querying Blockchain State: View account details, contract state, and call view functions on contracts.

- Asynchronous API: All network interactions are asynchronous, leveraging tokio for efficient concurrency.

- Builder Pattern: Utilizes TransactionBuilder for a flexible and error-resistant way to construct transactions.

  

## Getting Started

 ### Adding `near-accounts` to Your Crate
 
To use `near-accounts` in your project, add the following to your Cargo.toml:
```toml
[dependencies]
near-accounts = "0.1.0-alpha"
```

Ensure you have `tokio` and other dependencies set up in your project for asynchronous runtime.

 ### Usage
Look at `deploy_contract.rs` example in the examples directory. 

```rust
#[tokio::main]
async  fn main() -> Result<(), Box<dyn std::error::Error>> {
env_logger::init();
let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;

let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);
let signer = Arc::new(signer);

let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));

let account = Account::new(signer_account_id, signer, provider);
let wasm_code = read_wasm_file()?;

let result = account.deploy_contract(wasm_code).await;
println!("response: {:#?}", result);
Ok(())
}
```

 ### Examples
The crate includes examples that demonstrate how to use various features. To run an example, use the following command:
 
`cargo run --example <example_name>`

For instance, to test the `create_account` function:

`cargo run --example create_subaccount`

 
## Contributing
We welcome contributions to the `near-accounts` crate! Please feel free to submit pull requests or open issues to suggest improvements or add new features.