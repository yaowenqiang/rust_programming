struct Employee {
    name: String,
    salary: u64,
}

fn main() {
    let boxed_number = Box::new(42);
    println!("Explicitly dereferenced value: {}", *boxed_number);
    println!("Implicitly dereferenced value: {}", boxed_number);

    let value: i32 = *boxed_number;
    println!("Value of value: {}", value);

    do_it();
}

fn do_it() {
    println!("\nIn demo box");
    let boxed_emp = Box::new(Employee {
        name: "Test".to_string(),
        salary: 42,
    });
    process_employee(boxed_emp);

    // println!("{} earns {}",boxed_emp.name, boxed_emp.salary );
}

fn process_employee(emp: Box<Employee>) {
    println!("{} earns {}", emp.name, emp.salary);
}
