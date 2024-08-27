use crate::transaction_details::TransactionDetails;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use solana_sdk::signature::Signature;
use warp::Filter;

/// Create the RESTful API
///
/// This function creates the RESTful API.
///
/// # Arguments
/// * `transactions` - The transactions hash map
///
/// # Returns
/// A warp filter
pub fn create_api(
    transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("transactions")
        .and(warp::query::<HashMap<String, String>>())
        .and(with_transactions(transactions))
        .map(
            |params: HashMap<String, String>,
             transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>| {
                let transactions = transactions.lock().unwrap();

                if let Some(signature) = params.get("id") {
                    let signature =
                        Signature::from_str(signature).expect("Invalid signature format");
                    if let Some(transaction) = transactions.get(&signature) {
                        return warp::reply::json(transaction);
                    }
                }

                if let Some(day) = params.get("day") {
                    let date = chrono::NaiveDate::parse_from_str(day, "%d/%m/%Y")
                        .expect("Invalid date format");
                    let transactions_for_day: Vec<&TransactionDetails> = transactions
                        .values()
                        .filter(|transaction| {
                            chrono::NaiveDateTime::from_timestamp(transaction.timestamp, 0).date()
                                == date
                        })
                        .collect();
                    return warp::reply::json(&transactions_for_day);
                }

                warp::reply::json(&"Invalid query parameters")
            },
        )
}

/// Return a mutable reference to the transactions hash map
///
/// This function returns a mutable reference to the transactions hash map.
///
/// # Arguments
/// * `transactions` - The transactions hash map
///
/// # Returns
/// A warp filter
fn with_transactions(
    transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>,
) -> impl Filter<
    Extract = (Arc<Mutex<HashMap<Signature, TransactionDetails>>>,),
    Error = std::convert::Infallible,
> + Clone {
    warp::any().map(move || transactions.clone())
}
