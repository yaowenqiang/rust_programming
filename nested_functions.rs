fn main() {
    outer()
}

fn outer() {
    fn sqr(i: i32) -> i32 {
        i * i
    }
    println!("Square of 5 is {}", sqr(5));
    println!("Square of 7 is {}", sqr(7));
}
