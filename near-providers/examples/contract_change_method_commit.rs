//use near_crypto::Signer;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use near_primitives::types::{AccountId, BlockReference, Finality};
use near_primitives::views::TxExecutionStatus;
use near_providers::jsonrpc_client::{methods, JsonRpcClient};
use near_providers::types::query::QueryResponseKind;
use std::io::{Error, ErrorKind};

use near_providers::JsonRpcProvider;

// items from traits can only be used if the trait is in scope
// can we change it somehow with better crate design?
use near_providers::Provider;

use serde_json::json;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let provider = JsonRpcProvider::new("https://rpc.testnet1.near.org");

    let signer_account_id: AccountId = "near-api-rs.testnet".parse::<AccountId>()?;
    let signer_secret_key = "ed25519:29nYmQCZMsQeYtztXZzm57ayQt2uBHXdn2SAjK4ccMGSQaNUFNJ7Aoteno81eKTex9cGBbk1FuDuqJRsdzx34xDY".parse::<near_crypto::SecretKey>()?;
    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;

    let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);

    let current_nonce = fetch_nonce(&signer).await?;

    let block_reference = BlockReference::Finality(Finality::Final);
    let block = provider.block(block_reference).await?;
    let block_hash = block.header.hash;

    let transaction = Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: contract_id,
        block_hash: block_hash,
        actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
            method_name: "set_status".to_string(),
            args: json!({"message": "working1"}).to_string().into_bytes(),
            gas: 100_000_000_000_000, // 100 TeraGas
            deposit: 0,
        }))],
    };

    //let response = provider.send_transaction(transaction.sign(&signer)).await;

    let response = provider
        .send_tx(
            transaction.sign(&signer),
            TxExecutionStatus::ExecutedOptimistic,
        )
        .await;

    // match response {
    //     Ok(res) => Ok(res.transaction_outcome),
    //     Err(err) => {
    //         eprintln!("Error: {:#?}", err);
    //         Err(Error::new(ErrorKind::Other, "RPC query failed"))
    //     }
    // }
    println!("{:#?}", response);
    Ok(())
}

//println!("response: {:#?}", response);

//Ok(())

async fn fetch_nonce(signer: &near_crypto::InMemorySigner) -> Result<u64, std::io::Error> {
    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        })
        .await;

    match access_key_query_response {
        Ok(res) => match res.kind {
            QueryResponseKind::AccessKey(access_key) => Ok(access_key.nonce),
            _ => Err(Error::new(
                ErrorKind::Other,
                "Failed to extract current nonce",
            )),
        },
        Err(err) => {
            eprintln!("Error: {:#?}", err);
            Err(Error::new(ErrorKind::Other, "RPC query failed"))
        }
    }
}
