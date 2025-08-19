fn main() {
    let mut x = 5;
    let y = &x;
    println!("{}", x);
    println!("{}", 6);
    println!("{}", y);

    // 或者如果需要可变引用
    let z = &mut x;
    println!("{}", z);

    let s = String::from("hello world");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}, {}, {}, {}", s, r1, r2, r3);

    let mut s1 = String::from("Hello");
    let r1 = &mut s1;
    r1.push_str(" world");
    println!("{}", r1);

    let mut s2 = String::from("Hello");
    let r4 = &s2;
    println!("{}", r4);
    //s2.push_str(" world");
    //ush_str(&mut s2, "world")
    println!("{}", r4);
}
