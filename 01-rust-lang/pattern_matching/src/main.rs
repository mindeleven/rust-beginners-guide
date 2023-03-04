// pattern matching 

// multiple values (or): 1|2
// ranges: 1..=5
// conditions: x if a > b
// tuple matching

    fn main() {
    for i in 0..15 {
        println!("{}. I have {} oranges", i, get_oranges(i));
    }
    
    // tuple matching
    // let point = (0, 0);
    let point = (12, 0);
    // example: y and x of a coordinate system
    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("({}, {})", x, y),
    }


}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of"
    }
}
