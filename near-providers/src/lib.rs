// Re-export the Provider trait
pub use crate::provider::Provider;
// Re-export the JsonRpcProvider
pub use crate::json_rpc_provider::JsonRpcProvider;

mod json_rpc_provider;
mod provider;
