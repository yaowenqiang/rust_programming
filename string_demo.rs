fn main() {
    using_string_literals();
    using_string_objects();
    using_mutable_string_objects()
}

fn using_string_literals() {
    let m = "hello";
    let s: &'static str = "hello";
    println!("{} {}", s, m);
    println!("{} {:p}, {}", s, s.as_ptr(), s.len());
    println!("{} {:?}, {}", s, s.as_ptr(), s.len());
    println!("{} {:#?}, {}", s, s.as_ptr(), s.len());
}
fn using_string_objects() {
    let s3 = String::from("hello");
    let s4: String = String::from("hello");
    println!("{} {}", s3, s4);
    println!("{} {:p}, {}", s3, s3.as_ptr(), s3.len());
    println!("{} {:p}, {}", s4, s4.as_ptr(), s4.len());
    if s3 == s4 {
        println!("equal");
    } else {
        println!("not equal");
    }
}

fn using_mutable_string_objects() {
    let mut s5 = String::from("    hello");
    println!("{} {:p}, {}", s5, s5.as_ptr(), s5.len());
    s5.push_str(" world");
    println!("{} {:p}, {}", s5, s5.as_ptr(), s5.len());
    let s6 = s5.trim().to_uppercase();
    println!("{} {:p}, {}", s6, s6.as_ptr(), s6.len());
}
