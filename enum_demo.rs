// Crate-scope attribute
#![allow(dead_code)]
mod mytypes;

use mytypes::{Color, HouseLocation};

fn main() {
    demo_simple_enums();
    demo_enum_with_data();
    demo_using_option_enum();
}

fn demo_simple_enums() {
    println!("Demo simpleenums");
    let c: Color = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

fn demo_enum_with_data() {
    println!("\nDemo enums with data");
    let h = HouseLocation::Number(4);
    match h {
        HouseLocation::Number(n) => println!("Your live in house number {}", n),
        HouseLocation::Name(s) => println!("Your live in house nameed {}", s),
        HouseLocation::Unknown => println!("Your house location is unknown"),
    }
}

fn demo_using_option_enum() {
    println!("\nDemo using the Option<T> enum");

    let sec: Option<u32>;
    sec = sec_of_day(23, 59, 59);
    match sec {
        Some(s) => println!("Second of day:{}", s),
        None => println!("Second of day: no value available"),
    }

    println!("Unwrapped sec:{}", sec.unwrap_or(0));
}

fn sec_of_day(h: u32, m: u32, s: u32) -> Option<u32> {
    if h <= 23 && m <= 59 && s <= 59 {
        let secs = h * 3600 + m * 60 + s;
        return Option::Some(secs);
    } else {
        return Option::None;
    }
}
