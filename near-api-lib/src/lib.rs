//! A comprehensive Rust library for interacting with the NEAR blockchain.
//!
//! This crate simplifies the process of building applications on the NEAR platform by providing
//! re-exports of key components necessary for interacting with the NEAR blockchain, including
//! account management, transaction building and signing, provider interfaces for blockchain queries,
//! and cryptographic utilities.
//!
//! It is designed to offer Rust developers a streamlined, idiomatic way to create, sign, and
//! broadcast transactions, manage accounts, and query blockchain state without dealing with the
//! underlying complexity directly.
//!
//! ## Features
//! - Account management through `near_accounts`
//! - Transaction construction, signing, and submission via `near_transactions`
//! - Blockchain interaction through the `near_providers` with `JsonRpcProvider`
//! - Cryptographic operations facilitated by `near_crypto`
//! - Access to NEAR blockchain primitives through `near_primitives`
//!
//! This crate aims to be a one-stop solution for Rust developers building on the NEAR platform,
//! providing the necessary tools and abstractions to create robust, secure, and scalable applications.

pub use near_accounts::accounts;
pub use near_accounts::Account;

pub use near_providers as providers;
pub use near_providers::JsonRpcProvider;

pub use near_crypto::InMemorySigner;

pub use near_primitives as primitives;
