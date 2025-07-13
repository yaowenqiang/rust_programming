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
