use chrono::Utc;

trait Print {
    fn print(&self);
}

trait Log {
    const LOG_TIMESTAMP: bool = false;
    fn log(&self);
    fn log_verbose(&self) {
        println!("-----------------------");
        if Self::LOG_TIMESTAMP {
            println!("{} ", Utc::now());
        }
        self.log();
        println!("-----------------------");
    }
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

impl Log for Employee {
    const LOG_TIMESTAMP: bool = true;
   fn log(&self) {
        println!("{},{},{}", self.name, self.salary, self.fulltime);
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
