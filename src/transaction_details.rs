use serde::{Deserialize, Serialize};

/// Transaction details
///
/// This struct represents the details of a transaction.
///
/// # Fields
/// * `sender` - The sender of the transaction
/// * `receiver` - The receiver of the transaction
/// * `data` - The data of the transaction
/// * `timestamp` - The timestamp of the transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionDetails {
    pub sender: String,
    pub receiver: String,
    pub data: String,
    pub timestamp: i64,
}
