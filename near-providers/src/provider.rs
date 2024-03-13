//! This module defines the `Provider` trait for interacting with the NEAR blockchain. 
//! It provides a high-level API for various blockchain operations, such as querying the chain status, 
//! sending transactions, and fetching information about blocks, chunks, and validators. 
//! The `Provider` trait is designed to be implemented by specific providers, with a JSON RPC provider currently implemented, 
//! allowing users to easily connect to and interact with the NEAR chain.


use async_trait::async_trait;
use near_jsonrpc_client::{
    errors::JsonRpcError,
    methods::{self, status::RpcStatusResponse},
};
use near_primitives::{
    hash::CryptoHash,
    transaction::SignedTransaction,
    types::{BlockReference, EpochReference},
    views::{BlockView, ChunkView, EpochValidatorInfo, FinalExecutionOutcomeView, QueryRequest},
};
use near_chain_configs::ProtocolConfigView;
use near_jsonrpc_primitives::types::{
    blocks::RpcBlockError,
    chunks::{ChunkReference, RpcChunkError},
    config::RpcProtocolConfigError,
    query::{RpcQueryError, RpcQueryResponse},
    status::RpcStatusError,
    transactions::{RpcTransactionError, TransactionInfo},
    validator::RpcValidatorError,
};

#[async_trait]
pub trait Provider {
    /// Fetches the current status of the NEAR blockchain.
    async fn status(&self) -> Result<RpcStatusResponse, JsonRpcError<RpcStatusError>>;

    /// Sends a transaction to the NEAR blockchain, waiting for its final execution outcome.
    async fn send_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>>;

    /// Sends a transaction to the NEAR blockchain asynchronously, without waiting for its final execution outcome.
    async fn send_transaction_async(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<CryptoHash, JsonRpcError<methods::broadcast_tx_async::RpcBroadcastTxAsyncError>>;

    /// Fetches the status of a specific transaction, given its information.
    async fn tx_status(
        &self,
        transaction_info: TransactionInfo,
    ) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>>;

    /// Retrieves information about a specific chunk, identified by its chunk reference.
    async fn chunk(
        &self,
        chunk_reference: ChunkReference,
    ) -> Result<ChunkView, JsonRpcError<RpcChunkError>>;

    /// Retrieves a block from the NEAR blockchain, specified by its block reference.
    async fn block(
        &self,
        block_reference: BlockReference,
    ) -> Result<BlockView, JsonRpcError<RpcBlockError>>;

    /// Fetches information about validators for a specified epoch, identified by its epoch reference (epoch ID).
    async fn validators(
        &self,
        epoch_reference: EpochReference,
    ) -> Result<EpochValidatorInfo, JsonRpcError<RpcValidatorError>>;

    /// Performs a query to the NEAR blockchain, allowing to retrieve data based on a specific request (e.g., account details, contract state).
    async fn query(
        &self,
        request: QueryRequest,
    ) -> Result<RpcQueryResponse, JsonRpcError<RpcQueryError>>;

    /// Retrieves the protocol configuration data for a specific block, identified by its block reference.
    async fn experimental_protocol_config(
        &self,
        block_reference: BlockReference,
    ) -> Result<ProtocolConfigView, JsonRpcError<RpcProtocolConfigError>>;
}
