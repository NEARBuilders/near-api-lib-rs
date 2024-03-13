use async_trait::async_trait;
use near_chain_configs::ProtocolConfigView;
use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_client::methods;
use near_jsonrpc_client::methods::status::RpcStatusResponse;
use near_jsonrpc_primitives::types::blocks::RpcBlockError;
use near_jsonrpc_primitives::types::chunks::{ChunkReference, RpcChunkError};
use near_jsonrpc_primitives::types::config::RpcProtocolConfigError;
use near_jsonrpc_primitives::types::query::{RpcQueryError, RpcQueryResponse};
use near_jsonrpc_primitives::types::status::RpcStatusError;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_jsonrpc_primitives::types::validator::RpcValidatorError;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::SignedTransaction;
use near_primitives::types::{BlockReference, EpochReference};
use near_primitives::views::{
    BlockView, ChunkView, EpochValidatorInfo, FinalExecutionOutcomeView, QueryRequest,
};

#[async_trait]
pub trait Provider {
    async fn status(&self) -> Result<RpcStatusResponse, JsonRpcError<RpcStatusError>>;
    async fn send_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>>;
    async fn send_transaction_async(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<CryptoHash, JsonRpcError<methods::broadcast_tx_async::RpcBroadcastTxAsyncError>>;
    async fn tx_status(
        &self,
        transaction_info: TransactionInfo,
    ) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>>;
    async fn chunk(
        &self,
        chunk_reference: ChunkReference,
    ) -> Result<ChunkView, JsonRpcError<RpcChunkError>>;
    async fn block(
        &self,
        block_reference: BlockReference,
    ) -> Result<BlockView, JsonRpcError<RpcBlockError>>;
    async fn validators(
        &self,
        epoch_reference: EpochReference,
    ) -> Result<EpochValidatorInfo, JsonRpcError<RpcValidatorError>>;
    async fn query(
        &self,
        request: QueryRequest,
    ) -> Result<RpcQueryResponse, JsonRpcError<RpcQueryError>>;
    async fn experimental_protocol_config(
        &self,
        block_reference: BlockReference,
    ) -> Result<ProtocolConfigView, JsonRpcError<RpcProtocolConfigError>>;
}
