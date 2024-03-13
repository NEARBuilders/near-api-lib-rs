use near_accounts::accounts::get_access_key;
use near_primitives::types::AccountId;
use near_providers::JsonRpcProvider;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let account_id: AccountId = "near-api-rs.testnet".parse::<AccountId>()?;

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));

    let result = get_access_key(provider, account_id).await;

    println!("response: {:#?}", result);

    Ok(())
}
