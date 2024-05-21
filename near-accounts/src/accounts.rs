//! The `accounts` module provides functionalities for managing NEAR blockchain accounts,
//! allowing for operations such as account creation, access key management, contract deployment, making change and view function calls,
//! and transaction execution. It abstracts the complexities of transaction construction and signing,
//! making it easier to perform account-related operations.

use crate::access_keys::{full_access_key, function_call_access_key};
use near_crypto::{PublicKey, Signer};
use near_primitives::account::AccessKey;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::SignedTransaction;
use near_primitives::types::{AccountId, Balance, BlockReference, Finality, Gas};
use near_primitives::views::{FinalExecutionOutcomeView, QueryRequest, TxExecutionStatus};

use near_providers::types::query::{QueryResponseKind, RpcQueryResponse};
use near_providers::types::transactions::RpcTransactionResponse;
use near_providers::Provider;
use near_transactions::{ActionBuilder, TransactionBuilder};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::{Add, Mul, Sub};
use std::sync::Arc;

///This struct represent a Transaction Sender used specifically if you want to send transactions manually.
/// This gives user more control over how they want to send their transactions to the NEAR network for examples, asyn, sync or advanced.
/// It is only used by function_call method from Account for now to enable this flexibility.
#[derive(Clone)]
pub struct TransactionSender {
    pub signed_transaction: SignedTransaction,
    provider: Arc<dyn Provider>,
}

impl TransactionSender {
    /// Constructs a new `TransactionSender` instance.
    ///
    /// # Arguments
    ///
    /// * `signed_transaction` - Signed transaction to be sent to the NEAR chain.
    /// * `provider` - A provider instance for interacting with the blockchain.
    ///
    /// # Returns
    ///
    /// A new `Account` instance.
    pub fn new(signed_transaction: SignedTransaction, provider: Arc<dyn Provider>) -> Self {
        Self {
            signed_transaction,
            provider,
        }
    }

    ///Send your transaction to the NEAR blockchain synchronously using the send_tx RPC end point and default wait_until value
    pub async fn transact(self) -> Result<RpcTransactionResponse, Box<dyn std::error::Error>> {
        self.provider
            .send_tx(self.signed_transaction, TxExecutionStatus::default())
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    ///Send your transaction to the NEAR blockchain asynchronously using the send_tx RPC end point and default wait_until None.
    pub async fn transact_async(
        self,
    ) -> Result<RpcTransactionResponse, Box<dyn std::error::Error>> {
        self.provider
            .send_tx(self.signed_transaction, TxExecutionStatus::None)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    ///Send your transaction to the NEAR blockchain using the send_tx RPC end point and custom wait_until value.
    /// Different wait_until values and what they mean:
    ///
    /// * None
    /// Transaction is waiting to be included into the block
    ///
    /// * Included
    /// Transaction is included into the block. The block may be not finalised yet
    ///
    /// * ExecutedOptimistic,
    /// Transaction is included into the block +
    /// All the transaction receipts finished their execution.
    /// The corresponding blocks for tx and each receipt may be not finalised yet
    /// It is also the default value unless defined otherwise.
    ///
    /// * IncludedFinal
    /// Transaction is included into finalised block
    ///
    /// * Executed
    /// Transaction is included into finalised block +
    /// All the transaction receipts finished their execution.
    /// The corresponding blocks for each receipt may be not finalised yet
    ///
    /// * Final
    /// Transaction is included into finalised block +
    /// Execution of transaction receipts is finalised
    pub async fn transact_advanced(
        self,
        wait_until_str: &str,
    ) -> Result<RpcTransactionResponse, Box<dyn std::error::Error>> {
        let wait_until: TxExecutionStatus =
            serde_json::from_value(serde_json::json!(wait_until_str))?;
        self.provider
            .send_tx(self.signed_transaction, wait_until)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// Returns transaction hash for a given signed transaction
    pub fn get_transaction_hash(self) -> Result<CryptoHash, Box<dyn std::error::Error>> {
        Ok(self.signed_transaction.get_hash())
    }
}

/// Represents a NEAR account, encapsulating account ID, signer, and provider for blockchain interaction.
pub struct Account {
    pub account_id: AccountId,
    pub signer: Arc<dyn Signer>,     // Use your Signer abstraction
    pub provider: Arc<dyn Provider>, // Use your Provider abstraction
}

/// Represents the balance details of a NEAR account.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AccountBalance {
    pub total: String,
    pub state_staked: String,
    pub staked: String,
    pub available: String,
}

impl Account {
    /// Constructs a new `Account` instance.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The unique account identifier on the NEAR blockchain.
    /// * `signer` - A signer instance for signing transactions.
    /// * `provider` - A provider instance for interacting with the blockchain.
    ///
    /// # Returns
    ///
    /// A new `Account` instance.
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

    /// Prepares a `TransactionBuilder` for constructing a transaction.
    ///
    /// # Arguments
    ///
    /// * `receiver_id` - The account ID of the transaction's receiver.
    ///
    /// # Returns
    ///
    /// A result containing a `TransactionBuilder` instance or an error if fetching the nonce or block hash failed.
    async fn get_transaction_builder(
        &self,
        receiver_id: &AccountId,
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
            receiver_id.clone(),
            nonce + 1,
            block_hash,
        );
        Ok(signed_tx)
    }

    async fn signed_transaction(
        &self,
        actions: &mut ActionBuilder,
        receiver_id: &AccountId,
    ) -> Result<SignedTransaction, Box<dyn std::error::Error>> {
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
            receiver_id.clone(),
            nonce + 1,
            block_hash,
        )
        .set_action(actions.clone_builder())
        .sign_transaction(&*self.signer);
        Ok(signed_tx)
    }

