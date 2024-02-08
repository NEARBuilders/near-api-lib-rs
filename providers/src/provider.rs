use async_trait::async_trait;
use near_jsonrpc_client::methods::status::RpcStatusResponse;
use near_jsonrpc_primitives::types::status::RpcStatusError;
use near_primitives::views::FinalExecutionOutcomeView;
use near_primitives::transaction::SignedTransaction;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
use near_jsonrpc_client::errors::JsonRpcError;


// To-do
// Implement a Conversion From JsonRpcError<RpcStatusError> to RpcStatusError: If you need to keep the RpcStatusError as your function's error type for consistency or other reasons, you can implement a conversion using the From trait or manually handle the conversion in each call.

// Example of Manual Error Handling
// If you prefer to explicitly handle the error conversion to keep the RpcStatusError type, you can do something like this:

// rust
// Copy code
// use near_jsonrpc_primitives::errors::RpcError;

// // Inside your async function
// match self.client.call(request).await {
//     Ok(response) => Ok(response),
//     Err(e) => match e {
//         JsonRpcError::ServerError(inner) => Err(inner), // Assuming RpcStatusError can be directly extracted
//         _ => Err(RpcStatusError::new(/* construct your error here based on `e` */)),
//     },
// }


#[async_trait]
pub trait Provider {
    async fn status(&self) -> Result<RpcStatusResponse, JsonRpcError<RpcStatusError>>;
    async fn send_transaction(&self, signed_transaction: SignedTransaction) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>>;
}