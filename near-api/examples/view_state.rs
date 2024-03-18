use near_api::accounts::view_state;
use near_api::JsonRpcProvider;
use near_primitives::types::AccountId;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));

    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;

    let result = view_state(provider, contract_id, None).await;
    println!("response: {:#?}", result);

    Ok(())
}
