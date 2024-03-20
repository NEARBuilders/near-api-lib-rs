
# near-api-lib

  
The NEAR API library is a comprehensive Rust library designed to simplify the development of applications on the NEAR blockchain. It provides developers with essential tools and abstractions for account management, transaction building and signing, querying the blockchain state, and performing cryptographic operations, all from the comfort of Rust.
 

## Features

- Account Management: Easily manage NEAR accounts, allowing for the creation of new accounts, key management, and account deletion.

- Transaction Building and Signing: Utilize a builder pattern for constructing and signing transactions with support for various actions.

- Blockchain Interaction: Communicate with the NEAR blockchain using the provided JSON RPC provider to query data or send transactions.

- Cryptographic Utilities: Access cryptographic functions for key generation, signing, and verification. (Rexport for easy access to existing `near-crypto` crate.)

- NEAR Blockchain Primitives: Work directly with NEAR blockchain primitives for low-level operations. (Rexport for easy access to existing `near-primitives` crate.)

  
  

## Getting Started

Add the following to your Cargo.toml file:

```toml
[dependencies]
near-api-lib = "0.1.0-alpha"
```

### Usage

```rust
use near_api_lib::primitives::types::{AccountId, Balance, Gas};
use near_api_lib::Account;
use near_api_lib::InMemorySigner;
use near_api_lib::JsonRpcProvider;

use serde_json::json;
use std::sync::Arc;

mod utils;

#[tokio::main]
async  fn main() -> Result<(), Box<dyn std::error::Error>> {

env_logger::init();
let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
//To-do, implement account exist check.
let new_account_id: AccountId = utils::input("Enter new account name: ")?.parse()?;
let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

  
let gas: Gas = 100_000_000_000_000; // Example amount in yoctoNEAR
// Amount to transfer to the new account
let amount: Balance = 10_000_000_000_000_000_000_000; // Example amount in yoctoNEAR

  
let new_secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
let signer = Arc::new(signer);

  

let account = Account::new(signer_account_id, signer, provider);
let contract_id: AccountId = "testnet".parse::<AccountId>()?;
let method_name = "create_account".to_string();

let args_json = json!({
"new_account_id": new_account_id,
"new_public_key": new_secret_key.public_key()
});

let result = account
.function_call(contract_id, method_name, args_json, gas, amount)
.await;


println!("Response: {:#?}", result);
println!("New Account ID: {}", new_account_id);
println!("Secret Key: {}", new_secret_key);

Ok(())
}
```

### Examples

The crate includes examples that demonstrate how to use various features. To run an example, use the following command:
`cargo run --example <example_name>`

For instance, to test the `create_account` function:
`cargo run --example create_account`

  

## Contributing

We welcome contributions to the `near-api-lib` crate! Please feel free to submit pull requests or open issues to suggest improvements or add new features.


## Disclaimer

### General Use
The near-api-rs is provided "as is" and "as available," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the authors, contributors, or copyright holders be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the software or the use or other dealings in the software.

### Liability
The creators of near-api-rs shall not be held liable for any direct, indirect, incidental, special, exemplary, or consequential damages (including but not limited to procurement of substitute goods or services; loss of use, data, or profits; or business interruption) however caused and on any theory of liability, whether in contract, strict liability, or tort (including negligence or otherwise) arising in any way out of the use of this software, even if advised of the possibility of such damage.

### No Warranty
By using near-api-rs, you acknowledge and agree that the use of the software is entirely at your own risk. The software is provided "as is" and without warranty of any kind, either express or implied, including, but not limited to, the implied warranties of merchantability and fitness for a particular purpose.

### Accuracy and Reliability
The authors and contributors to near-api-rs make no warranty that:
1. The software will meet your requirements or expectations.
2. The software will be uninterrupted, timely, secure, or error-free.
3. The results obtained from the use of the software will be accurate or reliable.
4. Any errors in the software will be corrected.

### Responsibility
It is your responsibility to ensure that your use of near-api-rs complies with all applicable laws and regulations, and to decide whether the software is suitable for your intended use. The authors and contributors assume no responsibility for your use of the software.

### Modifications
The authors and contributors reserve the right to modify this disclaimer at any time, and such modifications shall be effective immediately upon posting of the modified disclaimer. Your continued use of near-api-rs shall be deemed your conclusive acceptance of the modified disclaimer.