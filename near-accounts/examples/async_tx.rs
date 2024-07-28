//! This example uses the transact_advance method to send  transaction and check its status
mod example_config;
use near_primitives::views::TxExecutionStatus;
use near_primitives::{types::Gas, views::FinalExecutionOutcomeViewEnum};
use near_providers::jsonrpc_primitives::types::transactions::TransactionInfo;
use near_providers::JsonRpcProvider;
use near_providers::Provider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;
use serde_json::json;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;

    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));

    let account = example_config::create_account();
    let method_name = "set_status".to_string();

    let args_json = json!({"message": "working1"});

    let transaction_sender = account
        .function_call(&contract_id, method_name, args_json, gas, 0)
        .await?;

    let tx_hash = transaction_sender.clone().get_transaction_hash().unwrap();

    let t1 = time::Instant::now();
    //Different Wait_until values:  None, Included, ExecutedOptimistic, IncludedFinal, Executed, Final
    let result = transaction_sender.transact_advanced("NONE").await;
    let t2 = time::Instant::now();
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

    time::sleep(time::Duration::from_secs(5)).await;

    let t3 = time::Instant::now();
    let tx_status = provider.tx_status(transaction_info, wait_until).await;
    let t4 = time::Instant::now();

    match tx_status {
        Ok(response) => {
            //println!("response gotten after: {}s", delta);
            println!("response: {:#?}", response);
        }
        Err(err) => println!("Error: {:#?}", err),
    }

    println!("Time taken for async request: {:?}", t2 - t1);
    println!("Time taken for status request: {:?}", t4 - t3);
    Ok(())
}
