use near_accounts::Account;
use near_crypto::InMemorySigner;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let user_account_id: AccountId =
        utils::input("Enter the account name which need to be deleted ")?.parse()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);

    let response = account.delete_account(user_account_id.clone()).await;

    match response {
        Ok(res) => {
            println!("transaction: {:#?}", res.transaction);
            println!("status: {:#?}", res.status);
            println!("receipts_outcome {:#?}", res.transaction_outcome);
        }
        Err(err) => println!("Error: {:#?}", err),
    }
    Ok(())
}
