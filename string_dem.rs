fn main() {
    let m = "hello";
    let s : &'static str ="hello";
    println!("{} {}", s,m);
    println!("{} {:p}, {}", s, s.as_ptr(),s.len())
}
