use near_accounts::Account;
use near_crypto::InMemorySigner;
use near_primitives::types::Gas;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::{AccountId, Balance};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    //To-do, implement account exist check.
    let new_account_id: AccountId = utils::input("Enter new account name: ")?.parse()?;

    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    // Amount to transfer to the new account
    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR
    let amount: Balance = 10_000_000_000_000_000_000_000; // Example amount in yoctoNEAR

    let new_secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);

    let contract_id: AccountId = "testnet".parse::<AccountId>()?;
    let method_name = "create_account".to_string();

    let args_json = json!({
        "new_account_id": new_account_id,
        "new_public_key": new_secret_key.public_key()
    });
    // Serialize the JSON to a Vec<u8>
    // .into will convert it into Value type.
    let args = serde_json::to_vec(&args_json)?.into();

    let result = account
        .function_call(contract_id, method_name, args, gas, amount)
        .await;

    println!("response: {:#?}", result);
    println!("New Account ID: {}", new_account_id);
    println!("Secret Key: {}", new_secret_key);

    Ok(())
}

//Test transaction - https://testnet.nearblocks.io/txns/81kqpntzQYWpVaG4TGhfcdJzeWVQLKFwTKCf4QMDcJbR#execution
