fn main() {
    demo_arrays();
    demo_arrays_techniques();
        
}

fn demo_arrays(){
    println!("\nDemo arrays");
    let a1 = [100, 101, 102];
    println!("a1 length is {}, first element is {}", a1.len(), a1[0]);

    let mut a2 = [100, 101, 102];
    a2[0] = 999;
    println!("a2 length is {}, first element is {}",a2.len(), a2[0]);

    println!("Elements in a2:");
    for elem in a2 {
        println!(" {}", elem);
    }
}

fn demo_arrays_techniques(){
    println!("\nArray techniques");
    let a1:[i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("a1 is {:?}", a1);

    let mut a2 = [99; 5];
    a2[0] = 50;
    a2[4] = 25;
    println!("a2 is {:?}", a2);

}

