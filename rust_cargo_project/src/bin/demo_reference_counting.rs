use std::rc::Rc;

struct Employee {
    name: String,
    celery: u64,
}

fn main() {
    let a = Rc::new(Employee {
        name: String::from("Emily"),
        celery: 1000,
    });
    println!("Reference count initially  is {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);

    println!("Reference count after clone is {}", Rc::strong_count(&b));

    use_employee(&a);
    println!("Reference count after function is {}", Rc::strong_count(&a));

    if true {
        let d = Rc::clone(&a);
        println!("Reference count inside block is {}", Rc::strong_count(&d));
    }

    println!("Reference count after block is {}", Rc::strong_count(&a));
}

fn use_employee(employee: &Rc<Employee>) {
    let c = Rc::clone(&employee);
    println!(
        "Reference count inside function is {}",
        Rc::strong_count(&c)
    );
}
