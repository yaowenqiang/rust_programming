// Crate-scope attribute
#![allow(dead_code)]
mod mytypes;

use mytypes::{Color, HouseLocation};

fn main(){
    demo_simple_enums();
    demo_enum_with_data();
}

fn demo_simple_enums() {
    println!("Demo simpleenums");
    let c:Color = Color::Red;
    match c {
        Color::Red =>println!("Red"),
        Color::Green =>println!("Green"),
        Color::Blue =>println!("Blue")
    }
}

fn demo_enum_with_data(){
   println!("\nDemo enums with data"); 
   let h = HouseLocation::Number(4);
   match h {
       HouseLocation::Number(n) =>println!("Your live in house number {}",n),
       HouseLocation::Name(s) =>println!("Your live in house nameed {}",s),
       HouseLocation::Unknown =>println!("Your house location is unknown"),
   }
}
