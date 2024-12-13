mod example_config;
use near_crypto::PublicKey;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let account = example_config::create_account();

    let public_key: PublicKey =
        "ed25519:EohEtHT8Dt8jURC3DcJ661hWCx6ExPRtDV82FpT4jfNB".parse::<PublicKey>()?;

    let result = account.delete_key(public_key).await;

    match result {
        Ok(res) => {
            println!("transaction: {:#?}", res.transaction);
            println!("status: {:#?}", res.status);
            println!("receipts_outcome {:#?}", res.transaction_outcome);
        }
        Err(err) => println!("Error: {:#?}", err),
    }

    Ok(())
}
