use async_trait::async_trait;
use near_chain_configs::ProtocolConfigView;
use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_client::methods::status::RpcStatusResponse;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::blocks::RpcBlockError;
use near_jsonrpc_primitives::types::chunks::{ChunkReference, RpcChunkError};
use near_jsonrpc_primitives::types::config::RpcProtocolConfigError;
use near_jsonrpc_primitives::types::query::{RpcQueryError, RpcQueryRequest, RpcQueryResponse};
use near_jsonrpc_primitives::types::status::RpcStatusError;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_jsonrpc_primitives::types::validator::RpcValidatorError;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::SignedTransaction;
use near_primitives::types::{BlockReference, EpochReference, Finality};
use near_primitives::views::{
    BlockView, ChunkView, EpochValidatorInfo, FinalExecutionOutcomeView, QueryRequest,
};

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

    async fn query(
        &self,
        request: QueryRequest,
    ) -> Result<RpcQueryResponse, JsonRpcError<RpcQueryError>> {
        let query_request = RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request,
        };
        let response: RpcQueryResponse = self.client.call(query_request).await?;
        // Deserialize the JSON string into `RpcQueryResponse`
        // let response: RpcQueryResponse = serde_json::from_str(&response_body)
        //     .map_err(|err| JsonRpcError::DeserializationError(err.to_string()))?;
        Ok(response)
    }

    async fn send_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>> {
        let request =
            methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest { signed_transaction };
        //should we typecast the response here or not,
        //do we recieve json string or specific response type on the basis of requestType
        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn send_transaction_async(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<CryptoHash, JsonRpcError<methods::broadcast_tx_async::RpcBroadcastTxAsyncError>>
    {
        let request =
            methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest { signed_transaction };
        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn tx_status(
        &self,
        transaction_info: TransactionInfo,
    ) -> Result<FinalExecutionOutcomeView, JsonRpcError<RpcTransactionError>> {
        let request = methods::tx::RpcTransactionStatusRequest { transaction_info };

        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn chunk(
        &self,
        chunk_reference: ChunkReference,
    ) -> Result<ChunkView, JsonRpcError<RpcChunkError>> {
        let request = methods::chunk::RpcChunkRequest { chunk_reference };

        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn block(
        &self,
        block_reference: BlockReference,
    ) -> Result<BlockView, JsonRpcError<RpcBlockError>> {
        let request = methods::block::RpcBlockRequest { block_reference };

        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn experimental_protocol_config(
        &self,
        block_reference: BlockReference,
    ) -> Result<ProtocolConfigView, JsonRpcError<RpcProtocolConfigError>> {
        let request =
            methods::EXPERIMENTAL_protocol_config::RpcProtocolConfigRequest { block_reference };
        let response = self.client.call(request).await?;
        Ok(response)
    }

    async fn validators(
        &self,
        epoch_reference: EpochReference,
    ) -> Result<EpochValidatorInfo, JsonRpcError<RpcValidatorError>> {
        let request = methods::validators::RpcValidatorRequest { epoch_reference };

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

//RpcQueryRequest

// pub struct RpcQueryRequest {
//     #[serde(flatten)]
//     pub block_reference: near_primitives::types::BlockReference,
//     #[serde(flatten)]
//     pub request: near_primitives::views::QueryRequest,
// }

//QueryRequest

// #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
// #[serde(tag = "request_type", rename_all = "snake_case")]
// pub enum QueryRequest {
//     ViewAccount {
//         account_id: AccountId,
//     },
//     ViewCode {
//         account_id: AccountId,
//     },
//     ViewState {
//         account_id: AccountId,
//         #[serde(rename = "prefix_base64")]
//         prefix: StoreKey,
//         #[serde(default, skip_serializing_if = "is_false")]
//         include_proof: bool,
//     },
//     ViewAccessKey {
//         account_id: AccountId,
//         public_key: PublicKey,
//     },
//     ViewAccessKeyList {
//         account_id: AccountId,
//     },
//     CallFunction {
//         account_id: AccountId,
//         method_name: String,
//         #[serde(rename = "args_base64")]
//         args: FunctionArgs,
//     },
// }

// RPC Query Response
// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// pub struct RpcQueryResponse {
//     #[serde(flatten)]
//     pub kind: QueryResponseKind,
//     pub block_height: near_primitives::types::BlockHeight,
//     pub block_hash: near_primitives::hash::CryptoHash,
// }

// Query Response Kind
// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// #[serde(untagged)]
// pub enum QueryResponseKind {
//     ViewAccount(near_primitives::views::AccountView),
//     ViewCode(near_primitives::views::ContractCodeView),
//     ViewState(near_primitives::views::ViewStateResult),
//     CallResult(near_primitives::views::CallResult),
//     AccessKey(near_primitives::views::AccessKeyView),
//     AccessKeyList(near_primitives::views::AccessKeyList),
// }
