fn main() {
    some_higher_level_func();
    let n = 42;
    let s = String::from("hello");
    f2(&n, &s);
    println!("{}", s);
}

fn f1(iparam:i32, sparam: String) {
    println!("{}", iparam);
    println!("{}", sparam);
    println!("{}, {}", iparam, sparam);
}


fn f2(iparam: &i32, sparam: &String) {
    println!("{}", iparam);
    println!("{}", sparam);
    println!("{}", *iparam);
    println!("{}", *sparam);
    println!("{}", (*sparam).to_uppercase());
    println!("{}", sparam.to_uppercase());
    println!("{:p}, {:p}", iparam, sparam);
}
fn some_higher_level_func() {
    let n = 42;
    let s = String::from("hello");
    let s2 = String::from("hi!");

    //f1(n, s);
    f2(&n, &s);
    f1(n, s);
    //some_func(s2);
    some_func(&s2);
}

fn some_func(s: &String) {
    println!("{}", s);
}

