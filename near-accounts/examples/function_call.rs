use near_accounts::Account;
use near_crypto::InMemorySigner;
use near_crypto::SecretKey;
use near_primitives::types::Gas;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;
use serde_json::json;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = "near-api-rs.testnet".parse::<AccountId>()?;
    let signer_secret_key = "ed25519:29nYmQCZMsQeYtztXZzm57ayQt2uBHXdn2SAjK4ccMGSQaNUFNJ7Aoteno81eKTex9cGBbk1FuDuqJRsdzx34xDY".parse::<SecretKey>()?;
    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);
    let method_name = "set_status".to_string();

    let args_json = json!({"message": "working1"});

    let t1 = time::Instant::now();
    let result = account
        .function_call(&contract_id, method_name, args_json, gas, 0)
        .await?
        .transact()
        .await;
    let t2 = time::Instant::now();
    println!("response: {:#?}", result);
    println!("Time taken: {:?}",t2-t1);

    Ok(())
}
