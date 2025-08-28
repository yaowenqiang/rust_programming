use std::fmt;
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
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

    fn reset_v1(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    fn reset_v2(self: &mut Point) {
        self.x = 0;
        self.y = 0;
    }

    fn reset_v3(self: &mut Self) {
        self.x = 0;
        self.y = 0;
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D{
            x,
            y,
            z
        }
    }
}
fn main() {
    let mut p = Point { x: 1, y: 2 };
    p.print_v1();
    p.print_v2();
    p.print_v3();
    println!("{}", p.to_string());
    println!("{:?}",p);
    println!("{:#?}",p);
    println!("{}",p);

    p.reset_v3();
    p.reset_v2();
    p.reset_v1();
    p.move_by(10, 20);
    println!("reset Point");
    println!("{}", p.to_string());
    println!("{:?}",p);
    println!("{:#?}",p);
    println!("{}",p);

    let p3 = Point3D::new(2, 3, 4);
    println!("{:?}", p3);
    let mut p4 = Point3D::new(2, 3, 4);
    println!("{:?}", p4);
}