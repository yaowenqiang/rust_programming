// i8
// i16
// i32
// i64
// i128
//
// u8
// u16
// u32
// u64
// u128
//
// platform-specific interger types
// isize
// usize

fn main() {
    demo_integers();
    demo_floats();
    demo_other_simple_types();
    demo_additional_techniques();
}

fn demo_integers() {
    let a1: i32 = -123456;
    let a2: i32 = 0xffff;
    let a3: i32 = 0o177;
    let a4: i32 = 0b1110;

    let b: u32 = 12345;
    let c: isize = 14600;
    println!("\n Numbers are {} {} {} {} {} {}", a1, a2, a3, a4, b, c);
    println!(
        "\n Numbers in reverse order are {5} {4} {3} {2} {1} {0}",
        a1, a2, a3, a4, b, c
    );
    println!(
        "isize is {} bytes on my machine.",
        std::mem::size_of::<isize>()
    );
}

// f32
// f64

fn demo_floats() {
    let f1: f32 = 1.2345;
    let f2: f64 = 9.87654;
    println!("\n Floats are {} {}", f1, f2);
    println!("\n Floats field width 10 L-aligned filled with space are \n***{:<10.2}*** and ***{:<10.2}***", f1, f2);
    println!("\n Floats field width 10 R-aligned filled with space are \n***{:>10.2}*** and ***{:>10.2}***", f1, f2);
    println!("\n Floats field width 10 L-aligned filled with tilde are \n***{:~<10.2}*** and ***{:~<10.2}***", f1, f2);
    println!("\n Floats field width 10 R-aligned filled with tilde are \n***{:~>10.2}*** and ***{:~>10.2}***", f1, f2);

    let f3: f32 = -1.6021763e-16;
    let f4: f64 = 2.99792458e8;
    println!("\nElectron charge {0}, {0:e}, {0:.4e}", f3);
    println!("\nSpeed of light {0}, {0:e}, {0:.4e}", f4);
}

fn demo_other_simple_types() {
    let is_welsh: bool = true;
    let can_sign: bool = false;
    println!("\nIs welsh? {}, can sign? {}", is_welsh, can_sign);

    let is_welsh_num: i32 = is_welsh as i32;
    let can_sign_num: i32 = can_sign as i32;

    println!(
        "\nIs Welsh as number: {}, can sign as number: {}",
        is_welsh_num, can_sign_num
    );

    let res1: bool = is_welsh && can_sign;
    let res2: bool = is_welsh || can_sign;
    let res3: bool = !(is_welsh || can_sign);
    print!("\n res1:{}, res2: {}, res3: {}", res1, res2, res3);

    let middle_initial: char = 'c';
    print!("\nhey you with the middle initial {}", middle_initial);
}

// Inferring Types

fn demo_additional_techniques() {
    // Rust can infer types
    let a = -12345;
    let b = 3.14;
    let c = 'x';
    println!("\n a is {}, b is {}, c is {}", a, b, c);

    // Variables are inmutable by default

    let d = 0;
    // d = 1;
    print!("\n d is {}", d);

    let mut e = 0;
    print!("\n e originally is {}", e);
    e = 1;
    print!("\n e afterwards is {}", e);

    // if you don't want to use a variable, prefix name with _ to avoid compiler warning
    let _f = 0;

    let g = 1.99;
    let h = g as i32;
    println!("\n g is {}, h is {}", g, h);

    let num = "12345";
    println!("\n num as a string is {}", num);

    let num = 12345;

    println!("\n num + 1 as a number is {}", num + 1);

    const SECONDS_IN_HOUR: i32 = 3_600;
    const SECONDS_IN_DAY: i32 = SECONDS_IN_HOUR * 24;
    println!("\nThere are {} seconds in a day.", SECONDS_IN_DAY);
}
