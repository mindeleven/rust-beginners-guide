use crate::Colors::Red;
use crate::Person::Name;

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    // use enum
    let my_color_red = Colors::Red;
    let my_color_blue = Colors::Blue;
    let my_color_green = Colors::Green;
    println!("My favorite colors are {:?}, {:?} and {:?}", my_color_red, my_color_green, my_color_blue);

    // direct access via use crate::Colors::Red at top
    let my_color = Red;

    let person = Name(String::from("Alex"));
    println!("{:?}", person);

}

// enums are an enumeration of values
// a collection of values 
// defined with the enum keyword
#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue
}

// add data types to enum elements
#[allow(dead_code)]
#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32)
}