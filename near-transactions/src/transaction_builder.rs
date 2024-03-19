//! Provides a builder pattern for constructing NEAR blockchain transactions.
//!
//! The `TransactionBuilder` facilitates the creation of transactions for various actions on the NEAR blockchain,
//! such as transferring tokens, creating accounts, deploying contracts, and more. It abstracts away the complexities
//! involved in constructing transactions manually, ensuring that transactions are built correctly before submission.
//!
//! With `TransactionBuilder`, users can dynamically add actions to a transaction and chain these actions together
//! in a fluent API style. After constructing a transaction, it can be signed with a `Signer` implementation,
//! producing a `SignedTransaction` that is ready for submission to the NEAR blockchain.
//!
//! This module aims to simplify transaction creation and enhance developer experience by providing a clear and concise
//! way to interact with the NEAR blockchain programmatically.
use near_crypto::{PublicKey, Signer};
use near_primitives::{
    account::AccessKey,
    hash::CryptoHash,
    transaction::{
        Action, AddKeyAction, CreateAccountAction, DeleteAccountAction, DeleteKeyAction,
        DeployContractAction, FunctionCallAction, SignedTransaction, StakeAction, Transaction,
        TransferAction,
    },
    types::{AccountId, Balance, Gas, Nonce},
};

// TransactionBuilder struct
pub struct TransactionBuilder {
    transaction: Transaction,
}

impl TransactionBuilder {
    /// Initialize a new TransactionBuilder with the required fields for a Transaction
    pub fn new(
        signer_id: AccountId,
        public_key: PublicKey,
        receiver_id: AccountId,
        nonce: Nonce,
        block_hash: CryptoHash,
    ) -> Self {
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

    /// Sign a transaction with your custom Signer.
    pub fn sign_transaction(self, signer: &dyn Signer) -> SignedTransaction {
        let signature = signer.sign(self.transaction.get_hash_and_size().0.as_ref());
        SignedTransaction::new(signature, self.transaction)
    }

    /// Methods to add CreateAccount action directly to the Transaction's actions vector
    pub fn create_account(mut self) -> Self {
        self.transaction
            .actions
            .push(Action::CreateAccount(CreateAccountAction {}));
        self
    }

    /// Method to add a DeployContract action
    pub fn deploy_contract(mut self, code: Vec<u8>) -> Self {
        self.transaction
            .actions
            .push(Action::DeployContract(DeployContractAction { code }));
        self
    }

    pub fn function_call(
        mut self,
        method_name: String,
        args: Vec<u8>,
        gas: Gas,
        deposit: Balance,
    ) -> Self {
        self.transaction
            .actions
            .push(Action::FunctionCall(Box::new(FunctionCallAction {
                method_name,
                args,
                gas,
                deposit,
            })));
        self
    }

    pub fn transfer(mut self, deposit: Balance) -> Self {
        self.transaction
            .actions
            .push(Action::Transfer(TransferAction { deposit }));
        self
    }

    pub fn stake(mut self, stake: Balance, public_key: PublicKey) -> Self {
        self.transaction
            .actions
            .push(Action::Stake(Box::new(StakeAction { stake, public_key })));
        self
    }
    pub fn add_key(mut self, public_key: PublicKey, access_key: AccessKey) -> Self {
        self.transaction
            .actions
            .push(Action::AddKey(Box::new(AddKeyAction {
                public_key,
                access_key,
            })));
        self
    }

    pub fn delete_key(mut self, public_key: PublicKey) -> Self {
        self.transaction
            .actions
            .push(Action::DeleteKey(Box::new(DeleteKeyAction { public_key })));
        self
    }

    pub fn delete_account(mut self, beneficiary_id: AccountId) -> Self {
        self.transaction
            .actions
            .push(Action::DeleteAccount(DeleteAccountAction {
                beneficiary_id,
            }));
        self
    }

    // Finalize and return the built Transaction
    pub fn build(self) -> Transaction {
        self.transaction
    }
}
