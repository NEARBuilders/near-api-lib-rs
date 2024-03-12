use near_providers::JsonRpcProvider;
use std::sync::Arc;
use near_crypto::{InMemorySigner, PublicKey};
use near_accounts::Account;
mod utils;
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);
        
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);

    let public_key: PublicKey = "ed25519:EohEtHT8Dt8jURC3DcJ661hWCx6ExPRtDV82FpT4jfNB".parse::<PublicKey>()?;
    
    let result = account.delete_key(public_key).await;


    println!("response: {:#?}", result);

    Ok(())
}
