use near_crypto::InMemorySigner;
use near_providers::Provider;
use near_providers::JsonRpcProvider;
use near_crypto::{Signer, KeyFile};

use std::sync::Arc;

pub struct Connection {
    network_id: String,
    provider: Arc<dyn Provider>,
    signer: Arc<dyn Signer>,
    jsvm_account_id: String,
}

pub struct  ConnectionConfig {
    rpc_endpoint: String,
    key_file: KeyFile,
    network_id: String,
    jsvm_account_id: String,
}

fn get_provider(rpc_endpoint: &str) -> Arc<dyn Provider> {
    Arc::new(JsonRpcProvider::new(rpc_endpoint))
}

fn get_signer(key_file: KeyFile) -> Arc<dyn Signer> {
    Arc::new(InMemorySigner::from(key_file))
}

impl Connection {
    fn from_config(config: ConnectionConfig) -> Connection {
        let provider = get_provider(&config.rpc_endpoint);
        let signer = get_signer(config.key_file);
        Connection {
            network_id: config.network_id,
            provider,
            signer,
            jsvm_account_id: config.jsvm_account_id,
        }
    }
}