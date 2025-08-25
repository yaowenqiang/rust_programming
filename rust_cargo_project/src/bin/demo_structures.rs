use std::iter::repeat_with;
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

    let em2 = build_employee("Jack".to_string(), 100, false);
    print_employee(&em2);

    let mut em3 = build_employee("Joy".to_string(), 1000, false);
    print_employee(&em3);

    // let mut em4 = build_employee2("Joy".to_string(), 1000, false);
    // print_employee(&em4);

    let c1 = choose_employee(&em2, &em3);
    let mut em5 = build_employee("Joy 5".to_string(), 1000, false);
    let mut em6 = build_employee("Joy 6".to_string(), 1000, false);
    print_employee(c1);
    print_employee(&c1);
    choose_employee(&em5, &em6);
    let em7 = choose_mutable_employee(&mut em5, &mut em6);
    print_employee(em7);
    reward_employee(em7);
    print_employee(em7);
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


fn build_employee(name: String, salary: u64, fulltime: bool) -> Employee {
    Employee {
        name,
        salary,
        fulltime
    }
}

// fn build_employee2(name: String, salary: u64, fulltime: bool) -> &Employee {
//     &Employee {
//         name,
//         salary,
//         fulltime
//     }
// }


fn choose_employee<'a> (e1: &'a Employee, e2:&'a Employee) -> &'a Employee {
    if e1.salary >e2.salary {e1} else {e2}
}

fn choose_mutable_employee<'a> (e1: &'a mut Employee, e2:&'a mut Employee) -> &'a mut Employee {
    if e1.salary >e2.salary {e1} else {e2}
}
