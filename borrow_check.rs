fn main() {
    let mut x = 5;
    let y = &x;
    println!("{}", x);
    println!("{}", 6);
    println!("{}", y);

    // 或者如果需要可变引用
    let z = &mut x;
    println!("{}", z);
}
