fn main() {
    do_it()
}

fn do_it(){
    println!("\nIn returning mutable reference: do_it");
    let mut r = String::from("hello");
    r.push_str(" world");
    println!("r: {}", r);
    some_func(&mut r);
    println!("r: {}", r);

}

fn some_func(s: &mut String) ->&mut String {
    s.push_str(", how is today?");
    s
}
