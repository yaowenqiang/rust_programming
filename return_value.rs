fn main() {
    //some_higher_level_func();
    //some_higher_level_func2();
    some_higher_level_func3();
}

/*
fn some_higher_level_func() {
    let s = f2();
    println!("{}", s);
    println!("{:p}", s.as_ptr());
}
*/

/*
fn some_higher_level_func2() {
    let r1 = f4();
    let r2 : &String = f4();
    println!("{}",r1);
    println!("{}",r2);
}

*/
fn some_higher_level_func3() {
    let s = String::from("hi");
    let r = good_fn2(&s);
    println!("{}", r);
}

/*
fn bad_f2() -> String {
    let s = String::from("hello world");
    println!("{:p}", s.as_ptr());
    return s;
}

fn bad_f3() -> &String {
    let s = String::from("hello");
    return &s;
}
*/

fn good_fn2(s: &String) -> &String {
    s
}

/*
fn f4(s:String) -> &String {
    return &s;
}
fn bad_func(mark: i32) -> String {
    //return if mark >50 {"PASS"} else {"FAIL"};
    return if mark > 50 {
        String::from("PASS")
    } else {
        String::from("FAIL")
    };
}
*/
