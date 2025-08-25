use rust_cargo_project::mytypes::Employee;

fn main() {
    do_it();
    create_employ();
    create_mutale_employ();
    let mut employee = Employee {
        name: String::from("Jane"),
        salary: 1000,
        fulltime: true,
    };
    print_employee(&employee);
    reward_employee(&mut employee);
    print_employee(&employee);
}

fn do_it() {
    println!("\nIn demo_accessing struct.");
    let el: Employee;
    let e2: Employee;

    let size = std::mem::size_of::<Employee>();
    println!("The size of Employee is {} bytes.", size);
}

fn create_employ() {
    let el = Employee {
        name: String::from("Jane"),
        salary: 1000,
        fulltime: true,
    };
    println!(
        "{} earns {}, fulltime status: {}",
        el.name, el.salary, el.fulltime
    );
}

fn create_mutale_employ() {
    let mut el = Employee {
        name: String::from("Jane"),
        salary: 1000,
        fulltime: true,
    };
    el.salary = 2000;
    println!(
        "{} earns {}, fulltime status: {}",
        el.name, el.salary, el.fulltime
    );
}

fn print_employee(emp: &Employee) {
    println!("{}", emp.name);
    println!("{}", (*emp).name);
    println!("name:{}, salary: {}", emp.name, emp.salary);
}

fn reward_employee(e: &mut Employee) {
    (*e).salary += 500;
    e.salary += 500;
}
