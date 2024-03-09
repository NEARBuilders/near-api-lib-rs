use near_transactions::TransactionBuilder;
use near_crypto::{Signer, PublicKey};
use near_primitives::types::{AccountId, Balance, BlockReference, Finality, FunctionArgs, Gas};
use near_primitives::views::{FinalExecutionOutcomeView, QueryRequest};
use near_jsonrpc_primitives::types::query::{RpcQueryResponse, QueryResponseKind};
use near_primitives::account::AccessKey;
use near_providers::Provider;
use std::sync::Arc;
use crate::access_keys::{full_access_key, function_call_access_key}; 
use serde_json::Value;

pub struct Account {
    pub account_id: AccountId,
    pub signer: Arc<dyn Signer>,
    pub provider: Arc<dyn Provider>, // Use your Provider abstraction
}

impl Account {
    pub fn new(account_id: AccountId, signer: Arc<dyn Signer>, provider: Arc<dyn Provider>) -> Self {
        Self { account_id, signer, provider }
    }

    async fn get_transaction_builder(&self, receiver_id: AccountId) -> Result<TransactionBuilder, Box<dyn std::error::Error>>  {
        // Fetch the current nonce for the signer account and latest block hash
        let nonce = self.fetch_nonce(&self.account_id, &self.signer.public_key()).await?;
        
        //Block hash
        let block_reference = BlockReference::Finality(Finality::Final);
        let block = self.provider.block(block_reference).await?;
        let block_hash = block.header.hash;

        // Use TransactionBuilder to construct the transaction
        let signed_tx = TransactionBuilder::new(
            self.account_id.clone(), 
            self.signer.public_key(), 
            receiver_id,
            nonce+1, 
            block_hash
        );
        Ok(signed_tx)
    }

    // Function to fetch the current nonce for an account's access key
    pub async fn fetch_nonce(&self, account_id: &AccountId, public_key: &PublicKey) -> Result<u64, Box<dyn std::error::Error>> {
        
        let query_request = QueryRequest::ViewAccessKey {
            account_id: account_id.clone(),
            public_key: public_key.clone(),
        };

        // Send the query to the NEAR blockchain
        let response: RpcQueryResponse = self.provider.query(query_request).await?;

        // Extract the access key view from the response
        if let QueryResponseKind::AccessKey(access_key_view) = response.kind {
            Ok(access_key_view.nonce)
        } else {
            Err("Unexpected response kind".into())
        }
    }

    pub async fn create_account(&self, new_account_id: AccountId, public_key: PublicKey, amount: Balance) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {

        // Use TransactionBuilder to construct the transaction
        let signed_tx = self.get_transaction_builder(new_account_id).await?
            .create_account()
            .transfer(amount)
            .add_key(public_key, full_access_key())
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn add_key(&self, public_key: PublicKey, allowance: Option<Balance>, contract_id: Option<String>, method_names: Option<Vec<String>>) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        let nonce = self.fetch_nonce(&self.account_id, &self.signer.public_key()).await?;
        
        //Block hash
        let block_reference = BlockReference::Finality(Finality::Final);
        let block = self.provider.block(block_reference).await?;
        let block_hash = block.header.hash;

        let access_key: AccessKey = match contract_id {
            Some(cid) => {
                if let Some(m_names) = method_names {
                    function_call_access_key(allowance, cid, m_names)
                } else {
                    return Err("No method_names argument provided for function call access keys. You should atleast provie an empty vector.".into());
                }
            },
            None => full_access_key(),
        };

        // Use TransactionBuilder to construct the transaction
        let signed_tx = TransactionBuilder::new(
            self.account_id.clone(), 
            self.signer.public_key(), 
            self.account_id.clone(), 
            nonce+1, 
            block_hash)
            .add_key(public_key, access_key)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn deploy_contract(&self, byte_code: Vec<u8>) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {

        // Use TransactionBuilder to construct the transaction
        let signed_tx = self.get_transaction_builder(self.account_id.clone()).await?
            .deploy_contract(byte_code)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn delete_account(&self, beneficiary_id: AccountId) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {

        // Use TransactionBuilder to construct the transaction
        let signed_tx = self.get_transaction_builder(self.account_id.clone()).await?
            .delete_account(beneficiary_id)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn send_money(&self, receiver_id: AccountId, amount: Balance) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {

        // Use TransactionBuilder to construct the transaction
        let signed_tx = self.get_transaction_builder(receiver_id.clone()).await?
            .transfer(amount)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn function_call(&self, contract_id: AccountId, method_name: String, args: Value, gas: Gas, deposit: Balance) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Serialize the JSON to a Vec<u8>
        let args = serde_json::to_vec(&args)?;
        
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self.get_transaction_builder(contract_id).await?
            .function_call(method_name, args, gas, deposit)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }


    pub async fn view_function(&self, contract_id: AccountId, method_name: String, args: FunctionArgs) -> Result<near_primitives::views::CallResult, Box<dyn std::error::Error>> {
        let query_request = QueryRequest::CallFunction { 
            account_id: contract_id.clone(), 
            method_name: method_name.clone(), 
            args: args.clone()
        };
        
        // Send the query to the NEAR blockchain
        let response: RpcQueryResponse = self.provider.query(query_request).await?;

        if let QueryResponseKind::CallResult(result) = response.kind {
            Ok(result)
        } else {
            Err("Unexpected response kind".into())
        }
    }

}

pub async fn view_state(provider: Arc<dyn Provider>, prefix: String, contract_id: AccountId) -> Result<near_primitives::views::ViewStateResult, Box<dyn std::error::Error>> {

    let query_request = QueryRequest::ViewState { 
        account_id: contract_id,
        prefix: near_primitives::types::StoreKey::from(prefix.into_bytes()),
        include_proof: false,
    };
    
    // Send the query to the NEAR blockchain
    let response: RpcQueryResponse = provider.query(query_request).await?;

    if let QueryResponseKind::ViewState(result) = response.kind {
        Ok(result)
    } else {
        Err("Unexpected response kind".into())
    }
}

pub async fn get_access_key(provider: Arc<dyn Provider>, account_id: AccountId) -> Result<near_primitives::views::AccessKeyList, Box<dyn std::error::Error>> {

    let query_request = QueryRequest::ViewAccessKeyList { 
        account_id:  account_id,
    };
    
    // Send the query to the NEAR blockchain
    let response: RpcQueryResponse = provider.query(query_request).await?;

    if let QueryResponseKind::AccessKeyList(result) = response.kind {
        Ok(result)
    } else {
        Err("Unexpected response kind".into())
    }
}

pub async fn state(provider: Arc<dyn Provider>, account_id: AccountId) -> Result<near_primitives::views::AccountView, Box<dyn std::error::Error>> {

    let query_request = QueryRequest::ViewAccount { 
        account_id: account_id, 
    };
    
    // Send the query to the NEAR blockchain
    let response: RpcQueryResponse = provider.query(query_request).await?;

    if let QueryResponseKind::ViewAccount(result) = response.kind {
        Ok(result)
    } else {
        Err("Unexpected response kind".into())
    }
}