    /// Fetches the current nonce for an account's access key.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account ID for which to fetch the nonce.
    /// * `public_key` - The public key of the access key.
    ///
    /// # Returns
    ///
    /// A result containing the nonce or an error if the query failed.
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

    /// Creates a sub account for the signer account id.
    ///
    /// # Arguments
    ///
    /// * `new_account_id` - The new account ID you want to create. As it will be a sub account of the signer, your new_account_id should be of the form *.signer_account_id.near/testnet
    /// * `public_key` - The public key for the new account.
    /// * `amount` - Initial balance of the new account
    ///
    /// # Returns
    ///
    /// A final execution outcome of the transaction.
    ///
    /// # Note: The accounts created by this function will be of the form *.signer_account_id.near/testnet
    pub async fn create_account(
        &self,
        new_account_id: &AccountId,
        public_key: PublicKey,
        amount: Balance,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let mut builder = ActionBuilder::new();
        let actions = builder
            .set_create_account()
            .set_transfer(amount)
            .set_add_key(public_key.clone(), full_access_key());
        // .clone_builder();

        // let signed_tx = &self
        //     .get_transaction_builder(new_account_id)
        //     .await?
        //     .set_action(actions)
        //     .sign_transaction(&*self.signer); // Sign the transaction

        let signed_tx = &self.signed_transaction(actions, new_account_id).await?;

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx.clone()).await?;
        Ok(transaction_result)
    }

    /// Adds a full or function call access key to an account
    ///
    /// # Arguments
    ///
    /// * `public_key` - The new access key you want to add.
    /// * `allowance` - The allowance this new key can use
    /// * `contract_id` - Incase of function call access key, define the contract the key has access to.
    /// * `method_names` - Incase of function call access key, which define names of methods which the key will have access to. Passing an empty array [] gives you access to call functions.
    ///
    /// # Returns
    ///
    /// A final execution outcome of the transaction.
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
                    return Err("No method_names argument provided for function call access keys. You should at-least provide an empty vector.".into());
                }
            }
            None => full_access_key(),
        };

        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(&self.account_id)
            .await?
            .add_key(public_key, access_key)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await;
        match transaction_result {
            Ok(transaction_result) => Ok(transaction_result),
            Err(err) => Err(Box::new(err)),
        }
    }

    /// Delete a key from an account
    ///
    /// # Arguments
    ///
    /// * `public_key` - The access key you want to delete.
    ///
    /// # Returns
    ///
    /// A final execution outcome of the transaction.
    pub async fn delete_key(
        &self,
        public_key: PublicKey,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(&self.account_id)
            .await?
            .delete_key(public_key)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await;
        match transaction_result {
            Ok(transaction_result) => Ok(transaction_result),
            Err(err) => Err(Box::new(err)),
        }
    }

    /// Deploys a contract to the account associated with this `Account` instance.
    ///
    /// # Arguments
    ///
    /// * `byte_code` - The compiled smart contract code as a vector of bytes.
    ///
    /// # Returns
    ///
    /// A `Result` containing the final execution outcome of the contract deployment or an error if the operation fails.
    pub async fn deploy_contract(
        &self,
        byte_code: &[u8],
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(&self.account_id)
            .await?
            .deploy_contract(byte_code)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await;
        match transaction_result {
            Ok(transaction_result) => Ok(transaction_result),
            Err(err) => Err(Box::new(err)),
        }
    }

    /// Deletes the specified account and transfers any remaining tokens to the beneficiary account.
    ///
    /// # Arguments
    ///
    /// * `beneficiary_id` - The account ID to which the remaining balance will be transferred.
    ///
    /// # Returns
    ///
    /// A `Result` containing the final execution outcome of the account deletion or an error if the operation fails.
    pub async fn delete_account(
        &self,
        beneficiary_id: AccountId,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(&self.account_id)
            .await?
            .delete_account(beneficiary_id)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await;
        match transaction_result {
            Ok(transaction) => Ok(transaction),
            Err(err) => Err(Box::new(err)),
        }
    }

    /// Transfers a specified amount of NEAR tokens from this account to another account.
    ///
    /// # Arguments
    ///
    /// * `receiver_id` - The account ID of the recipient.
    /// * `amount` - The amount of NEAR tokens to transfer, in yoctoNEAR.
    ///
    /// # Returns
    ///
    /// A `Result` containing the final execution outcome of the transfer or an error if the operation fails.
    ///
    pub async fn send_money(
        &self,
        receiver_id: &AccountId,
        amount: Balance,
    ) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(receiver_id)
            .await?
            .transfer(amount)
            .sign_transaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await;
        match transaction_result {
            Ok(transaction_result) => Ok(transaction_result),
            Err(err) => Err(Box::new(err)),
        }
    }

    /// Calls a function on a smart contract deployed on the NEAR blockchain.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The account ID of the contract.
    /// * `method_name` - The name of the function to call.
    /// * `args` - The arguments to the function call, serialized into a JSON `Value`.
    /// * `gas` - The amount of gas to attach to the call.
    /// * `deposit` - The amount of NEAR tokens to transfer to the contract, in yoctoNEAR.
    ///
    /// # Returns
    ///
    /// A `Result` containing the TransactionSender consisting of the signed transaction and the provider(sends transactions to the blockchain)) for the function call or an error if the operation fails.
    pub async fn function_call(
        &self,
        contract_id: &AccountId,
        method_name: String,
        args: Value,
        gas: Gas,
        deposit: Balance,
    ) -> Result<TransactionSender, Box<dyn std::error::Error>> {
        // Serialize the JSON to a Vec<u8>
        let args = serde_json::to_vec(&args)?;

        // Use TransactionBuilder to construct the transaction
        let signed_tx = self
            .get_transaction_builder(contract_id)
            .await?
            .function_call(method_name, args, gas, deposit)
            .sign_transaction(&*self.signer); // Sign the transaction

        // To-do. Needs error handling here.
        Ok(TransactionSender::new(signed_tx, self.provider.clone()))
    }
}

