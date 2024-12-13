use near_crypto::{PublicKey, Signer};
use near_primitives::{
    action::delegate::{DelegateAction, NonDelegateAction, SignedDelegateAction},
    transaction::Action,
    types::{AccountId, BlockHeight, Nonce},
};

pub fn create_signed_delegate_action(delegate: DelegateAction, signer: &dyn Signer) -> Action {
    Action::Delegate(Box::new(SignedDelegateAction {
        delegate_action: delegate.clone(),
        signature: signer.sign(delegate.get_nep461_hash().as_bytes()),
    }))
}

pub fn create_delegate_action(
    signer_id: AccountId,
    receiver_id: AccountId,
    actions: Vec<Action>,
    nonce: Nonce,
    max_block_height: BlockHeight,
    public_key: PublicKey,
) -> DelegateAction {
    DelegateAction {
        sender_id: signer_id,
        receiver_id,
        actions: actions
            .iter()
            .map(|a| NonDelegateAction::try_from(a.clone()).unwrap())
            .collect(),
        nonce,
        max_block_height,
        public_key,
    }
}
