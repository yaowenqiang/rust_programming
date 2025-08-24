use chrono::{DateTime, Utc};
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let get_timestamp = || -> DateTime<Utc> { Utc::now() };
    println!("Timestamp: {}", get_timestamp().format("%c"));

    let reciprocal = |m: f64| {
        if m == 0.0 {
            0.0
        } else {
            1.0 / m
        }
    };
    println!("Reciprocal: {}", reciprocal(5.0));

    let reciprocal2 = |n| if n == 0.0 {0.0} else { 1.0 / n };
    let reciprocal3 = |n| if n == 0 {0} else { 1 / n };
    println!("Reciprocal2: {}", reciprocal2(0.0));
    println!("Reciprocal3: {}", reciprocal3(0));


    let get_timestamp_after_delay = |seconds: u64| -> DateTime<Utc> {
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };
    println!("Timestamp: {}", get_timestamp_after_delay(5).format("%D"));

    let product = |a: i32, b: i32| -> i32 { a * b};
    println!("Product: {}", product(10, 20));

    let get_timestamp2 = || Utc::now();
    println!("Timestamp2: {}", get_timestamp2());

    // capture
    capture_immutable_reference();
    capture_mutable_reference();

    capture_value_automatically();
    capture_value_forcibly();
    std::thread::sleep(Duration::from_secs(10));
}

fn capture_immutable_reference() {
    let b1 = String::from("---------------");
    let b2 = String::from("---------------");

    let display_heading = |s| {
        println!("{}", b1);
        println!("| {:<15} |",s);
        println!("{}", b2);
    };

    display_heading(String::from("hello world"));

    println!("{b1} {b2}");
}
fn capture_mutable_reference() {
    let mut b1 = String::from("---------------");
    let mut b2 = String::from("---------------");

    let mut display_heading = |s| {
        b1.push_str(" xxx");
        b2.push_str(" xxx");
        println!("{}", b1);
        println!("| {:<15} |",s);
        println!("{}", b2);
    };

    display_heading(String::from("hello world"));

    println!("{b1} {b2}");
}

fn capture_value_automatically() {
    let message = String::from("hello world");
    println!("Message initially: {message}");
    let consume_message = || {
        println!("Message in closure: {message}");
        std::mem::drop(message);
    };
    // println!("{message}");

    consume_message();
    // consume_message();

}

fn capture_value_forcibly() {
    let message = String::from("hello world");
    println!("Start of method...");
    std::thread::spawn(move || {
        println!("Message at start of closure: {message}");
        std::thread::sleep(Duration::new(10,0));
        println!("Message at end of closure: {message}");
    });

    println!("End of method...");

}
