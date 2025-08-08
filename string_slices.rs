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
    let m = &message[0..3];
    println!("{}", m);

    let s3 = &message[0..3];
    let s4 = &message[..3];
    let s5 = &message[3..6];
    let s6 = &message[3..];
    println!(
        "\ns3 ptr: {:p}, len: {}, text: {}",
        s3.as_ptr(),
        s3.len(),
        s3
    );
    println!("s4 ptr: {:p}, len: {}, text: {}", s4.as_ptr(), s4.len(), s4);
    println!("s5 ptr: {:p}, len: {}, text: {}", s5.as_ptr(), s5.len(), s5);
    println!("s6 ptr: {:p}, len: {}, text: {}", s6.as_ptr(), s6.len(), s6);

    // mutable string slices

    let mut message2 = String::from("mutable string");
    let ms: &mut str = &mut message2[9..];
    ms.make_ascii_uppercase();
    println!("{}", message2);
    //println!("{}", ms);
}
