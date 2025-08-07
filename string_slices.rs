fn main() {
    let obj = String::from("hello");
    //let s: &str = &obj;
    let s = &obj;

    let s2 = String::from("你好");
    println!("{:p}, {}, {}", s2.as_ptr(), s2.len(), s2);

    for b in s2.bytes() {
        println!("{}", b);
    }

    for b in s2.bytes() {
        println!("{}, {:x},{:o}", b, b, b);
    }

    for c in s2.chars() {
        println!("{}", c);
    }

    let message = "今天天气不错";
    // byte index
    //
    let m = &message[0 .. 3];
    println!("{}", m);
}
