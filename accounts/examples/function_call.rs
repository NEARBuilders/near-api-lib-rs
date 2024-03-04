use near_primitives::types::Gas;
use providers::Provider;
use providers::JsonRpcProvider;
use std::sync::Arc;
use near_crypto::InMemorySigner;
use near_primitives::types::Balance;
use near_crypto::{Signer};
use accounts::Account;
mod utils;
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let contract_id: AccountId = utils::input("Enter the Contract ID: ")?.parse()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);
        
    // Amount to transfer to the new account
    let gas: Gas = 300_000_000_000_000; // Example amount in yoctoNEAR

    let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);
    let method_name = "set_status".to_string();
    let args = ["status working"];

    // Call create_account
    let result = account.function_call(contract_id, method_name, args, gas, 0).await;


    println!("response: {:#?}", result);

    Ok(())
}
