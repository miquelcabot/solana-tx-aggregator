use chrono::prelude::*;

pub fn format_time(timestamp: i64) -> String {
    // Format the transaction time for logging
    let datetime = DateTime::from_timestamp(timestamp, 0);
    let newdate = datetime.unwrap().format("%Y-%m-%d %H:%M:%S");

    newdate.to_string()
}
