use near_primitives::types::AccountId;
use near_primitives::views::QueryRequest;

use near_providers::FastNearHTTPClient;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let near_fast_client = FastNearHTTPClient::new("https://rpc.web4.testnet.page");
    let account_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;
    let query_req = QueryRequest::ViewCode { account_id };
    let response = near_fast_client.contract_methods(query_req).await;

    println!("response: {:#?}", response);

    Ok(())
}
