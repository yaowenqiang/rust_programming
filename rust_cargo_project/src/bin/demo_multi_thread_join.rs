use rand;
use rand::Rng;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut rng = rand::rng();
            let num = rng.random_range(5..10);
            println!(
                "{:?} sleep for {} sec - starting",
                thread::current().id(),
                num
            );
            thread::sleep(Duration::from_secs(num));
            println!("{:?} sleep for {} sec - ended", thread::current().id(), num);
        });

        handles.push(handle);
    }

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_millis(500));
    }

    println!(
        "{:?} waiting for other threads to end",
        thread::current().id()
    );

    for handle in handles {
        handle.join().unwrap();
    }

    println!("That's all, folks!");
}
