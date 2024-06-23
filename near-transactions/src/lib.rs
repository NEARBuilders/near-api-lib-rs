//! The `near-transactions` crate provides utilities for building, signing, and managing transactions on the NEAR blockchain.
//!
//! It offers a `TransactionBuilder` for constructing transactions with support for various actions such as creating accounts,
//! transferring tokens, deploying contracts, and more. Once a transaction is assembled, it can be signed with a signer
//! and submitted to the NEAR network.
//!
//! # Examples
//!
//! ```no_run
//! use near_transactions::TransactionBuilder;
//! use near_crypto::{InMemorySigner, KeyType};
//! use near_primitives::types::{AccountId, Balance, Gas};
//!
//! let signer = InMemorySigner::from_seed("example.signer", KeyType::ED25519, "seed");
//! let transaction = TransactionBuilder::new(
//!         "example.signer.near".parse().unwrap(),
//!         signer.public_key(),
//!         "example.receiver.near".parse().unwrap(),
//!         1, // nonce
//!         "e...".parse().unwrap(), // block hash
//!     )
//!     .transfer(100_000_000_000_000_000_000_000) // transferring 100 NEAR
//!     .sign_transaction(&signer); // Sign the transaction
//! ```
//!
//! This crate aims to simplify transaction creation and management, making it more accessible for developers to
//! interact with the NEAR blockchain programmatically.

pub use crate::action_builder::ActionBuilder;
pub use crate::delegate_action::{create_delegate_action, create_signed_delegate_action};
pub use crate::transaction_builder::TransactionBuilder;

mod action_builder;
mod delegate_action;
mod transaction_builder;
