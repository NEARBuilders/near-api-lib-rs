use near_crypto::{KeyPair, PublicKey};
use std::collections::HashMap;
use std::sync::Mutex;

pub trait KeyStore {
    fn set_key(&self, network_id: &str, account_id: &str, key_pair: KeyPair) -> Result<(), Box<dyn std::error::Error>>;
    fn get_key(&self, network_id: &str, account_id: &str) -> Result<Option<KeyPair>, Box<dyn std::error::Error>>;
    fn remove_key(&self, network_id: &str, account_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn clear(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_networks(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    fn get_accounts(&self, network_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}
