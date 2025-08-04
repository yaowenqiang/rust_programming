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
}
