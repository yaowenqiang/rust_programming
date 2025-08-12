fn main() {
    let i = 1;
    let s = "hello world".to_string();
    let s2 = "hello world";
    some_func1(&i, &s);
    some_func2(&i, &s2);
    some_func3(&i, &s2);
}

fn some_func1(iparam: &i32, sparam: &String) {
    if *iparam > 50 {
        println!("{}, {} PASS", *iparam, (*sparam).to_uppercase());
    } else {
        println!("{}, {} FAIL", *iparam, (*sparam).to_uppercase());
    }
}

fn some_func2(iparam: &i32, sparam: &str) {
    if *iparam > 50 {
        println!("{}, {} PASS", *iparam, (*sparam).to_uppercase());
    } else {
        println!("{}, {} FAIL", *iparam, (*sparam).to_uppercase());
        println!("{}, {} FAIL", *iparam, sparam.to_uppercase());
    }
}

fn some_func3(iparam: &i32, sparam: &str) {
    println!(
        "values:{0} and {1}, address {0:p} and {1:p}",
        iparam, sparam
    );
}
