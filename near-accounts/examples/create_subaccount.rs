mod example_config;
use near_accounts::Account;
use near_crypto::{InMemorySigner, SecretKey};
mod utils;
use near_primitives::types::{AccountId, Balance};
use near_providers::JsonRpcProvider;
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

    //Ask user for the new account id, it should be of the form something.near-api-rs.testnet
    //or whatever signer account you are using.
    let new_account_id: AccountId =
        utils::input("Enter the account name of new account ")?.parse()?;

    // Amount to transfer to the new account
    let amount: Balance = 10_000_000_000_000_000_000_000; // Example amount in yoctoNEAR

    let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);

    // Call create_account
    let result = account
        .create_account(&new_account_id, new_key_pair.public_key(), amount)
        .await;

    match result {
        Ok(res) => {
            println!("transaction: {:#?}", res.transaction);
            println!("status: {:#?}", res.status);
            println!("receipts_outcome {:#?}", res.transaction_outcome);
        }
        Err(err) => println!("Error: {:#?}", err),
    }

    println!("=============================================================");
    println!("New Account ID: {}", new_account_id);
    println!("    Secret Key: {}", new_key_pair);
    println!("    Public Key: {}", new_key_pair.public_key());
    println!("       Deposit: {}", amount);
    println!("-------------------------------------------------------------");

    Ok(())
}
