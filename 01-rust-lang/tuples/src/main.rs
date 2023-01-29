fn main() {
    // tuples are collections of various types 
    // variables of different type collected together in a structure 
    // tuples are static (cannot be resized)
    // element values can be updated
    // indexed
    // limited to 12 elements 

    // let person = ("John", 27, true);
    // specifying the types we're expecting
    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("{:?}", person);

    // accessing elements by index with dot notation
    println!("Name: {}", person.0);

    // updating elements is possible as long as the variable is mutable
    person.0 = "Mike";
    println!("Name: {}", person.0);

    // destructuring a tuple
    // number of variables has to correspond with the number of elements
    let (name, age, employed) = person;
    println!("About this person:");
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Employed: {}", employed);

}
