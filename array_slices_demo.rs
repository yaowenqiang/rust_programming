fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = &a;
    println!("{:?}", b);
    let c: &[i32] = &a;
    println!("{:?}", c);
    //let d: &[i32] = a;
    //println!("{:?}", d);
    println!("{:p}, {}, {:?}", b.as_ptr(), b.len(), b);
    println!("{:p}, {}, {:?}", c.as_ptr(), c.len(), c);

    for elem in b {
        println!("{}", elem);
    }

    let s2 = &a[0..3];
    let s3 = &a[..3];
    let s4 = &a[2..4];
    let s5 = &a[2..];

    println!(
        "\ns2 ptr: {:p}, len: {}, elements:{:?}",
        s2.as_ptr(),
        s2.len(),
        s2
    );
    println!(
        "s3 ptr: {:p}, len: {}, elements:{:?}",
        s3.as_ptr(),
        s3.len(),
        s3
    );
    println!(
        "s4 ptr: {:p}, len: {}, elements:{:?}",
        s4.as_ptr(),
        s4.len(),
        s4
    );
    println!(
        "s5 ptr: {:p}, len: {}, elements:{:?}",
        s5.as_ptr(),
        s5.len(),
        s5
    );

    let mut ma = [1, 2, 3, 4, 5];
    let ms: &mut [i32] = &mut ma;
    ms[0] = 0;
    println!("{:?}", ma);
    //println!("{:?}", ms);
}
