use transactions::TransactionBuilder;
use near_crypto::{Signer, PublicKey};
use near_primitives::types::{AccountId, Balance, BlockReference, Finality};
use near_primitives::views::{FinalExecutionOutcomeView, QueryRequest};
use near_jsonrpc_primitives::types::query::{RpcQueryResponse, QueryResponseKind};
use near_primitives::account::AccessKey;
//use near_jsonrpc_client::errors::JsonRpcError;
//use near_jsonrpc_primitives::types::transactions::RpcTransactionError;


//items from traits can only be used if the trait is in scope
// can we change it somehow with better crate design?
use providers::Provider;
//use providers::JsonRpcProvider;

use std::sync::Arc;


pub struct Account {
    pub account_id: AccountId,
    pub signer: Arc<dyn Signer>,
    pub provider: Arc<dyn Provider>, // Use your Provider abstraction
}

impl Account {
    pub fn new(account_id: AccountId, signer: Arc<dyn Signer>, provider: Arc<dyn Provider>) -> Self {
        Self { account_id, signer, provider }
    }

    // Function to fetch the current nonce for an account's access key
    pub async fn fetch_nonce(&self, account_id: &AccountId, public_key: &PublicKey) -> Result<u64, Box<dyn std::error::Error>> {
        
        let query_request = QueryRequest::ViewAccessKey {
            account_id: account_id.clone(),
            public_key: public_key.clone(),
        };

        // Send the query to the NEAR blockchain
        let response: RpcQueryResponse = self.provider.query(query_request).await?;

        // Extract the access key view from the response
        if let QueryResponseKind::AccessKey(access_key_view) = response.kind {
            Ok(access_key_view.nonce)
        } else {
            Err("Unexpected response kind".into())
        }
    }

    pub async fn create_account(&self, new_account_id: AccountId, public_key: PublicKey, amount: Balance) -> Result<FinalExecutionOutcomeView, Box<dyn std::error::Error>> {
        //Look into the whole access key thingy. We need it anyway but it also helps with nonce.
        // Fetch the current nonce for the signer account and latest block hash
        let nonce = self.fetch_nonce(&self.account_id, &self.signer.public_key()).await?;
        
        //Block hash
        let block_reference = BlockReference::Finality(Finality::Final);
        let block = self.provider.block(block_reference).await?;
        let block_hash = block.header.hash;

        // Use TransactionBuilder to construct the transaction
        let signed_tx = TransactionBuilder::new(
            self.account_id.clone(), 
            self.signer.public_key(), 
            new_account_id.clone(), 
            nonce+1, 
            block_hash)
            .create_account()
            .transfer(amount)
            .add_key(public_key, AccessKey::full_access())
            //.build()
            .signTransaction(&*self.signer); // Sign the transaction

        // Send the transaction
        let transaction_result = self.provider.send_transaction(signed_tx).await?;
        Ok(transaction_result)
    }

    // Implement other account methods using TransactionBuilder...
}


//To-do
//JS reference for 
// protected async signTransaction(receiverId: string, actions: Action[]): Promise<[Uint8Array, SignedTransaction]> {
//     const accessKeyInfo = await this.findAccessKey(receiverId, actions);
//     if (!accessKeyInfo) {
//         throw new TypedError(`Can not sign transactions for account ${this.accountId} on network ${this.connection.networkId}, no matching key pair exists for this account`, 'KeyNotFound');
//     }
//     const { accessKey } = accessKeyInfo;

//     const block = await this.connection.provider.block({ finality: 'final' });
//     const blockHash = block.header.hash;

//     const nonce = accessKey.nonce.add(new BN(1));
//     return await signTransaction(
//         receiverId, nonce, actions, baseDecode(blockHash), this.connection.signer, this.accountId, this.connection.networkId
//     );
// }

#[cfg(test)]
mod tests {

    use providers::JsonRpcProvider;
    use std::sync::Arc;
    use near_crypto::InMemorySigner;
    use near_primitives::types::Balance;
    use near_crypto::{Signer, PublicKey};
    use crate::Account;
    mod utils;
    use std::io::{self, Write};
    
    #[tokio::test]
    async fn test_create_account() {
        
        let signer_account_id = input("Enter the signer Account ID: ")?.parse()?;
        let signer_secret_key = input("Enter the signer's private key: ")?.parse()?;
        let new_account_id = input("Enter the signer's private key: ")?.parse()?;
        //let new_account_id = "newaccount.testnet".parse().unwrap();
        //let private_key = "ed25519:3tNQ8Nt6y9m7Kq3VkaQH8k2L7yD3xq6CJXbwz1tEPVZD".parse().unwrap(); // Example private key
        let signer = InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
        // Amount to transfer to the new account
        let amount: Balance = 10_000_000_000_000_000_000_000; // Example amount in yoctoNEAR

        // Public key for the new account (normally generated but for the test we can use a fixed one)
        
        //Create a keypaid using near_crypto
        let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
        //let public_key: PublicKey = PublicKey::from_str("ed25519:8LXEySyBYewiTTLxjfF1TKDsxxxxxxxxxxxxxx").unwrap();

        //let provider = Arc::new(MockProvider);
        let provider = JsonRpcProvider::new("https://rpc.testnet.near.org");
        let signer = Arc::new(signer);

        let account = Account::new(signer_account_id, signer, provider);

        // Call create_account
        let result = account.create_account(new_account_id, new_key_pair.public_key(), amount).await;

        assert!(result.is_ok());
    }

    pub fn input(query: &str) -> io::Result<String> {
        print!("{}", query);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_owned())
    }
}