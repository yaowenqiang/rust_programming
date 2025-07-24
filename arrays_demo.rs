fn main() {
    demo_arrays()
}

fn demo_arrays(){
    println!("\nDemo arrays");
    let a1 = [100, 101, 102];
    println!("a1 length is {}, first element is {}", a1.len(), a1[0]);

    let mut a2 = [100, 101, 102];
    a2[0] = 999;
    println!("a2 length is {}, first element is {}",a2.len(), a2[0]);
}
