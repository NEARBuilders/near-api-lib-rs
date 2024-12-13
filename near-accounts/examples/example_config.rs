use near_accounts::Account;
use near_crypto::{InMemorySigner, SecretKey};
use near_primitives::types::AccountId;
use near_providers::JsonRpcProvider;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[serde()]
pub struct TestConfig {
    pub near_account: AccountConfig,
    pub rpc_testnet_endpoint: String,
    pub contract_account: AccountConfig,
}

#[derive(Debug, Deserialize)]
#[serde()]
pub struct AccountConfig {
    pub account_id: String,
    pub secret_key: String,
    pub public_key: String,
}

pub fn get_test_config() -> TestConfig {
    let the_file = "examples/resources/config/test_config.json";

    let data = fs::read_to_string(the_file).expect("Unable to read file");
    let test_config: TestConfig =
        serde_json::from_str(data.as_str()).expect("JSON was not well-formatted");
    return test_config;
}

#[allow(dead_code)]
pub fn create_account() -> Account {
    let config = get_test_config();
    let signer_account_id: AccountId = config.near_account.account_id.parse().unwrap();
    let signer_secret_key: SecretKey = config.near_account.secret_key.parse().unwrap();
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    let provider = Arc::new(JsonRpcProvider::new(config.rpc_testnet_endpoint.as_str()));
    let signer = Arc::new(signer);

    return Account::new(signer_account_id, signer, provider);
}

#[allow(dead_code)]
pub fn read_wasm_file() -> io::Result<Vec<u8>> {
    let file_path = "examples/resources/contract-wasm/status_message.wasm";
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

#[allow(dead_code)]
fn main() {
    panic!("not a binary")
}
