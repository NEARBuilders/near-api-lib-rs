use near_accounts::Account;
use near_crypto::InMemorySigner;
use near_primitives::types::Balance;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let receiver_account_id: AccountId =
        utils::input("Enter the account name of receiver account ")?.parse()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    // Amount to transfer to the receiver account
    let amount: Balance = 10_000_000_000_000; // Example amount in yoctoNEAR

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);
    // Call create_account
    let result = account
        .send_money(receiver_account_id.clone(), amount)
        .await;

    println!("response: {:#?}", result);

    Ok(())
}
