use near_accounts::Account;
mod example_config;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let account: Account = example_config::create_account();

    let wasm_code = example_config::read_wasm_file()?;

    let response = account.deploy_contract(&wasm_code).await;

    match response {
        Ok(res) => {
            println!("transaction: {:#?}", res.transaction);
            println!("status: {:#?}", res.status);
            println!("receipts_outcome {:#?}", res.transaction_outcome);
        }
        Err(err) => println!("Error: {:#?}", err),
    }
    Ok(())
}
