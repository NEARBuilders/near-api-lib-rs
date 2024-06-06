use near_crypto::{InMemorySigner, PublicKey, SecretKey};
use near_primitives::types::{AccountId, BlockReference, Finality, Gas};
use near_primitives::views::QueryRequest;
use near_providers::types::query::{QueryResponseKind, RpcQueryResponse};
use near_providers::JsonRpcProvider;
use near_providers::Provider;
use near_transactions::action_builder;
use near_transactions::delegate_actions::delegate_actions;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let provider = JsonRpcProvider::new("https://rpc.testnet.near.org");

    let inner_signer_account_id: AccountId = "jaswinders.testnet".parse::<AccountId>()?;
    let inner_signer_secret_key = "ed25519:4znkRNvCKhNLospBBGqjkTzxq9DyFtopnhJ8URGC7mWUeYJnZUsZKNw1wUnNEUSAhKXZhqkVc4A8QjkV4LV4N34H".parse::<SecretKey>()?;
    let inner_signer =
        InMemorySigner::from_secret_key(inner_signer_account_id.clone(), inner_signer_secret_key);

    let method_name = "set_status".to_string();
    let args_json = json!({"message": "meta_transactions"});
    let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR

    let args = serde_json::to_vec(&args_json)?;
    let actions = action_builder::ActionBuilder::new()
        .function_call(method_name, args, gas, 0)
        .build();

    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;

    let inner_nonce = fetch_nonce(
        &inner_signer_account_id,
        &inner_signer.public_key,
        &provider,
    )
    .await?;

    //Block hash
    let block_reference = BlockReference::Finality(Finality::Final);
    let block = provider.block(block_reference).await?;
    let block_height = block.header.height;

    let delegate_action = delegate_actions::create_delegate_action(
        inner_signer_account_id.clone(),
        contract_id,
        actions,
        inner_nonce + 1,
        block_height + 100,
        inner_signer.clone().public_key,
    );

    let signed_delegate_action =
        delegate_actions::create_signed_delegate_action(delegate_action, &inner_signer);

    let outer_signer_account_id: AccountId = "near-api-rs.testnet".parse::<AccountId>()?;
    let outer_signer_secret_key = "ed25519:29nYmQCZMsQeYtztXZzm57ayQt2uBHXdn2SAjK4ccMGSQaNUFNJ7Aoteno81eKTex9cGBbk1FuDuqJRsdzx34xDY".parse::<SecretKey>()?;

    let outer_signer =
        InMemorySigner::from_secret_key(outer_signer_account_id.clone(), outer_signer_secret_key);

    let outer_nonce = fetch_nonce(
        &outer_signer_account_id,
        &outer_signer.public_key,
        &provider,
    )
    .await?;

    let block_hash = block.header.hash;

    let tx = near_primitives::transaction::Transaction {
        signer_id: outer_signer_account_id,
        public_key: outer_signer.clone().public_key,
        nonce: outer_nonce + 1,
        block_hash,
        receiver_id: inner_signer_account_id,
        actions: vec![signed_delegate_action],
    };

    let signed_tx = tx.sign(&outer_signer);

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
