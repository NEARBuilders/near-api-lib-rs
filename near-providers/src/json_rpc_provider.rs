//! The `JsonRpcProvider` module offers a concrete implementation of the `Provider` trait, utilizing JSON RPC to communicate with the NEAR blockchain.
//! This provider enables applications to query blockchain status, submit transactions, and fetch various blockchain data in an asynchronous manner.

use crate::types::{
    blocks::RpcBlockError,
    chunks::{ChunkReference, RpcChunkError},
    config::RpcProtocolConfigError,
    query::{RpcQueryError, RpcQueryRequest, RpcQueryResponse},
    status::RpcStatusError,
    transactions::{RpcTransactionError, TransactionInfo},
    validator::RpcValidatorError,
};
use crate::Provider;
use crate::{
    error::ProviderError,
    jsonrpc_client::{
        errors::JsonRpcError,
        methods::{self, status::RpcStatusResponse},
        JsonRpcClient,
    },
};
use async_trait::async_trait;
use near_chain_configs::ProtocolConfigView;
use near_jsonrpc_client::methods::tx::RpcTransactionResponse;
use near_primitives::{
    hash::CryptoHash,
    transaction::SignedTransaction,
    types::{BlockReference, EpochReference, Finality},
    views::{
        BlockView, ChunkView, EpochValidatorInfo, FinalExecutionOutcomeView, QueryRequest,
        TxExecutionStatus,
    },
};

/// Represents a provider that uses JSON RPC to interact with the NEAR blockchain.
pub struct JsonRpcProvider {
    client: JsonRpcClient,
}

impl JsonRpcProvider {
    /// Constructs a new `JsonRpcProvider` with the specified RPC endpoint.
    pub fn new(rpc_endpoint: &str) -> Self {
        Self {
            client: JsonRpcClient::connect(rpc_endpoint),
        }
    }
}

#[async_trait]
impl Provider for JsonRpcProvider {
    /// Retrieves the current status of the NEAR blockchain.
    async fn status(&self) -> Result<RpcStatusResponse, JsonRpcError<RpcStatusError>> {
        let request = methods::status::RpcStatusRequest; // No params needed
        self.client.call(request).await
    }

    /// Executes a query on the NEAR blockchain using a given `QueryRequest`.
    async fn query(
        &self,
        request: QueryRequest,
    ) -> Result<RpcQueryResponse, JsonRpcError<RpcQueryError>> {
        // example, test the request here and throw an error if it is wrong.
        // Or maybe just pass on what internal method returns
        let query_request = RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request,
        };
        //do something similar here as well.
        self.client.call(query_request).await
    }

    /// Sends a signed transaction to the NEAR blockchain, waiting for its final execution outcome.
    async fn send_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>> {
        let request =
            methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest { signed_transaction };
        self.client.call(request).await
    }

    /// Sends a signed transaction to the NEAR blockchain asynchronously, without waiting for its final execution outcome.
    async fn send_transaction_async(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<CryptoHash, JsonRpcError<methods::broadcast_tx_async::RpcBroadcastTxAsyncError>>
    {
        let request =
            methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest { signed_transaction };
        self.client.call(request).await
    }

    /// Sends a signed transaction to the NEAR blockchain. With additional parameter wait_until to define transaction finality.
    async fn send_tx(
        &self,
        signed_transaction: SignedTransaction,
        wait_until: TxExecutionStatus,
    ) -> Result<RpcTransactionResponse, ProviderError> {
        let request = methods::send_tx::RpcSendTransactionRequest {
            signed_transaction,
            wait_until: wait_until.clone(),
        };
        self.client
            .call(request)
            .await
            .map_err(|e| ProviderError::JsonRpcError(e))
    }

    /// Retrieves the status of a transaction on the NEAR blockchain, identified by `TransactionInfo`.
    async fn tx_status(
        &self,
        transaction_info: TransactionInfo,
        wait_until: TxExecutionStatus,
    ) -> Result<RpcTransactionResponse, JsonRpcError<RpcTransactionError>> {
        let request = methods::tx::RpcTransactionStatusRequest {
            transaction_info,
            wait_until,
        };

        self.client.call(request).await
    }

    /// Fetches details of a specific chunk from the NEAR blockchain, identified by `ChunkReference`.
    async fn chunk(
        &self,
        chunk_reference: ChunkReference,
    ) -> Result<ChunkView, JsonRpcError<RpcChunkError>> {
        let request = methods::chunk::RpcChunkRequest { chunk_reference };

        self.client.call(request).await
    }

    /// Retrieves a block from the NEAR blockchain, specified by its `BlockReference`.
    async fn block(
        &self,
        block_reference: BlockReference,
    ) -> Result<BlockView, JsonRpcError<RpcBlockError>> {
        let request = methods::block::RpcBlockRequest { block_reference };

        self.client.call(request).await
    }

    /// Fetches the experimental protocol configuration for a specific block, identified by `BlockReference`.
    async fn experimental_protocol_config(
        &self,
        block_reference: BlockReference,
    ) -> Result<ProtocolConfigView, JsonRpcError<RpcProtocolConfigError>> {
        let request =
            methods::EXPERIMENTAL_protocol_config::RpcProtocolConfigRequest { block_reference };
        self.client.call(request).await
    }

    /// Retrieves information about validators for a given epoch, specified by `EpochReference`.
    async fn validators(
        &self,
        epoch_reference: EpochReference,
    ) -> Result<EpochValidatorInfo, JsonRpcError<RpcValidatorError>> {
        let request = methods::validators::RpcValidatorRequest { epoch_reference };

        self.client.call(request).await
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_status() {
    let provider = JsonRpcProvider::new("https://rpc.testnet.near.org");
    match provider.status().await {
        Ok(response) => {
            // Perform checks on the response
            // For example, checking if the chain_id matches testnet
            println!("Received response: {:?}", response);
            assert!(
                response.chain_id.contains("testnet"),
                "Chain ID should contain 'testnet'"
            );
        }
        Err(e) => panic!("Status request failed with {:?}", e),
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_block() {
    let provider = JsonRpcProvider::new("https://rpc.testnet.near.org");
    let block_reference = BlockReference::Finality(Finality::Final);
    //let block_hash = provider.block(block_reference).await?;
    match provider.block(block_reference).await {
        Ok(response) => {
            println!("{}", response.author);
        }
        Err(e) => panic!("Status request failed with {:?}", e),
    }
}
