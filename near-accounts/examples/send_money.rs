mod example_config;
use near_accounts::Account;
use near_crypto::{InMemorySigner, SecretKey};
use near_primitives::types::Balance;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;

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

    let receiver_account_id: AccountId =
        utils::input("Enter the account name of receiver account ")?.parse()?;

    // Amount to transfer to the receiver account
    let amount: Balance = 10_000_000_000; // Example amount in yoctoNEAR

    let result = account.send_money(&receiver_account_id, amount).await;

    println!("response: {:#?}", result);

    Ok(())
}
