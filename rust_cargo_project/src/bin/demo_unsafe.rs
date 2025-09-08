fn main() {
    let mut x = 100;
    x += 1;
    let mut y = 200;
    y += 1;
    let p1: *const i32 = &x;
    let p2: *mut i32 = &mut y;

    unsafe {
        // *p1 = 111;
        // println!("{}", *p1);

        *p2 = 222;
        println!("{}", *p2);
    }
}
