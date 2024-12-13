mod example_config;
use near_primitives::types::Balance;
mod utils;
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let receiver_account_id: AccountId =
        utils::input("Enter the account name of receiver account ")?.parse()?;

    // Amount to transfer to the receiver account
    let amount: Balance = 10_000_000_000; // Example amount in yoctoNEAR

    let account = example_config::create_account();
    // Call create_account
    let result = account.send_money(&receiver_account_id, amount).await;

    println!("response: {:#?}", result);

    Ok(())
}
