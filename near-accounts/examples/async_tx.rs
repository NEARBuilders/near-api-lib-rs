//! This example uses the transact_advance method to send  transaction and check its status
mod example_config;
use near_primitives::views::TxExecutionStatus;
use near_primitives::{types::Gas, views::FinalExecutionOutcomeViewEnum};
use near_providers::jsonrpc_primitives::types::transactions::TransactionInfo;
use near_providers::JsonRpcProvider;
use near_providers::Provider;
use std::sync::Arc;
mod utils;
use near_accounts::Account;
use near_crypto::{InMemorySigner, SecretKey};
use near_primitives::types::AccountId;
use serde_json::json;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // Get test account and rpc details.
    let config = example_config::get_test_config();

    //Create a signer
    let signer_account_id: AccountId = config.near_account.account_id.parse().unwrap();
    let signer_secret_key: SecretKey = config.near_account.secret_key.parse().unwrap();
    let signer = Arc::new(InMemorySigner::from_secret_key(
        signer_account_id.clone(),
        signer_secret_key,
    ));

    //Creat a Provider
    let provider = Arc::new(JsonRpcProvider::new(config.rpc_testnet_endpoint.as_str()));

    //Create an Account object
    let account = Account::new(signer_account_id, signer, provider.clone());

    //Create argumements for function_call
    //Contract id, method_name, method args, gas and deposit.
    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;
    let method_name = "set_status".to_string();
    let args_json = json!({"message": "working1"});
    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR

    //Create a Transaction Sender Object;
    let transaction_sender = account
        .function_call(&contract_id, method_name, args_json, gas, 0)
        .await?;
    //Get the transaction hash to query the chain later.
    let tx_hash = transaction_sender.clone().get_transaction_hash().unwrap();

    //Send the transaction
    //Different Wait_until values:  None, Included, ExecutedOptimistic, IncludedFinal, Executed, Final
    let result = transaction_sender.transact_advanced("NONE").await;
    match result {
        Ok(res) => match &res.final_execution_outcome {
            //Final Execution outcome for finality NONE would always be empty.
            Some(FinalExecutionOutcomeViewEnum::FinalExecutionOutcome(outcome)) => {
                println!("Final Execution outcome: {:?}", outcome);
                println!("Final Execution outcome: {:?}", outcome.transaction);
            }
            Some(FinalExecutionOutcomeViewEnum::FinalExecutionOutcomeWithReceipt(
                outcome_receipt,
            )) => {
                println!("Final Execution outcome_receipts: {:?}", outcome_receipt)
            }
            None => println!("No Final execution outcome."),
        },
        Err(err) => println!("Error: {:#?}", err),
    }

    //Check the status of the transaction now.
    let transaction_info = TransactionInfo::TransactionId {
        tx_hash,
        sender_account_id: account.account_id,
    };
    let wait_until = TxExecutionStatus::ExecutedOptimistic;
    let tx_status = provider.tx_status(transaction_info, wait_until).await;

    match tx_status {
        Ok(response) => {
            println!("response: {:#?}", response);
        }
        Err(err) => println!("Error: {:#?}", err),
    }
    Ok(())
}
