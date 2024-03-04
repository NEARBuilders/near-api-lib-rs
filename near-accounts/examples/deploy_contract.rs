use providers::Provider;
use providers::JsonRpcProvider;
use std::sync::Arc;
use near_crypto::InMemorySigner;
use near_primitives::types::Balance;
use near_crypto::{Signer};
use accounts::Account;
mod utils;
use near_primitives::types::AccountId;
use std::io;
use std::fs::File;
use std::io::Read;


// fn read_wasm_file() ->  Result<String, Box<dyn std::error::Error>> {
//     let file_path = "accounts/examples/contract-wasm/status_message.wasm";
//     let mut file = match File::open(file_path) {
//         Ok(file) => file,
//         Err(e) => {
//             eprintln!("Error opening file: {}", e);
//             Ok();
//         }
//     };

//     // Read the contents of the file into a string
//     let mut contents = String::new();
//     match file.read_to_string(&mut contents) {
//         Ok(_) => {
//             println!("File contents:\n{}", contents);
//         }
//         Err(e) => {
//             eprintln!("Error reading file: {}", e);
//             return;
//         }
//     }
//     Ok(contents)
// }

// Corrected to read as binary and return Vec<u8>
fn read_wasm_file() -> io::Result<Vec<u8>> {
    let file_path = "accounts/examples/contract-wasm/status_message.wasm";
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);

    let wasm_code = read_wasm_file()?;
    
    let result = account.deploy_contract(wasm_code).await;


    println!("response: {:#?}", result);

    Ok(())
}


// New Account ID: contract.near-api-rs.testnet
// Secret Key: ed25519:2ytXTGiGkMfpdW1JujZNebTCKRFQAFqq89fbkq9akBXy8kqqfhTqUCzmDexeNrCD1sjijMATdPWKzyCj9XnteFgN
// Public Key: ed25519:4mKgZ8e9PgSJvrVtJ4omkgmPR7ssgpCPGc2N5AGWkhfQ
// Deposit: 10000000000000000000000