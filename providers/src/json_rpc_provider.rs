use near_jsonrpc_client::methods::status::RpcStatusResponse;
use near_jsonrpc_primitives::types::status::RpcStatusError;
use near_jsonrpc_client::{methods, JsonRpcClient};
use async_trait::async_trait;
use near_jsonrpc_client::errors::JsonRpcError;
use near_primitives::views::{FinalExecutionOutcomeView, ChunkView, BlockView, EpochValidatorInfo};
use near_primitives::transaction::SignedTransaction;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
use near_primitives::hash::CryptoHash;
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_jsonrpc_primitives::types::chunks::{RpcChunkError,  ChunkReference};
use near_primitives::types::{BlockReference, EpochReference, Finality};
use near_jsonrpc_primitives::types::blocks::RpcBlockError;
use near_jsonrpc_primitives::types::validator::RpcValidatorError;


use crate::Provider;


pub struct JsonRpcProvider {
    client: JsonRpcClient,
}

impl JsonRpcProvider {
    pub fn new(rpc_endpoint: &str) -> Self {
        Self {
            client: JsonRpcClient::connect(rpc_endpoint),
        }
    }
}

#[async_trait]
impl Provider for JsonRpcProvider {
    async fn status(&self) -> Result<RpcStatusResponse, JsonRpcError<RpcStatusError>> {
        let request = methods::status::RpcStatusRequest; // No params needed
        let server_status: RpcStatusResponse = self.client.call(request).await?;
        Ok(server_status)
    }

    async fn send_transaction(&self, signed_transaction: SignedTransaction) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>>{
        let request = methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
            signed_transaction,
        };
        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn send_transaction_async(&self, signed_transaction: SignedTransaction) -> Result<CryptoHash, JsonRpcError<methods::broadcast_tx_async::RpcBroadcastTxAsyncError>>{
        let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
            signed_transaction,
        };
        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn tx_status(&self, transaction_info: TransactionInfo) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>>{
        let request = methods::tx::RpcTransactionStatusRequest{
            transaction_info,
        };

        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn chunk(&self, chunk_reference: ChunkReference) -> Result<ChunkView, JsonRpcError<RpcChunkError>> {
        let request = methods::chunk::RpcChunkRequest{
            chunk_reference,
        };

        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn block(&self, block_reference: BlockReference) -> Result<BlockView, JsonRpcError<RpcBlockError>> {
        let request = methods::block::RpcBlockRequest{
            block_reference,
        };

        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn validators(&self, epoch_reference: EpochReference) -> Result<EpochValidatorInfo, JsonRpcError<RpcValidatorError>> {
        let request = methods::validators::RpcValidatorRequest{
            epoch_reference,
        };

        let response = self.client.call(request).await?;
        Ok(response)
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
            //println!("Received response: {:?}", response);
            assert!(response.chain_id.contains("testnet"), "Chain ID should contain 'testnet'");
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
            // Perform checks on the response
            // For example, checking if the chain_id matches testnet
            println!("Received response: {:?}", response);
            //assert!(response.chain_id.contains("testnet"), "Chain ID should contain 'testnet'");
        }
        Err(e) => panic!("Status request failed with {:?}", e),
    }
}
