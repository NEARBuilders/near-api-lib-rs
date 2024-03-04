use near_crypto::{KeyPair, PublicKey};
use std::collections::HashMap;
use std::sync::Mutex;


use crate::KeyStore;

pub struct InMemoryKeyStore {
    keys: Mutex<HashMap<String, KeyPair>>,
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        Self {
            keys: Mutex::new(HashMap::new()),
        }
    }
}

impl KeyStore for InMemoryKeyStore {
    fn set_key(&self, network_id: &str, account_id: &str, key_pair: KeyPair) -> Result<(), Box<dyn std::error::Error>> {
        let mut keys = self.keys.lock().unwrap();
        keys.insert(format!("{}:{}", account_id, network_id), key_pair);
        Ok(())
    }

    fn get_key(&self, network_id: &str, account_id: &str) -> Result<Option<KeyPair>, Box<dyn std::error::Error>> {
        let keys = self.keys.lock().unwrap();
        Ok(keys.get(&format!("{}:{}", account_id, network_id)).cloned())
    }

    fn remove_key(&self, network_id: &str, account_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut keys = self.keys.lock().unwrap();
        keys.remove(&format!("{}:{}", account_id, network_id));
        Ok(())
    }

    fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut keys = self.keys.lock().unwrap();
        keys.clear();
        Ok(())
    }

    fn get_networks(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let keys = self.keys.lock().unwrap();
        let networks: Vec<String> = keys.keys().map(|k| k.split(':').last().unwrap().to_string()).collect();
        Ok(networks)
    }

    fn get_accounts(&self, network_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let keys = self.keys.lock().unwrap();
        let accounts: Vec<String> = keys.keys().filter_map(|k| {
            let parts: Vec<&str> = k.split(':').collect();
            if parts[1] == network_id {
                Some(parts[0].to_string())
            } else {
                None
            }
        }).collect();
        Ok(accounts)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import the contents of the outer module
    use near_crypto::{KeyPair, SecretKey, PublicKey, KeyType};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_in_memory_key_store() -> Result<(), Box<dyn std::error::Error>> {
        let key_store = Arc::new(InMemoryKeyStore::new());

        let network_id = "testnet";
        let account_id = "test.near";
        let key_pair = KeyPair::from_secret_key(SecretKey::from_seed(KeyType::ED25519, "test"));

        // Test setKey
        key_store.set_key(network_id, account_id, key_pair.clone()).await?;
        assert!(key_store.get_key(network_id, account_id).await?.is_some());

        // Test getKey
        let retrieved_key_pair = key_store.get_key(network_id, account_id).await?.unwrap();
        assert_eq!(key_pair, retrieved_key_pair);

        // Test removeKey
        key_store.remove_key(network_id, account_id).await?;
        assert!(key_store.get_key(network_id, account_id).await?.is_none());

        // Test clear
        key_store.set_key(network_id, account_id, key_pair).await?;
        key_store.clear().await?;
        assert!(key_store.get_key(network_id, account_id).await?.is_none());

        Ok(())
    }
}

