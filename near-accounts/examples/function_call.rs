use near_primitives::types::Gas;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
use near_crypto::InMemorySigner;
use near_accounts::Account;
mod utils;
use near_primitives::types::AccountId;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);
        
    // Amount to transfer to the new account
    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);
    let method_name = "set_status".to_string();

    let args_json = json!({"message": "working1"});
    // Serialize the JSON to a Vec<u8>
    //let args = serde_json::to_vec(&args_json)?;

    let result = account.function_call(contract_id, method_name, args_json, gas, 0).await;

    println!("response: {:#?}", result);

    Ok(())
}
