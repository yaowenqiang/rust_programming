trait Print {
    fn print(&self);
}
struct Employee {
    name: String,
    salary: i32,
    fulltime: bool,
}

impl Employee {
    fn new(name: String, salary: i32, fulltime: bool) -> Employee {
        Employee {
            name,
            salary,
            fulltime,
        }
    }
}

impl Print for Employee {
    fn print(&self) {
        println!(
            "{} earns {},fulltime: {}",
            self.name, self.salary, self.fulltime
        );
    }
}
fn main() {
    let employee = Employee::new("name".to_string(), 10, false);
    employee.print();
}
