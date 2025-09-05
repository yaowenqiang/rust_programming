#[derive(Debug)]
struct Coordinate<T> {
    x: T,
    y: T,
    z: T,
}
fn main() {
    let c1 = Coordinate::<i32> { x: 1, y: 2, z: 3 };
    let c2 = Coordinate {
        x: 1.1,
        y: 2.2,
        z: 3.3,
    };
    println!("x:{}, y:{}, z: {}", c1.x, c1.y, c1.z);
    println!("x:{}, y:{}, z: {}", c2.x, c2.y, c2.z);
    println!("{:?}", c1);
    println!("{:?}", c2);
    let i = [1, 2, 3, 4];
    process_array_ints(&i);
    process_array::<i32>(&i);
    let j = ["abc".to_string()];
    process_array::<String>(&j);
    let k = ["你好".to_string()];
    process_array(&k);
    let l = ["a"];
    process_array::<&'static str>(&l);
    display_array(&l);
}

fn process_array_ints(arr: &[i32]) {
    println!(
        "{} elements, {} bytes each ",
        arr.len(),
        std::mem::size_of::<i32>()
    );
}

fn process_array<T>(arr: &[T]) {
    println!(
        "{} elements, {} bytes each",
        arr.len(),
        std::mem::size_of::<T>()
    );
}

// type constraints
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    for elem in arr {
        println!("{:?}", elem);
    }
}
