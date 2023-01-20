#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // rust is a strongly typed language
    // variables are declared with the let keyword
    let name = "Fyodor Mikhailovich";
    let age = 201;
    println!("{} is {} years old.", name, 2022-1821);
    
    // large number needs to be explicitly defined as i64
    let amount: i64 = 9787887665654;

    // variales are immutable by default
    // can be declared as mutable with the mut keyword
    let mut new_age = 1968;
    new_age = 1957;
    println!("Now the new age begins in {}!", new_age);

    // shadowing in allowed in rust
    // the same variable can be declared twice
    let color = "blue";
    let color = "red";
    println!("{}", color);

    // declaring multiple variables simultaniously
    let (a,b,c) = (1,2,3);
    print!("{} + {} + {} = {}", a, b, c, a+b+c)

}
