use crate::access_keys::{full_access_key, function_call_access_key};
use near_crypto::{PublicKey, Signer};
use near_primitives::account::AccessKey;
use near_primitives::types::{AccountId, Balance, BlockReference, Finality, FunctionArgs, Gas};
use near_primitives::views::{FinalExecutionOutcomeView, QueryRequest};
use near_providers::types::query::{QueryResponseKind, RpcQueryResponse};
use near_providers::Provider;
use near_transactions::TransactionBuilder;
use num_bigint::BigInt;
use serde_json::Value;
use std::sync::Arc;

pub struct Account {
    pub account_id: AccountId,
    pub signer: Arc<dyn Signer>,     // Use your Signer abstraction
    pub provider: Arc<dyn Provider>, // Use your Provider abstraction
}

#[derive(Debug)]
pub struct AccountBalance {
    pub total: String,
    pub state_staked: String,
    pub staked: String,
    pub available: String,
}

impl Account {
    pub fn new(
        account_id: AccountId,
        signer: Arc<dyn Signer>,
        provider: Arc<dyn Provider>,
    ) -> Self {
        Self {
            account_id,
            signer,
            provider,
        }
    }

    async fn get_transaction_builder(
        &self,
        receiver_id: AccountId,
    ) -> Result<TransactionBuilder, Box<dyn std::error::Error>> {
        // Fetch the current nonce for the signer account and latest block hash
        let nonce = self
            .fetch_nonce(&self.account_id, &self.signer.public_key())
            .await?;

        //Block hash
        let block_reference = BlockReference::Finality(Finality::Final);
        let block = self.provider.block(block_reference).await?;
        let block_hash = block.header.hash;

        // Use TransactionBuilder to construct the transaction
        let signed_tx = TransactionBuilder::new(
            self.account_id.clone(),
            self.signer.public_key(),
            receiver_id,
            nonce + 1,
            block_hash,
        );
        Ok(signed_tx)
    }

    // Function to fetch the current nonce for an account's access key
    pub async fn fetch_nonce(
        &self,
        account_id: &AccountId,
        public_key: &PublicKey,
    ) -> Result<u64, Box<dyn std::error::Error>> {
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

    pub async fn create_account(
        &self,
        new_account_id: AccountId,
        public_key: PublicKey,
        amount: Balance,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(new_account_id)
            .await?
            .create_account()
            .transfer(amount)
            .add_key(public_key, full_access_key())
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn add_key(
        &self,
        public_key: PublicKey,
        allowance: Option<Balance>,
        contract_id: Option<String>,
        method_names: Option<Vec<String>>,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        let access_key: AccessKey = match contract_id {
            Some(cid) => {
                if let Some(m_names) = method_names {
                    function_call_access_key(allowance, cid, m_names)
                } else {
                    return Err("No method_names argument provided for function call access keys. You should atleast provie an empty vector.".into());
                }
            }
            None => full_access_key(),
        };

        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(self.account_id.clone())
            .await?
            .add_key(public_key, access_key)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn delete_key(
        &self,
        public_key: PublicKey,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(self.account_id.clone())
            .await?
            .delete_key(public_key)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn deploy_contract(
        &self,
        byte_code: Vec<u8>,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(self.account_id.clone())
            .await?
            .deploy_contract(byte_code)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn delete_account(
        &self,
        beneficiary_id: AccountId,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(self.account_id.clone())
            .await?
            .delete_account(beneficiary_id)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn send_money(
        &self,
        receiver_id: AccountId,
        amount: Balance,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(receiver_id.clone())
            .await?
            .transfer(amount)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn function_call(
        &self,
        contract_id: AccountId,
        method_name: String,
        args: Value,
        gas: Gas,
        deposit: Balance,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Serialize the JSON to a Vec<u8>
        let args = serde_json::to_vec(&args)?;

        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(contract_id)
            .await?
            .function_call(method_name, args, gas, deposit)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    pub async fn view_function(
        &self,
        contract_id: AccountId,
        method_name: String,
        args: FunctionArgs,
    ) -> Result<near_primitives::views::CallResult, Box<dyn std::error::Error>> {
        let query_request = QueryRequest::CallFunction {
            account_id: contract_id.clone(),
            method_name: method_name.clone(),
            args: args.clone(),
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

pub async fn view_state(
    provider: Arc<dyn Provider>,
    contract_id: AccountId,
    prefix: Option<String>,
) -> Result<near_primitives::views::ViewStateResult, Box<dyn std::error::Error>> {
    let prefix_op = match prefix {
        Some(pf) => pf,
        None => String::from(""),
    };
    let query_request = QueryRequest::ViewState {
        account_id: contract_id,
        prefix: near_primitives::types::StoreKey::from(prefix_op.into_bytes()),
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

pub async fn get_access_key(
    provider: Arc<dyn Provider>,
    account_id: AccountId,
) -> Result<near_primitives::views::AccessKeyList, Box<dyn std::error::Error>> {
    let query_request = QueryRequest::ViewAccessKeyList { account_id };

    // Send the query to the NEAR blockchain
    let response: RpcQueryResponse = provider.query(query_request).await?;

    if let QueryResponseKind::AccessKeyList(result) = response.kind {
        Ok(result)
    } else {
        Err("Unexpected response kind".into())
    }
}

pub async fn state(
    provider: Arc<dyn Provider>,
    account_id: AccountId,
) -> Result<near_primitives::views::AccountView, Box<dyn std::error::Error>> {
    let query_request = QueryRequest::ViewAccount { account_id };

    // Send the query to the NEAR blockchain
    let response: RpcQueryResponse = provider.query(query_request).await?;

    if let QueryResponseKind::ViewAccount(result) = response.kind {
        Ok(result)
    } else {
        Err("Unexpected response kind".into())
    }
}

pub async fn get_account_balance(
    provider: Arc<dyn Provider>,
    account_id: AccountId,
) -> Result<AccountBalance, Box<dyn std::error::Error>> {
    // Assuming `experimental_protocol_config` and `state` are async functions you can call on the provider
    let block_reference = BlockReference::Finality(Finality::Final);
    let protocol_config = provider
        .experimental_protocol_config(block_reference)
        .await?;
    let cost_per_byte = BigInt::from(protocol_config.runtime_config.storage_amount_per_byte);

    let state = state(provider, account_id).await?;

    // Assuming state.storage_usage, state.locked, and state.amount are already BigInt or can be converted to BigInt
    let state_staked = BigInt::from(state.storage_usage) * &cost_per_byte;
    let staked = BigInt::from(state.locked);
    let total_balance = BigInt::from(state.amount) + &staked;
    let available_balance = if staked > state_staked {
        &total_balance - &staked
    } else {
        &total_balance - &state_staked
    };

    // Convert BigInt to String for the struct. Handle potential conversion errors as needed
    Ok(AccountBalance {
        total: total_balance.to_string(),
        state_staked: state_staked.to_string(),
        staked: staked.to_string(),
        available: available_balance.to_string(),
    })
}
