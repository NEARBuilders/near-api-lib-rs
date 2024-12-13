mod example_config;
use near_accounts::Account;
use near_crypto::InMemorySigner;
use near_crypto::SecretKey;
use near_primitives::types::Gas;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = example_config::get_test_config();
    let signer_account_id: AccountId = config.near_account.account_id.parse().unwrap();
    let signer_secret_key: SecretKey = config.near_account.secret_key.parse().unwrap();

    let contract_id: AccountId = config.contract_account.account_id.parse().unwrap();
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR

    let provider = Arc::new(JsonRpcProvider::new(config.rpc_testnet_endpoint.as_str()));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);
    let method_name = "set_status".to_string();

    let args_json = json!({"message": "working1"});

    let result = account
        .function_call(&contract_id, method_name, args_json, gas, 0)
        .await?
        .transact()
        .await;
    println!("response: {:#?}", result);

    Ok(())
}
