fn main() {
    let a = 42;
    let b = a;
    println!("{}, {}", a, b);
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2);
    //println!("{}, {}", s1);
    let s3 = s2.clone();
    println!("{}", s3);
    println!("{:?}", s3);
    println!("{:#?}", s3);

    let mut s4 = String::from("I am mutable");
    let s5 = s4.clone();
    s4.push_str(", and can be pushed");
    println!("{}", s4);
    println!("{}", s5);
}
