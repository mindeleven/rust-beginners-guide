use std::io;

/// Crate comment
/// What is the module trying to achieve?

fn main() {

    //! # Main function
    //! 
    //! ```
    //! fn main()
    //! ```
    //! 
    //! Reads user input and prints it to the console
    //! 
    let mut input = String::new();

    // print a message to the user
    println!("Say something");
    /*
    check response
    and act accordingly
     */
    match io::stdin().read_line( &mut input) {
        Ok(_) => {
            println!("You said {}.", input);
        },
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }
}
