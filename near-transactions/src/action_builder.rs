//! Provides a builder pattern for constructing NEAR blockchain transactions.
//!
//! The `ActionBuilder` facilitates the addition for various actions on the NEAR blockchain,
//! such as transferring tokens, creating accounts, deploying contracts, and more. It abstracts away the complexities
//! involved in constructing transactions manually, ensuring that transactions are built correctly before submission.
//!
//! With `ActionBuilder`, users can dynamically add actions to a transaction and chain these actions together
//! in a fluent API style. After constructing a transaction, it can be signed with a `Signer` implementation,
//! producing a `SignedTransaction` that is ready for submission to the NEAR blockchain.
//!
//! This module aims to simplify transaction creation and enhance developer experience by providing a clear and concise
//! way to interact with the NEAR blockchain programmatically.
use near_crypto::PublicKey;
use near_primitives::{
    account::AccessKey,
    action::StakeAction,
    transaction::{
        Action, AddKeyAction, CreateAccountAction, DeleteAccountAction, DeleteKeyAction,
        DeployContractAction, FunctionCallAction, TransferAction,
    },
    types::{AccountId, Balance, Gas},
};

use serde::{Deserialize, Serialize};

// Define the ActionBuilder struct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBuilder {
    actions: Vec<Action>,
}

impl Default for ActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionBuilder {
    // Constructor for ActionBuilder
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    // Methods to add various actions to the builder
    pub fn create_account(&mut self) -> &mut Self {
        self.actions
            .push(Action::CreateAccount(CreateAccountAction {}));
        self
    }

    pub fn deploy_contract(&mut self, code: &[u8]) -> &mut Self {
        self.actions
            .push(Action::DeployContract(DeployContractAction {
                code: code.to_vec(),
            }));
        self
    }

    pub fn function_call(
        &mut self,
        method_name: String,
        args: Vec<u8>,
        gas: Gas,
        deposit: Balance,
    ) -> &mut Self {
        self.actions
            .push(Action::FunctionCall(Box::new(FunctionCallAction {
                method_name,
                args,
                gas,
                deposit,
            })));
        self
    }

    pub fn transfer(&mut self, deposit: Balance) -> &mut Self {
        self.actions
            .push(Action::Transfer(TransferAction { deposit }));
        self
    }

    pub fn stake(&mut self, stake: Balance, public_key: PublicKey) -> &mut Self {
        self.actions
            .push(Action::Stake(Box::new(StakeAction { stake, public_key })));
        self
    }

    pub fn add_key(&mut self, public_key: PublicKey, access_key: AccessKey) -> &mut Self {
        self.actions.push(Action::AddKey(Box::new(AddKeyAction {
            public_key,
            access_key,
        })));
        self
    }

    pub fn delete_key(&mut self, public_key: PublicKey) -> &mut Self {
        self.actions
            .push(Action::DeleteKey(Box::new(DeleteKeyAction { public_key })));
        self
    }

    pub fn delete_account(&mut self, beneficiary_id: AccountId) -> &mut Self {
        self.actions
            .push(Action::DeleteAccount(DeleteAccountAction {
                beneficiary_id,
            }));
        self
    }

    // Build method to finalize and retrieve the actions
    pub fn build(&self) -> Vec<Action> {
        self.clone().actions
    }
}
