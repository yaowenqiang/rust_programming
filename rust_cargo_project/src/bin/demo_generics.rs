use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate<T> {
    x: T,
    y: T,
    z: T,
}

struct Angle {
    degrees: i32,
}
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Currency {
    dollars: i32,
    cents: i32,
}

#[derive(PartialEq)]
struct TimeSeconds {
    s: i32,
}

#[derive(PartialEq)]
struct TimeMinutes {
    s: i32,
}

#[derive(Debug)]
struct Emp {
    name: String,
    salary: f32,
}

impl Emp {
    fn new(name: &str, salary: f32) -> Emp {
        Emp {
            name: name.to_string(),
            salary,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct EmpCode {
    country: String,
    empnum: String,
}

impl EmpCode {
    fn new(country: &str, empnum: &str) -> EmpCode {
        EmpCode {
            country: country.to_string(),
            empnum: empnum.to_string(),
        }
    }
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

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let d1 = self.degrees % 360;
        let d2 = other.degrees % 360;
        Some(d1.cmp(&d2))
    }
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

    let mut staff: HashMap<EmpCode, Emp> = HashMap::new();
    staff.insert(EmpCode::new("USA", "123"), Emp::new("Jack", 1000.0));
    staff.insert(EmpCode::new("UK", "456"), Emp::new("Bill", 2000.0));
    staff.insert(EmpCode::new("UK", "789"), Emp::new("Marry", 2100.0));

    let emp = &staff[&EmpCode::new("UK", "789")];
    println!("{:?}", emp);

    let c1 = Currency {
        dollars: 10,
        cents: 75,
    };
    let c2 = Currency {
        dollars: 20,
        cents: 50,
    };
    let c3 = Currency {
        dollars: 30,
        cents: 75,
    };

    println!("c1 < c2?: {}", c1 < c2);
    println!("c1 <= c2?: {}", c1 <= c2);
    println!("c1 > c2?: {}", c1 > c2);
    println!("c1 >= c2?: {}", c1 >= c2);

    let a1 = Angle { degrees: 10 };
    let a2 = Angle { degrees: 400 };

    println!("a1 < a2?: {}", a1 < a2);
    println!("a1.lt(a2)?: {}", a1.lt(&a2));
    println!("a1 <= a2?: {}", a1 <= a2);
    println!("a1.le(a2)?: {}", a1.le(&a2));

    println!("a1 > a2?: {}", a1 > a2);
    println!("a1.gt(a2)?: {}", a1.gt(&a2));
    println!("a1 >= a2?: {}", a1 >= a2);
    println!("a1.ge(a2)?: {}", a1.ge(&a2));

    let m1 = ExamMark { value: 90 };
    let m2 = ExamMark { value: 99 };
    let m3 = ExamMark { value: 180 };
    let m4 = ExamMark { value: 042 };

    // let min = m1.min(m2);
    // let max = m1.max(m2);
    println!("m1.min(m2): {:?}", m1.min(m2));
    println!("m1.max(m2): {:?}", m1.max(m2));
    println!(
        "{:?}",
        m3.clamp(ExamMark { value: 0 }, ExamMark { value: 100 })
    );
    println!(
        "{:?}",
        m4.clamp(ExamMark { value: 0 }, ExamMark { value: 100 })
    );
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct ExamMark {
    value: i32,
}
