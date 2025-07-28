use std::collections::HashMap;
fn main() {
    demo_arrays();
    demo_arrays_techniques();
    demo_tup0les();
    demo_vectors();
    demo_maps();
}

fn demo_arrays() {
    println!("\nDemo arrays");
    let a1 = [100, 101, 102];
    println!("a1 length is {}, first element is {}", a1.len(), a1[0]);

    let mut a2 = [100, 101, 102];
    a2[0] = 999;
    println!("a2 length is {}, first element is {}", a2.len(), a2[0]);

    println!("Elements in a2:");
    for elem in a2 {
        println!(" {}", elem);
    }
}

fn demo_arrays_techniques() {
    println!("\nArray techniques");
    let a1: [i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("a1 is {:?}", a1);

    let mut a2 = [99; 5];
    a2[0] = 50;
    a2[4] = 25;
    println!("a2 is {:?}", a2);
}

fn demo_tup0les() {
    println!("\nUsing tuples");
    let t1 = (9, "HI", 3.5);
    println!("t1 elements are {},{},{}", t1.0, t1.1, t1.2);

    let mut t2 = (9, "hi", 3.5);
    t2.0 = 99;
    println!("t2 elements are {},{},{}", t2.0, t2.1, t2.2);

    let t3 = ();
    println!("t3 is {:?}", t3);

    let t4: (i32, bool, f64);

    t4 = (50, true, 1.67);

    println!("t4 is {:?}, elements are {}, {},{}", t4, t4.0, t4.1, t4.2);
}

fn demo_vectors() {
    println!("\nUsing vectors");

    let mut _v1: Vec<i32> = Vec::new();
    let mut _v2 = Vec::<i32>::new();

    let mut _v3 = vec![100, 101, 102];

    let item = _v3[0];
    println!("Value: {}", item);

    let opt = _v3.get(0);
    match opt {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }
    _v3.push(103);
    _v3.push(104);
    _v3.push(105);
    _v3.push(106);
    _v3.pop();
    _v3.insert(0, 99);
    println!("Items in v3:");
    for item in _v3 {
        println!(" {}", item);
    }
}

fn demo_maps() {
    println!("\nUsing maps");
    let mut m: std::collections::HashMap<String, i32> = HashMap::new();
    let mut _m2: HashMap<String, i32> = HashMap::new();
    m.insert(String::from("UK"), 44);
    m.insert(String::from("NO"), 47);
    m.insert(String::from("5G"), 65);

    m.entry(String::from("54")).or_insert(37);

    let val = m["UK"];
    println!("Value: {}", val);

    let opt = m.get("UK");
    match opt {
        Some(value) => println!("Value: {}", value),
        None => println!("No value")
    }

    println!("Entries in m:");

    for entry in &m {
        println!("  {:?}", entry);
    }

}
