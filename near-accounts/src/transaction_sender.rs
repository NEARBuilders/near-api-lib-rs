use crate::accounts::ArcProviderSendSync;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::SignedTransaction;
use near_primitives::views::TxExecutionStatus;
use near_providers::types::transactions::RpcTransactionResponse;

///This struct represent a Transaction Sender used specifically if you want to send transactions manually.
/// This gives user more control over how they want to send their transactions to the NEAR network for examples, asyn, sync or advanced.
/// It is only used by function_call method from Account for now to enable this flexibility.
#[derive(Clone)]
pub struct TransactionSender {
    pub signed_transaction: SignedTransaction,
    provider: ArcProviderSendSync,
}

impl TransactionSender {
    /// Constructs a new `TransactionSender` instance.
    ///
    /// # Arguments
    ///
    /// * `signed_transaction` - Signed transaction to be sent to the NEAR chain.
    /// * `provider` - A provider instance for interacting with the blockchain.
    ///
    /// # Returns
    ///
    /// A new `Account` instance.
    pub fn new(signed_transaction: SignedTransaction, provider: ArcProviderSendSync) -> Self {
        Self {
            signed_transaction,
            provider,
        }
    }

    ///Send your transaction to the NEAR blockchain synchronously using the send_tx RPC end point and default wait_until value
    pub async fn transact(self) -> Result<RpcTransactionResponse, Box<dyn std::error::Error>> {
        self.provider
            .send_tx(self.signed_transaction, TxExecutionStatus::default())
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    ///Send your transaction to the NEAR blockchain asynchronously using the send_tx RPC end point and default wait_until None.
    pub async fn transact_async(
        self,
    ) -> Result<RpcTransactionResponse, Box<dyn std::error::Error>> {
        self.provider
            .send_tx(self.signed_transaction, TxExecutionStatus::None)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    ///Send your transaction to the NEAR blockchain using the send_tx RPC end point and custom wait_until value.
    /// Different wait_until values and what they mean:
    ///
    /// * None
    /// Transaction is waiting to be included into the block
    ///
    /// * Included
    /// Transaction is included into the block. The block may be not finalised yet
    ///
    /// * ExecutedOptimistic,
    /// Transaction is included into the block +
    /// All the transaction receipts finished their execution.
    /// The corresponding blocks for tx and each receipt may be not finalized yet
    /// It is also the default value unless defined otherwise.
    ///
    /// * IncludedFinal
    /// Transaction is included into finalized block
    ///
    /// * Executed
    /// Transaction is included into finalized block +
    /// All the transaction receipts finished their execution.
    /// The corresponding blocks for each receipt may be not finalized yet
    ///
    /// * Final
    /// Transaction is included into finalize block +
    /// Execution of transaction receipts is finalized
    pub async fn transact_advanced(
        self,
        wait_until_str: &str,
    ) -> Result<RpcTransactionResponse, Box<dyn std::error::Error>> {
        let wait_until: TxExecutionStatus =
            serde_json::from_value(serde_json::json!(wait_until_str))?;
        self.provider
            .send_tx(self.signed_transaction, wait_until)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// Returns transaction hash for a given signed transaction
    pub fn get_transaction_hash(self) -> Result<CryptoHash, Box<dyn std::error::Error>> {
        Ok(self.signed_transaction.get_hash())
    }
}
