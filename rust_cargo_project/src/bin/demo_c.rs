extern "C" {
    fn abs(n: i32) -> i32;
    fn fabs(n: f64) -> f64;
}

#[no_mangle]
pub extern "C" fn you_can_call_me_from_c() {
    println!("Greetings from my Rust function");
}

fn main() {
    unsafe {
        let res1 = abs(-42);
        println!("res1 is {}", res1);
        let res2 = fabs(-3.14);
        println!("res2 is {}", res2);

        let res3 = my_unsafe_function();
        println!("res3 is {}", res3);
    }

    you_can_call_me_from_c();
}

unsafe fn my_unsafe_function() -> i32 {
    42
}
