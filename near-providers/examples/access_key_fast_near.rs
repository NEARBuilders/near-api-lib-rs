use near_crypto::PublicKey;
use near_primitives::types::AccountId;
use near_primitives::views::QueryRequest;

use near_providers::FastNearHTTPClient;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let near_fast_client = FastNearHTTPClient::new("https://rpc.web4.near.page");
    let account_id: AccountId = "vlad.near".parse::<AccountId>()?;
    let public_key = "ed25519:JBHUrhF61wfScUxqGGRmfdJTQYg8MzRr5H8pqMMjqygr".parse::<PublicKey>()?;
    let query_req = QueryRequest::ViewAccessKey {
        account_id,
        public_key,
    };
    let response = near_fast_client.access_key(query_req).await;

    println!("response: {:#?}", response);

    Ok(())
}
