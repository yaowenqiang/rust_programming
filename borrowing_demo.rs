fn main() {
    let s = String::from("hello");
    let r = &s;
    let r2: &String = &s;
    println!("{},{}, {}", r, s, r2);

    let mut s: String = String::from("hello");
    println!("{}", s);
    let r3: &mut String = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}
