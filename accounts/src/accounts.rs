use transaction::TransactionBuilder;
use near_crypto::{Signer, PublicKey};
use near_primitives::types::{AccountId, Balance};
use near_primitives::views::FinalExecutionOutcome;


//items from traits can only be used if the trait is in scope
// can we change it somehow with better crate design?
use providers::Provider;
use providers::JsonRpcProvider;

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

    pub async fn create_account(&self, new_account_id: AccountId, public_key: PublicKey, amount: Balance) -> Result<FinalExecutionOutcome, Box<dyn std::error::Error>> {
        //Look into the whole access key thingy. We need it anyway but it also helps with nonce.
        // Fetch the current nonce for the signer account and latest block hash
        let nonce = self.provider.fetch_nonce(&self.account_id).await?;
        
        //Implement provider.block for this.
        let block_hash = self.provider.fetch_latest_block_hash().await?;

        // Use TransactionBuilder to construct the transaction
        let signed_tx = TransactionBuilder::new(
            self.account_id.clone(), 
            self.signer.public_key(), 
            nonce, 
            new_account_id.clone(), 
            block_hash)
            .create_account()
            .transfer(amount)
            .add_key(public_key, AccessKey::full_access())
            .build();
            .signTransaction(&*self.signer); // Sign the transaction

        // Sign the transaction
        //let signed_tx = self.signer.sign_transaction(&transaction).await?;

        // Send the transaction
        self.provider.send_transaction(&signed_tx).await
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