use crate::Colors::Red;

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    let p1: Point<i32> = Point {x: 8, y: 7};
    let p2: Point<f64> = Point {x: 3.25, y: 8.63};
    println!("{:?}", p1);
    println!("{:?}", p2);

    let c1 = Red("#f00");
    let c2 = Red(255);
    println!("{:?}, {:?}", c1, c2);

    let ca1: Calc<i32, f64> = Calc {x: 6, y: 12.89};
    println!("{:?}", ca1);
}

// generics as a way to have different types of values in our structures or enums
// we have a structure that takes a generic type 
// which can be marked foe example with <T> as a placeholder
#[allow(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

#[allow(dead_code)]
#[derive(Debug)]
enum Colors<T> {
    Red(T),
    Blue(T),
    Green(T)
}

// using multiple generics
#[allow(dead_code)]
#[derive(Debug)]
struct Calc<T, V> {
    x: T,
    y: V
}
