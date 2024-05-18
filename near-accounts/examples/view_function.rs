use near_accounts::accounts::view_function;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));

    let args_json = json!({"account_id": "contract.near-api-rs.testnet"});
    let method_name = "get_status".to_string();

    let result = view_function(provider, contract_id, method_name, args_json).await;

    println!("response: {:#?}", result);

    Ok(())
}
