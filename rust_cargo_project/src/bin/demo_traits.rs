use chrono::Utc;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter, Pointer, Result};

trait Print {
    fn print(&self);
}

trait Queue {
    fn len(&self) -> usize;
    fn push_back(&mut self, m: i32);
    fn pop_front(&mut self, m: i32) -> Option<i32>;
}

trait Deque: Queue {
    fn push_front(&mut self, m: i32);
    fn pop_back(&mut self) -> Option<i32>;
}

struct MyDeque {
    data: VecDeque<i32>,
}

impl MyDeque {
    pub fn new() -> MyDeque {
        let data = VecDeque::<i32>::new();
        MyDeque { data }
    }
}

impl Queue for MyDeque {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn push_back(&mut self, m: i32) {
        self.data.push_back(m);
    }
    fn pop_front(&mut self, m: i32) -> Option<i32> {
        self.data.pop_front()
    }
}

impl Deque for MyDeque {
    fn push_front(&mut self, m: i32) {
        self.data.push_front(m);
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.data.pop_back()
    }
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

struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Employee {
    name: String,
    salary: i32,
    fulltime: bool,
}

impl Drop for Employee {
    fn drop(&mut self) {
        println!("{} is dropped", self.name);
    }
}

impl Display for Employee {
    // fn fmt(&self, f: &mut Formatter) -> Result {
    //     write!(f, "name: {} salary: {}, fulltime: {}", self.name, self.salary, self.fulltime)
    // }
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Employee")
            .field("name", &self.name)
            .field("salary", &self.salary)
            .field("fulltime", &self.fulltime)
            .finish()
    }
}

// impl Debug for Employee {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "name: {} salary: {}, fulltime: {}", self.name, self.salary, self.fulltime)
//     }
// }

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

impl Print for Point {
    fn print(&self) {
        println!("x: {} , y: {}", self.x, self.y);
    }
}

fn print_something(p: &dyn Print) {
    println!("from print_something");
    p.print();
}

fn main() {
    let employee = Employee::new("name".to_string(), 10, false);
    employee.print();
    let obj1 = Employee::new("obj1".to_string(), 10, true);
    println!("{obj1}");
    println!("{:?}", obj1);
    let obj2: Point = Point { x: 10, y: 10 };
    print_something(&obj1);
    print_something(&obj2);

    println!("\n demo trait inheritance");
    let mut d = MyDeque::new();
    d.push_back(300);
    d.push_back(400);
    d.push_back(500);
    d.push_front(200);
    d.push_front(100);

    println!("MyQueue object has {} items", d.len());

    loop {
        match d.pop_back() {
            Some(v) => println!("{}", v),
            None => break,
        }
    }
}
