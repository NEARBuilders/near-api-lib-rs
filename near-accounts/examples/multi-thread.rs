use near_accounts::Account;
use near_crypto::{InMemorySigner, SecretKey};
use near_primitives::{types::Gas, views::FinalExecutionOutcomeViewEnum};
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::{AccountId, Balance};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialization and user inputs as before

    let signer_account_id: AccountId = "near-api-rs.testnet".parse::<AccountId>()?;
    let signer_secret_key = "ed25519:29nYmQCZMsQeYtztXZzm57ayQt2uBHXdn2SAjK4ccMGSQaNUFNJ7Aoteno81eKTex9cGBbk1FuDuqJRsdzx34xDY".parse::<SecretKey>()?;

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    //let provider = JsonRpcProvider::new("https://rpc.testnet.near.org");
    let signer = Arc::new(InMemorySigner::from_secret_key(
        signer_account_id.clone(),
        signer_secret_key,
    ));
    let account = Account::new(signer_account_id, signer.clone(), provider.clone());

    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;

    // This spawns a new asynchronous task to handle account creation
    let handle = tokio::spawn(async move {
        let method_name = "set_status".to_string();
        let args_json = json!({"message": "working1"});

        // Amount to transfer to the new account
        let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR
        let amount: Balance = 10_000_000_000_000_000_000_000; // Example amount in yoctoNEAR

        let result = account
            .function_call(&contract_id, method_name, args_json, gas, amount)
            .await
            .expect("Reason")
            .transact()
            .await;

        match result {
            Ok(res) => match &res.final_execution_outcome {
                Some(FinalExecutionOutcomeViewEnum::FinalExecutionOutcome(outcome)) => {
                    println!("Final Execution outcome: {:#?}", outcome);
                }
                Some(FinalExecutionOutcomeViewEnum::FinalExecutionOutcomeWithReceipt(
                    outcome_receipt,
                )) => {
                    println!(
                        "Final Execution outcome with receipt: {:#?}",
                        outcome_receipt
                    );
                }
                None => println!("No Final execution outcome."),
            },
            Err(err) => println!("Error: {:#?}", err),
        }
    });

    // You can do more work here or wait for the handle if needed
    handle.await?;

    Ok(())
}
