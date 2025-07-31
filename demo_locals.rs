use once_cell::sync::Lazy;
use chrono::{Datetime, Utc};
use std::thread::sleep;
use std::time::Duration;
pub fn do_it() {
    println!("\nIn demo_locals::do_it()");
    let x = 42;
    if x != 0 {
        let s1 = "Andy";
        println!("s1: {}", s1);
    }

    //println!("s1: {}", s1);
    static_init_at_compile_time();
}

fn static_init_at_compile_time() {
    static MESSAGE: &str = "Create a Gynru";
    println!("Message: {}", MESSAGE);
}

fn static_init_at_run_time() {
    println!("Curr time:{}", Utc::now().format("%T"));
    static TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
        sleep(Duration::new(5,0));
        let now = Utc::now();
        println!("Curr time:{}", now.format("%T"));
        return now;
    });
    println!("timestamp: {}", (*TIMESTAMP).format("%T"));
}

fn f1() {
    println!("\nIn f1, GLOBAL_MESSAGE: {}", GLOBAL_MESSAGE);
    println!("\nIn f1, GLOBAL_TIMESTAMP: {}", GLOBAL_TIMESTAMP);
}
