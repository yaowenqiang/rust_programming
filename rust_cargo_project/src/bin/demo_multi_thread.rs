use std::thread;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    println!("\nIn demo_spawning_thread");
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("{:?} displaying {}", thread::current().id(), i);
            sleep(Duration::from_secs(4));
        }
    });

    // handle.join().unwrap();

    match handle.join() {
        Ok(v) => {
            println!("{:?} joined", v)
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        sleep(Duration::from_secs(1));
    }

    let handle2 = thread::spawn(move || {
        println!("{:?} starting", thread::current().id());
        sleep(Duration::from_secs(10));
        println!("{:?} ending", thread::current().id());
        panic!("Deliberate panicking, dude!");
    });

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        sleep(Duration::from_millis(500));
    }

    println!(
        "{:?} Waiting for other thread to end",
        thread::current().id()
    );

    // handle2.join().unwrap();
    match handle2.join() {
        Ok(v) => {
            println!("{:?} joined", v)
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }

    println!("That's all");
}
