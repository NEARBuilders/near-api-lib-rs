mod example_config;
use near_accounts::Account;
use near_crypto::{InMemorySigner, SecretKey};
use near_primitives::{types::Gas, views::FinalExecutionOutcomeViewEnum};
mod utils;
use near_primitives::types::{AccountId, Balance};
use near_providers::JsonRpcProvider;
use serde_json::json;
use std::sync::Arc;

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

    //Ask the user for the new Account id.
    let new_account_id: AccountId = utils::input("Enter new account name: ")?.parse()?;
    // Amount to transfer to the new account
    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR
    let amount: Balance = 10_000_000_000_000_000_000_000; // Example amount in yoctoNEAR

    let contract_id: AccountId = "testnet".parse::<AccountId>()?;
    let method_name = "create_account".to_string();

    let new_secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
    let args_json = json!({
        "new_account_id": new_account_id,
        "new_public_key": new_secret_key.public_key()
    });

    println!("New Secret key : {}", new_secret_key);
    println!("New Public key: {}", new_secret_key.public_key());

    let result = account
        .function_call(&contract_id, method_name, args_json, gas, amount)
        .await?
        .transact()
        .await;

    match result {
        Ok(res) => match &res.final_execution_outcome {
            Some(FinalExecutionOutcomeViewEnum::FinalExecutionOutcome(outcome)) => {
                println!("Final Execution outcome Status: {:?}", outcome.status);
                println!("Final Execution Transaction: {:?}", outcome.transaction);
            }
            Some(FinalExecutionOutcomeViewEnum::FinalExecutionOutcomeWithReceipt(
                outcome_receipt,
            )) => {
                println!("Final Execution outcome receipt: {:?}", outcome_receipt)
            }
            None => println!("No Final execution outcome."),
        },
        Err(err) => println!("Error: {:#?}", err),
    }

    Ok(())
}

//Test transaction - https://testnet.nearblocks.io/txns/81kqpntzQYWpVaG4TGhfcdJzeWVQLKFwTKCf4QMDcJbR#execution
