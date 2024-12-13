mod example_config;
mod utils;
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let beneficiary_account_id: AccountId =
        utils::input("Enter the account name where you want to transfer current account balance before deleting it")?.parse()?;
    let account = example_config::create_account();

    let response = account.delete_account(beneficiary_account_id.clone()).await;

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
