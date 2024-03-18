//! `near-providers` crate provides a set of abstractions for connecting to the NEAR blockchain,
//! allowing users to interact with the network in a simplified way. This crate includes
//! the `Provider` trait, which defines a common interface for blockchain interactions,
//! and specific implementations of this interface, such as the `JsonRpcProvider`.
//!
//! The `Provider` trait offers methods for querying blockchain status, sending transactions,
//! and retrieving information about transactions, blocks, chunks, and validators.
//! The `JsonRpcProvider` is an implementation of the `Provider` trait that uses JSON RPC
//! to communicate with NEAR blockchain nodes.
//!
//! This crate is designed to be easily extendable with more providers and to offer a
//! straightforward way to integrate NEAR blockchain functionalities into Rust applications.

// Re-export the Provider trait
pub use crate::provider::Provider;
// Re-export the JsonRpcProvider
pub use crate::json_rpc_provider::JsonRpcProvider;

/// Re-exporting jsonrpc_primitives types so users of near-providers don't need
/// to keep track of multiple jsonrpc crates. For now we export them as types
/// but when we implement more providers, we can change it to jsonrpc_types
pub use near_jsonrpc_primitives::types;

pub use near_jsonrpc_client as jsonrpc_client;
/// Use near_jsonrpc_primitives/client easily with near_providers
pub use near_jsonrpc_primitives as jsonrpc_primitives;

mod json_rpc_provider;
mod provider;
