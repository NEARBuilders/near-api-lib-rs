use near_api::JsonRpcProvider;
use std::sync::Arc;
use near_crypto::InMemorySigner;
use near_api::accounts::{view_state};
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));

    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;
    let prefix = "".to_string() ; //change the prefix to specific key you want from contract's state.

    let result = view_state(provider, prefix, contract_id).await;
    println!("response: {:#?}", result);

    Ok(())  
}
