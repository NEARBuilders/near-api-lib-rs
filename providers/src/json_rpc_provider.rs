use near_jsonrpc_client::methods::status::RpcStatusResponse;
use near_jsonrpc_primitives::types::status::RpcStatusError;
use near_jsonrpc_client::{methods, JsonRpcClient};
use async_trait::async_trait;
use near_jsonrpc_client::errors::JsonRpcError;
use near_primitives::views::FinalExecutionOutcomeView;
use near_primitives::transaction::SignedTransaction;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError;


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
            signed_transaction: signed_transaction,
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
