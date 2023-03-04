#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // loop through a collection or range
    // execute code for each element
    for i in 1..10 {
        println!("{0} * {0} = {1}", i, i*i);
    }

    // continue will skip a step
    let pets = ["cat", "dog", "chihuahua", "hamster", "bear"];
    // iterating through collection
    for pet in pets.iter() {
        if pet == &"chihuahua" {
            println!("a {} barks too much", pet);
            // code after continue doesn't get executed
            continue
        }
        // break will stop the loop
        if pet == &"bear" {
            println!("a {} is not a pet", pet);
            break
        }
        println!("I love my {}", pet);
    }

    // getting a position and an element from a range
    // with enumerate function
    for (pos, i) in (1..11).enumerate() {
        let square = i * i;
        let nb = pos + 1; 
        println!("{0} * {0} = {1}", nb, square);
    }
    
}
