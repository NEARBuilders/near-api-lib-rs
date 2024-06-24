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
    hash::CryptoHash,
    transaction::{Action, SignedTransaction, Transaction},
    types::{AccountId, Nonce},
};

// TransactionBuilder struct
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn sign_transaction(&self, signer: &dyn Signer) -> SignedTransaction {
        let signature = signer.sign(self.transaction.get_hash_and_size().0.as_ref());
        SignedTransaction::new(signature, self.transaction.clone())
    }

    pub fn set_action(&mut self, actions: &[Action]) -> &mut Self {
        actions.clone_into(&mut self.transaction.actions);
        self
    }

    // Finalize and return the built Transaction
    pub fn build(self) -> Transaction {
        self.transaction
    }
}
