use std::fmt;
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn print_v1(&self) {
       println!("In print_v1(), point is [{},{}].", self.x, self.y);
    }
    fn print_v2(self: &Point) {
        println!("In print_v2(), point is [{},{}].", self.x, self.y);
    }

    fn print_v3(self: &Self) {
        println!("In print_v3(), point is [{},{}].", self.x, self.y);
    }
    fn to_string(self: &Self) -> String {
        format!("[{},{}]", self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn main() {
    let p = Point { x: 1, y: 2 };
    p.print_v1();
    p.print_v2();
    p.print_v3();
    println!("{}", p.to_string());
    println!("{:?}",p);
    println!("{:#?}",p);
    println!("{}",p);
}