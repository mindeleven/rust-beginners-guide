#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    // array data types in rust 
    // a collection of values of the same type 
    // arrays are static and cannot be resized
    // elements can be modified if array is mutable but not deleted
    // array is indexed

    // array of integers 
    let primes = [2, 3, 5, 7, 11];

    // explicitly defining an array with type and size 
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];

    // to print arrays debug printer is needed
    println!("{:?}", primes);
    println!("{:?}", doubles);

    // create array with default values
    let mut numbers = [0;15];
    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;
    // shadowing the numbers variable 
    let mut numbers = [DEFAULT;15];
    println!("{:?}", numbers);

    // updating elements via the index
    numbers[3] = 666;
    println!("{:?}", numbers);

    // using an iterator
    for number in numbers.iter() {
        println!("{}", number + 3)
    }


}
