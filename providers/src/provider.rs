use async_trait::async_trait;
use near_jsonrpc_client::methods::status::RpcStatusResponse;
use near_jsonrpc_primitives::types::status::RpcStatusError;
use near_primitives::views::{FinalExecutionOutcomeView, ChunkView, BlockView, EpochValidatorInfo};
use near_primitives::transaction::SignedTransaction;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_client::methods;
use near_primitives::hash::CryptoHash;
use near_primitives::types::{BlockReference, EpochReference};
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_jsonrpc_primitives::types::chunks::{RpcChunkError,  ChunkReference};
use near_jsonrpc_primitives::types::blocks::RpcBlockError;
use near_jsonrpc_primitives::types::validator::RpcValidatorError;
use near_jsonrpc_primitives::types::query::{RpcQueryError, RpcQueryRequest, RpcQueryResponse, QueryResponseKind};

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
    async fn send_transaction_async(&self, signed_transaction: SignedTransaction) -> Result<CryptoHash, JsonRpcError<methods::broadcast_tx_async::RpcBroadcastTxAsyncError>>;
    async fn tx_status(&self, transaction_info: TransactionInfo) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>>;
    async fn chunk(&self, chunk_reference: ChunkReference) -> Result<ChunkView, JsonRpcError<RpcChunkError>>;
    async fn block(&self, block_reference: BlockReference) -> Result<BlockView, JsonRpcError<RpcBlockError>>;
    async fn validators(&self, epoch_reference: EpochReference) -> Result<EpochValidatorInfo, JsonRpcError<RpcValidatorError>>;
    async fn query<T: QueryResponseKind>(&self, params: RpcQueryRequest) -> Result<T, JsonRpcError<RpcQueryError>>;
    //chat gpt version
    //async fn query<T: QueryResponseKind>(&self, params: RpcQueryRequest) -> Result<T, Box<dyn std::error::Error>>;
}