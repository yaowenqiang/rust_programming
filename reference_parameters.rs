fn main(){
   let i = 1;
   let s = "hello world".to_string();
   some_func1(&i, &s);
}

fn some_func1(iparam:&i32, sparam:&String) {
    if *iparam >50 {
        println!("{}, {} PASS", *iparam, (*sparam).to_uppercase());
    } else {
        println!("{}, {} FAIL", *iparam, (*sparam).to_uppercase());
    }
}
