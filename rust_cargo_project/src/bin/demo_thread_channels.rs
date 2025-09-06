use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    do_it();
    do_some_other();
}

fn do_it() {
    println!("\nIn demo channels single message.");
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        tx.send(String::from("Hello")).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Received: {}", received);

    handle.join().unwrap();
}

fn do_some_other() {
    println!("\nIn demo channels multiple messages.");
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        for i in 1..=10 {
            let s = std::format!("Message {}", i);
            println!("sending: {}", s);
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }

    handle.join().unwrap();
}
