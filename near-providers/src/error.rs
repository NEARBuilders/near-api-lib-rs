use crate::jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_client::methods::tx::RpcTransactionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error(transparent)]
    JsonRpcError(JsonRpcError<RpcTransactionError>),
}
