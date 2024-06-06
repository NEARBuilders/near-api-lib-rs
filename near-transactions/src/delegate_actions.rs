pub mod delegate_actions {

    use near_crypto::{PublicKey, Signer};
    use near_primitives::{
        action::delegate::{DelegateAction, NonDelegateAction, SignedDelegateAction},
        transaction::Action,
        types::{AccountId, BlockHeight, Nonce},
    };

    //DelegateAction looks like a transaction actually.
    // pub struct DelegateAction {
    //     /// Signer of the delegated actions
    //     pub sender_id: AccountId,
    //     /// Receiver of the delegated actions.
    //     pub receiver_id: AccountId,
    //     /// List of actions to be executed.
    //     ///
    //     /// With the meta transactions MVP defined in NEP-366, nested
    //     /// DelegateActions are not allowed. A separate type is used to enforce it.
    //     pub actions: Vec<NonDelegateAction>,
    //     /// Nonce to ensure that the same delegate action is not sent twice by a
    //     /// relayer and should match for given account's `public_key`.
    //     /// After this action is processed it will increment.
    //     pub nonce: Nonce,
    //     /// The maximal height of the block in the blockchain below which the given DelegateAction is valid.
    //     pub max_block_height: BlockHeight,
    //     /// Public key used to sign this delegated action.
    //     pub public_key: PublicKey,
    // }
    //
    //
    //
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
}
