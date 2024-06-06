use near_crypto::{InMemorySigner, PublicKey, SecretKey};
use near_primitives::transaction::Transaction;
use near_primitives::types::{AccountId, BlockReference, Finality, Gas};
use near_primitives::views::QueryRequest;
use near_providers::types::query::{QueryResponseKind, RpcQueryResponse};
use near_providers::JsonRpcProvider;
use near_providers::Provider;
use near_transactions::action_builder;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let provider = JsonRpcProvider::new("https://rpc.testnet.near.org");

    //create a function call Action
    let method_name = "set_status".to_string();
    let args_json = json!({"message": "meta_transactions"});
    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR
    let args = serde_json::to_vec(&args_json)?;
    let actions = action_builder::ActionBuilder::new()
        .function_call(method_name, args, gas, 0)
        .build();

    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;

    let signer_account_id: AccountId = "near-api-rs.testnet".parse::<AccountId>()?;
    let signer_secret_key = "ed25519:29nYmQCZMsQeYtztXZzm57ayQt2uBHXdn2SAjK4ccMGSQaNUFNJ7Aoteno81eKTex9cGBbk1FuDuqJRsdzx34xDY".parse::<SecretKey>()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    let nonce = fetch_nonce(&signer_account_id, &signer.public_key, &provider).await?;

    //Block hash
    let block_reference = BlockReference::Finality(Finality::Final);
    let block = provider.block(block_reference).await?;
    let block_hash = block.header.hash;

    let tx: Transaction = near_transactions::Transaction {
        signer_id: signer_account_id,
        public_key: signer.clone().public_key,
        nonce: nonce + 1,
        receiver_id: contract_id,
        block_hash,
        actions,
    };

    let signed_tx = tx.sign(&signer);
    let result = provider.send_transaction(signed_tx).await;
    println!("response: {:#?}", result);

    Ok(())
}

pub async fn fetch_nonce(
    account_id: &AccountId,
    public_key: &PublicKey,
    provider: &dyn Provider,
) -> Result<u64, Box<dyn std::error::Error>> {
    let query_request = QueryRequest::ViewAccessKey {
        account_id: account_id.clone(),
        public_key: public_key.clone(),
    };

    // Send the query to the NEAR blockchain
    let response: RpcQueryResponse = provider.query(query_request).await?;

    // Extract the access key view from the response
    if let QueryResponseKind::AccessKey(access_key_view) = response.kind {
        Ok(access_key_view.nonce)
    } else {
        Err("Unexpected response kind".into())
    }
}
