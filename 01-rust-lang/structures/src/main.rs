#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    let emp1 = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35
    };
    // printing only works with the #[derive(Debug)] annotation
    println!("{:?}", emp1);

    // printing out specific element with the dot notation
    println!("Name of employee: {}", emp1.name);

    // printing the method
    println!("{}", emp1.fn_detail());

    // printing the static method using the structure itself
    // not an implementation
    println!("{}", Employee::static_fn_detail());
}

// structures
// key-value pairs, equivalent to dictionaries in other languages
// similar to a class, take over the task of classes/objects
// a collection of key-value pairs
// to print structure we've to annotate it
#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32
}

// adding methods to a structure with the impl keyword
impl Employee {
    fn fn_detail(&self) -> String {
        // no semicolon at the end to be used as a return value
        format!("name: {}, age: {}, company: {}", &self.name, &self.age, &self.company)
    }

    // a structure can have static methods too
    // a static method doesn't take the self parameter
    // can be used as a general functionality for this particular structure
    fn static_fn_detail() -> String {
        String::from("Details of a person")
    }
}

