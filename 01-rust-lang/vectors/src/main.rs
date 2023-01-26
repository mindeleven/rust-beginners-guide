#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    // vectors are very similar to arrays
    // the only difference is the vectors are arrays of variable size
    // vectors don't exist as a type in rust
    // created (1) through new function of the vector class (structure vector)
    let primes: Vec<i32> = Vec::new();
    // or (2) with the macro vec!
    let mut primes = vec![2, 3, 5, 7, 11];
    println!("{:?}", primes);

    // adding elements (not possible with arrays)
    primes.push(13);
    println!("{:?}", primes);
    // remove element
    primes.remove(2); // removes element at index position
    println!("{:?}", primes);

    // create vectors with default values
    // in a similar way how we do it for arrays
    let mut numbers = vec![2;10];
    println!("{:?}", numbers);
    const DEFAULT: bool = true;
    let values = vec![DEFAULT;8];
    println!("{:?}", values);

    numbers[5] = 8; // vector is mutable
    println!("{:?}", numbers);

    // using an iterator 
    for number in numbers.iter() {
        println!("{}", number * number);
    }
}
