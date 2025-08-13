fn main() {
    some_higher_level_func();
}

fn some_higher_level_func() {
    let mut n = 42;
    let mut s = String::from("hello");

    some_func(&mut n, &mut s);

    println!("{}", n);
    println!("{}", s);
}

fn some_func(iparam:&mut i32, sparam:&mut String) {
    println!("{:p} {:?} {:?}", iparam, iparam, *iparam);
    println!("{:p} {:?} {:?}", sparam, sparam, *sparam);
    *iparam += 10;
    //iparam = iparam + 10;
    (*sparam).push_str(" world");
    sparam.push_str(" world");
    sparam.push_str(",hi");
}
