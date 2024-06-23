//use near_providers::Provider;
mod example_config;
use near_primitives::types::Balance;
mod utils;
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let new_account_id: AccountId =
        utils::input("Enter the account name of new account ")?.parse()?;

    // Amount to transfer to the new account
    let amount: Balance = 10_000_000_000_000_000_000_000; // Example amount in yoctoNEAR

    let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);

    let account = example_config::create_account();
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