/// Calls a view function on a contract deployed on the NEAR blockchain.
///
/// View functions are read-only and do not modify state. They're free to call.
///
/// # Arguments
///
/// * `contract_id` - The account ID of the contract.
/// * `method_name` - The name of the view function to call.
/// * `args` - The arguments to the function call, typically in the form of a serialized byte array (`FunctionArgs`).
///
/// # Returns
///
/// A `Result` containing the result of the function call or an error if the operation fails.
pub async fn view_function(
    provider: Arc<dyn Provider>,
    contract_id: AccountId,
    method_name: String,
    args: Value,
) -> Result<near_primitives::views::CallResult, Box<dyn std::error::Error>> {
    let args_vec = serde_json::to_vec(&args)?.into();

    let query_request = QueryRequest::CallFunction {
        account_id: contract_id.clone(),
        method_name: method_name.clone(),
        args: args_vec,
    };

    // Send the query to the NEAR blockchain
    let response: RpcQueryResponse = provider.query(query_request).await?;

    if let QueryResponseKind::CallResult(result) = response.kind {
        Ok(result)
    } else {
        Err("Unexpected response kind".into())
    }
}

/// Queries the state of a contract on the NEAR blockchain using a key prefix.
///
/// This method allows you to inspect the storage of a contract, filtered by a key prefix.
///
/// # Arguments
///
/// * `provider` - The provider through which to query the blockchain.
/// * `contract_id` - The account ID of the contract whose state is being queried.
/// * `prefix` - An optional key prefix to filter the state by. If `None`, all state is returned.
///
/// # Returns
///
/// A `Result` containing the contract's state filtered by the specified prefix, or an error if the query fails.
pub async fn view_state(
    provider: Arc<dyn Provider>,
    contract_id: AccountId,
    prefix: Option<String>,
) -> Result<near_primitives::views::ViewStateResult, Box<dyn std::error::Error>> {
    let prefix_op = prefix.unwrap_or("".to_string());
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

/// Retrieves the list of access keys for a given account.
///
/// # Arguments
///
/// * `provider` - The provider through which to query the blockchain.
/// * `account_id` - The account ID for which to retrieve access keys.
///
/// # Returns
///
/// A `Result` containing a list of access keys for the specified account, or an error if the operation fails.
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

/// Retrieves the state of an account on the NEAR blockchain.
///
/// # Arguments
///
/// * `provider` - The provider through which to query the blockchain.
/// * `account_id` - The account ID whose state is being queried.
///
/// # Returns
///
/// A `Result` containing the state of the specified account, or an error if the query fails.
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

/// Retrieves the balance details of a specific account on the NEAR blockchain.
///
/// # Arguments
///
/// * `provider` - The provider through which to query the blockchain.
/// * `account_id` - The account ID whose balance details are being queried.
///
/// # Returns
///
/// A `Result` containing the balance details of the account, structured as `AccountBalance`, or an error if the query fails.
pub async fn get_account_balance(
    provider: Arc<dyn Provider>,
    account_id: AccountId,
) -> Result<AccountBalance, Box<dyn std::error::Error>> {
    // Assuming `experimental_protocol_config` and `state` are async functions you can call on the provider
    let block_reference = BlockReference::Finality(Finality::Final);
    let protocol_config = provider
        .experimental_protocol_config(block_reference)
        .await?;
    // let cost_per_byte = BigInt::from(protocol_config.runtime_config.storage_amount_per_byte);

    let state = state(provider, account_id).await?;

    let staked = state.locked;
    let state_staked = protocol_config
        .runtime_config
        .storage_amount_per_byte
        .mul(state.storage_usage as u128);
    let total_balance = staked.add(state.amount);

    let available_balance = if staked.ge(&state_staked) {
        total_balance.sub(staked)
    } else {
        total_balance.sub(state_staked)
    };

    // Convert BigInt to String for the struct. Handle potential conversion errors as needed
    Ok(AccountBalance {
        total: total_balance.to_string(),
        state_staked: state_staked.to_string(),
        staked: staked.to_string(),
        available: available_balance.to_string(),
    })
}
