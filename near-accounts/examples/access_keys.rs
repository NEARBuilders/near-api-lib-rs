use near_providers::JsonRpcProvider;
use std::sync::Arc;
use near_crypto::InMemorySigner;
use near_primitives::types::Balance;
use near_crypto::{Signer};
use near_accounts::Account;
mod utils;
use near_primitives::types::AccountId;

async fn add_full_access() -> Result<(), Box<dyn std::error::Error>> {
    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);
    
    let new_secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
    
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);
    
    let result = account.add_key(new_secret_key.public_key(), None, None, None).await;

    println!("response: {:#?}", result);

    println!("=============================================================");
    println!("Full access Public Key: {}", new_secret_key.public_key());
    println!("Secret Key: {}", new_secret_key);
    println!("-------------------------------------------------------------");

    Ok(())
}

async fn add_function_call_key() -> Result<(), Box<dyn std::error::Error>> {
    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);
    
    let new_secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
    
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);

    let allowance: Balance = 1_000_000_000_000_000_000_000_000; // Example amount in yoctoNEAR
    let contract_id = "contract.near-api-rs.testnet".to_string();
    //let method_names = vec!["set_status".to_string()];
    //let method_names = vec![];

    let result = account.add_key(new_secret_key.public_key(), Some(allowance), Some(contract_id), None).await;

    println!("response: {:#?}", result);

    println!("=============================================================");
    println!("Function access Public Key: {}", new_secret_key.public_key());
    println!("Secret Key: {}", new_secret_key);
    println!("-------------------------------------------------------------");

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("Running add full access key Example...");
    //add_full_access().await?;

    println!("Running add function call access key Example...");
    add_function_call_key().await?;

    Ok(())
}
