mod example_config;
use near_accounts::Account;
use near_crypto::{InMemorySigner, SecretKey};
use near_primitives::types::Balance;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;

async fn add_full_access() -> Result<(), Box<dyn std::error::Error>> {
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
    let account = Account::new(signer_account_id, signer, provider);

    //Generate a secret Key for new access key
    let new_secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);

    //Call add_key function on an Account
    let result = account
        .add_key(new_secret_key.public_key(), None, None, None)
        .await;

    println!("response: {:#?}", result);

    println!("=============================================================");
    println!("Full access Public Key: {}", new_secret_key.public_key());
    println!("Secret Key: {}", new_secret_key);
    println!("-------------------------------------------------------------");

    Ok(())
}

async fn add_function_call_key() -> Result<(), Box<dyn std::error::Error>> {
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
    let account = Account::new(signer_account_id, signer, provider);

    // Create a secret key for the new function call access key
    let new_secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);

    let allowance: Balance = 1_000_000_000_000_000_000_000_000; // Example amount in yoctoNEAR
    let contract_id = "contract.near-api-rs.testnet".to_string();
    //Create an array of methods
    let method_names = vec!["set_status".to_string()];
    // An empty array means access to all functions
    //let method_names = vec![];

    let result = account
        .add_key(
            new_secret_key.public_key(),
            Some(allowance),
            Some(contract_id),
            Some(method_names),
        )
        .await;

    println!("response: {:#?}", result);

    println!("=============================================================");
    println!(
        "Function access Public Key: {}",
        new_secret_key.public_key()
    );
    println!("Secret Key: {}", new_secret_key);
    println!("-------------------------------------------------------------");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("Running add full access key Example...");
    add_full_access().await?;

    println!("Running add function call access key Example...");
    add_function_call_key().await?;

    Ok(())
}
