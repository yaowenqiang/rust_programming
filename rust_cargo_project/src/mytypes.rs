pub enum Color {
    Red,
    Green,
    Blue,
}

pub enum HouseLocation {
    Number(i32),
    Name(String),
    Unknown,
}


pub struct Employee  {
   pub name: String,
   pub salary: u64,
    pub fulltime: bool

}