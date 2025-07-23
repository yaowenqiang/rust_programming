fn main() {
    let mut x = 5;
    let y = &x;
    let z = &mut x;
    println!("{}", x);
    println!("{}", 6);
    println!("{}", z);
}
