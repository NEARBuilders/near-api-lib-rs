//! The `near-accounts` crate provides a high-level abstraction for managing accounts on the NEAR blockchain.
//! It simplifies the process of creating new accounts, managing access keys, deploying contracts, and executing transactions.
//!
//! Built on top of the `near-transactions` and `near-providers` crate, `near-accounts` utilizes the `TransactionBuilder`
//! to construct and sign transactions in a flexible and error-resistant manner. This crate abstracts away
//! the complexities involved in directly interacting with NEAR transactions and provides a more
//! accessible interface for Rust developers to build NEAR blockchain applications.
//!
//! Key functionalities include:
//! - Account creation and deletion
//! - Access key management (adding and deleting keys)
//! - Smart contract deployment and function calls
//! - NEAR token transfers
//! - Querying account information and contract state
//!
//! `near-accounts` integrates with the `near-providers` crate to interact with the NEAR blockchain,
//! offering both synchronous and asynchronous methods to perform blockchain operations.
//!
//! # Example - `state`
//!
//! ```no_run
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use near_providers::JsonRpcProvider;
//! use std::sync::Arc;
//! use near_accounts::accounts::state;
//!
//! let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
//! let account_id = "example_account.testnet".parse()?;
//!
//! let result = state(provider, account_id).await?;
//! println!("Account state: {:?}", result);
//! # Ok(())
//! # }
//! ```
//!
//! # Example - `function_call`
//! ```no_run
//! use near_accounts::Account;
//! use near_crypto::InMemorySigner;
//! use near_primitives::types::Gas;
//! use near_providers::JsonRpcProvider;
//! use std::sync::Arc;
//! use near_primitives::types::AccountId;
//! use serde_json::json;

//! #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     env_logger::init();

//!     let signer_account_id: AccountId = ("your_account_id")?.parse()?;
//!     let signer_secret_key = utils::input("your_private_key")?.parse()?;
//!     let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;
//!     let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

//!     // Amount to transfer to the new account
//!     let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR

//!     let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
//!     let signer = Arc::new(signer);

//!     let account = Account::new(signer_account_id, signer, provider);
//!     let method_name = "set_status".to_string();

//!     let args_json = json!({"message": "working1"});
//!     let result = account
//!         .function_call(contract_id, method_name, args_json, gas, 0)
//!         .await;

//!     println!("response: {:#?}", result);

//! #    Ok(())
//! #}
//! ```

pub use crate::accounts::Account;

mod access_keys;
pub mod accounts;
mod transaction_sender;
