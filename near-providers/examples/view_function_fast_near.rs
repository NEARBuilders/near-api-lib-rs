use near_primitives::types::AccountId;
use near_primitives::views::QueryRequest;

use near_providers::FastNearHTTPClient;
use serde_json::json;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let near_fast_client = FastNearHTTPClient::new("https://rpc.web4.testnet.page");
    let account_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;

    let args_json = json!({"account_id": "near-api-rs.testnet"});
    let method_name = "get_status".to_string();
    let args_vec = serde_json::to_vec(&args_json)?.into();

    let query_req = QueryRequest::CallFunction {
        account_id,
        method_name,
        args: args_vec,
    };
    let response = near_fast_client.view_function::<String>(query_req).await;

    println!("response: {:#?}", response);

    // Supporting json data type as input and output
    let near_fast_client = FastNearHTTPClient::new("https://rpc.web4.near.page");
    let account_id: AccountId = "lands.near".parse::<AccountId>()?;

    let args_json = json!({"request.json": {"path":"/"}});
    let method_name = "web4_get".to_string();
    let args_vec = serde_json::to_vec(&args_json)?.into();

    let query_req = QueryRequest::CallFunction {
        account_id,
        method_name,
        args: args_vec,
    };
    let response = near_fast_client
        .view_function::<serde_json::Value>(query_req)
        .await?;

    println!("response: {:#?}", response.to_string());

    Ok(())
}
