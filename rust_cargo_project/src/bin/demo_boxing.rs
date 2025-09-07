struct Employee {
    name: String,
    salary: u64,
}
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Node {
        Node { data, next: None }
    }
    fn append(&mut self, data: i32) {
        match self.next {
            None => {
                self.next = Some(Box::new(Node::new(data)));
            }
            Some(ref mut boxed_next_node) => {
                boxed_next_node.append(data);
            }
        }
    }

    fn display(&self) {
        println!("{}", self.data);
        match self.next {
            None => {
                println!("[End]");
            }
            Some(ref boxed_next_node) => {
                boxed_next_node.display();
            }
        }
    }
}

struct Chain {
    head: Option<Box<Node>>,
}

impl Chain {
    fn new() -> Chain {
        Chain { head: None }
    }

    fn insert(&mut self, data: i32) {
        match self.head {
            None => {
                self.head = Some(Box::new(Node::new(data)));
            }
            Some(ref mut boxed_head_node) => {
                boxed_head_node.append(data);
            }
        }
    }

    fn display(&self) {
        match self.head {
            None => {
                println!("[Empty chain]");
            }
            Some(ref boxed_head_node) => {
                boxed_head_node.display();
            }
        }
    }
}

fn main() {
    let boxed_number = Box::new(42);
    println!("Explicitly dereferenced value: {}", *boxed_number);
    println!("Implicitly dereferenced value: {}", boxed_number);

    let value: i32 = *boxed_number;
    println!("Value of value: {}", value);

    do_it();

    let mut chain = Chain::new();
    chain.insert(42);
    chain.insert(11);
    chain.insert(22);
    chain.insert(33);
    chain.display();
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
