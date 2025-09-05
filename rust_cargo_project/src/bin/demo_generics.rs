#[derive(Debug, PartialEq)]
struct Coordinate<T> {
    x: T,
    y: T,
    z: T,
}

struct Angle {
    degrees: i32,
}

#[derive(PartialEq)]
struct TimeSeconds {
    s: i32,
}

#[derive(PartialEq)]
struct TimeMinutes {
    s: i32,
}

impl PartialEq<TimeMinutes> for TimeSeconds {
    fn eq(&self, other: &TimeMinutes) -> bool {
        self.s == other.s * 60
    }
}

impl PartialEq<TimeSeconds> for TimeMinutes {
    fn eq(&self, other: &TimeSeconds) -> bool {
        other == self
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.degrees % 360 == other.degrees % 360
    }
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

    let c1 = Coordinate { x: 1, y: 2, z: 3 };
    let c2 = Coordinate { x: 1, y: 2, z: 3 };
    let c3 = Coordinate { x: 3, y: 4, z: 5 };

    println!("c1 = c2? {}", c1 == c2);
    println!("c2 = c3? {}", c2 == c3);

    let c4 = Coordinate {
        x: 1.1,
        y: 2.2,
        z: 3.3,
    };
    let c5 = Coordinate {
        x: 1.1,
        y: 2.2,
        z: 3.3,
    };
    println!("c4 = c5? {}", c4 == c5);
    println!("c4 = c5? {}", c4.eq(&c5));
    println!("c4 != c5? {}", c4 != c5);
    println!("c4 != c5? {}", c4.ne(&c5));

    let a1 = Angle { degrees: 1 };
    let a2 = Angle { degrees: 361 };

    println!("a1 == a2? {}", a1 == a2);

    let s1 = TimeSeconds { s: 60 };
    let m1 = TimeMinutes { s: 1 };
    println!("t1 == m1? {}", s1 == m1);
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
