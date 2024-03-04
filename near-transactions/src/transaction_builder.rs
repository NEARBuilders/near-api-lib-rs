// Import necessary types from near_primitives
use near_primitives::transaction::{
    //Transaction, Action, CreateAccountAction, DeployContractAction, FunctionCallAction, TransferAction, AddKeyAction, DeleteKeyAction, DeleteAccountAction, StakeAction,
    Action, AddKeyAction, CreateAccountAction, DeleteAccountAction, DeleteKeyAction,
    DeployContractAction, FunctionCallAction, SignedTransaction, StakeAction, Transaction,
    TransferAction,
};
use near_primitives::types::{AccountId, Nonce, Balance, Gas};
use near_primitives::hash::CryptoHash;

use near_primitives::account::{AccessKey};

use near_crypto::{ PublicKey, Signer};

// TransactionBuilder struct
pub struct TransactionBuilder {
    transaction: Transaction
}

impl TransactionBuilder {
    // Initialize a new TransactionBuilder with the required fields for a Transaction
    pub fn new(signer_id: AccountId, public_key: PublicKey, receiver_id: AccountId, nonce: Nonce, block_hash: CryptoHash) -> Self {
        Self {
            transaction: Transaction {
                signer_id,
                public_key,
                receiver_id,
                nonce,
                block_hash,
                actions: Vec::new(), // Initialize the actions vector here
            },
        }
    }

    pub fn sign_transaction(self, signer: &dyn Signer) -> SignedTransaction {
        let signature = signer.sign(self.transaction.get_hash_and_size().0.as_ref());
        SignedTransaction::new(signature, self.transaction)
    }

    // Methods to add actions directly to the Transaction's actions vector
    pub fn create_account(mut self) -> Self {
        self.transaction.actions.push(Action::CreateAccount(CreateAccountAction {}));
        self
    }

    // Method to add a DeployContract action
    pub fn deploy_contract(mut self, code: Vec<u8>) -> Self {
        self.transaction.actions.push(Action::DeployContract(DeployContractAction { code }));
        self
    }

    pub fn function_call(
        mut self,
        method_name: String,
        args: Vec<u8>,
        gas: Gas,
        deposit: Balance,
    ) -> Self {
        self.transaction.actions.push(Action::FunctionCall(Box::new(FunctionCallAction {
            method_name,
            args,
            gas,
            deposit,
        })));
        self
    }

    pub fn transfer(mut self, deposit: Balance) -> Self {
        self.transaction.actions.push(Action::Transfer(TransferAction { deposit }));
        self
    }

    pub fn stake(mut self, stake: Balance, public_key: PublicKey) -> Self {
        self.transaction.actions.push(Action::Stake(Box::new(StakeAction { stake, public_key })));
        self
    }
    pub fn add_key(mut self, public_key: PublicKey, access_key: AccessKey) -> Self {
        self.transaction.actions.push(Action::AddKey(Box::new(AddKeyAction { public_key, access_key })));
        self
    }

    pub fn delete_key(mut self, public_key: PublicKey) -> Self {
        self.transaction.actions.push(Action::DeleteKey(Box::new(DeleteKeyAction { public_key })));
        self
    }

    pub fn delete_account(mut self, beneficiary_id: AccountId) -> Self {
        self.transaction.actions.push(Action::DeleteAccount(DeleteAccountAction { beneficiary_id }));
        self
    }

    // Finalize and return the built Transaction
    pub fn build(self) -> Transaction {
        self.transaction
    }

}
