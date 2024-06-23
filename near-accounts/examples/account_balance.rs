mod example_config;
use near_accounts::accounts::get_account_balance;
use near_primitives::types::AccountId;
use near_providers::JsonRpcProvider;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = example_config::get_test_config();
    let account_id: AccountId = config.near_account.account_id.parse().unwrap();

    let provider = Arc::new(JsonRpcProvider::new(&config.rpc_testnet_endpoint));

    let result = get_account_balance(provider, account_id).await;

    match result {
        Ok(res) => {
            println!("available balance: {:#?}", res.available);
            println!("total balance: {:#?}", res.total);
            println!("state staked {:#?}", res.state_staked);
        }
        Err(err) => println!("Error: {:#?}", err),
    }

    Ok(())
}
