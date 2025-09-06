use std::thread;
use std::time::Duration;
fn main() {
    println!("\n In demo capturing state implicit move .");
    let handle = do_some_work();
    handle.join().unwrap();
    println!("That's all, folks!");

    let handle2 = do_some_other_work();
    handle2.join().unwrap();

    println!("That's all, folks!");
}

fn do_some_work() -> thread::JoinHandle<()> {
    let v = vec![100, 101, 102, 103, 104, 105];
    let handle = thread::spawn(|| {
        for i in v {
            println!("{:?} displaying {}", thread::current().id(), i);
            thread::sleep(Duration::from_millis(500));
        }
    });
    // println!("{:?}", v);
    handle
}

fn do_some_other_work() -> thread::JoinHandle<()> {
    let v = vec![100, 101, 102, 103, 104, 105];
    let handle = thread::spawn(move || {
        // for i in &v {
        for i in v {
            println!(
                "{:?} in move closure, displaying {}",
                thread::current().id(),
                i
            );
            thread::sleep(Duration::from_millis(500));
        }
    });
    // println!("{:?}", v);
    handle
}
