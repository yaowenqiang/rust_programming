use chrono::{DateTime, Utc};
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let get_timestamp = || -> DateTime<Utc> { Utc::now() };
    println!("{}", get_timestamp().format("%T"));

    let reciprocal = |m: f64| {
        if m == 0.0 {
            0.0
        } else {
            1.0 / m
        }
    };
    println!("{}", reciprocal(1.0));

    let get_timestamp_after_delay = |seconds: u64| -> DateTime<Utc> {
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };
    println!("{}", get_timestamp_after_delay(5));
}
