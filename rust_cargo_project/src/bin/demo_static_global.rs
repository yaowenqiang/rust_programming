use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use std::time::Duration;

pub static GLOBAL_MESSAGE: &str = "Hello";

lazy_static! {
    static ref GLOBAL_TIMESTAMP: DateTime<Utc> = {
        let now = Utc::now();
        println!(
            "global GLOBAL_TIMESTAMP: {} ***** initialization **** {}",
            now.format("%T")
        );
        now
    };
}

pub fn do_it() {
    println!("\nIn demo_static_global::do_it()");
    f1();
    f1();
    f2();
    f2();
}

fn f1() {
    println!("\nIn f1, GLOBAL_MESSAGE: {}", GLOBAL_MESSAGE);
    println!(
        "\nIn f1, GLOBAL_TIMESTAMP: {}",
        GLOBAL_TIMESTAMP.format("%T")
    );
}

fn f2() {
    println!("\nIn f2, GLOBAL_MESSAGE: {}", GLOBAL_MESSAGE);
    println!(
        "\nIn f2, GLOBAL_TIMESTAMP: {}",
        GLOBAL_TIMESTAMP.format("%T")
    );
}
