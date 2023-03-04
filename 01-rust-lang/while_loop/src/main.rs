fn main() {
    get_squares(3478);
    println!("--------------");
    get_cubes(4938);
}

fn get_squares(limit: i32) {
    let mut x = 1;
    // loop as long as condition is true
    // continue skips a step
    while x * x < limit {
        println!("{0} * {0} = {1}", x, x*x);
        x += 1;
    }
}

// loop that will only stop when we stop it ourselves
// equivalent to 'while true' loop
// break stops loop
fn get_cubes(limit: i32) {
    let mut x = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", x, x*x);
        x += 1;
        if x * x * x > limit {
            break
        }
    }
}
