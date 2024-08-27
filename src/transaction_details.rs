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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_details_serialization() {
        let transaction_details = TransactionDetails {
            sender: "SenderPubkey".to_string(),
            receiver: "ReceiverPubkey".to_string(),
            data: "some_data".to_string(),
            timestamp: 1620000000,
        };

        let serialized = serde_json::to_string(&transaction_details).unwrap();
        let deserialized: TransactionDetails = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.sender, "SenderPubkey");
        assert_eq!(deserialized.receiver, "ReceiverPubkey");
        assert_eq!(deserialized.data, "some_data");
        assert_eq!(deserialized.timestamp, 1620000000);
    }
}
