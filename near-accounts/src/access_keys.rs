use near_primitives::account::{AccessKey, AccessKeyPermission, FunctionCallPermission};
use near_primitives::types::Balance;


pub fn full_access_key() -> AccessKey {
    AccessKey::full_access()
}

pub fn function_call_access_key(allowance: Option<Balance>, receiver_id: String, method_names: Vec<String>) -> AccessKey {
    AccessKey { nonce: 0, permission: AccessKeyPermission::FunctionCall(FunctionCallPermission { allowance, receiver_id, method_names }) }
}