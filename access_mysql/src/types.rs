use std::fmt::{Display, Formatter, Result};

pub struct Employee {
    pub employee_id: i32,
    pub name: String,
    pub salary: f64,
    pub region: String,
}

impl Employee {
    pub fn new(employee_id: i32, name: String, salary: f64, region: String) -> Employee {
        Employee {
            employee_id,
            name,
            salary,
            region,
        }
    }
}

impl Display for Employee {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} {} {} {}",
            self.employee_id, self.name, self.salary, self.region
        )
    }
}
