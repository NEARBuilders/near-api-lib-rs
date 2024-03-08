use near_api::JsonRpcProvider;
use std::sync::Arc;
use near_api::accounts::{get_access_key};
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let account_id: AccountId = "near-api-rs.testnet".parse::<AccountId>()?;

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));

    let result = get_access_key(provider, account_id).await;

    println!("response: {:#?}", result);

    Ok(())
}
