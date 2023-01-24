// import Rng for integers and floats
// import thread_rng for strings
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn main() {
    // generate random integer
    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);

    // bounded generation
    // generate integer in a bounded range
    println!("bounded int: {}", rng.gen_range(0, 100));

    // generate float in a bounded range
    println!("bounded float: {}", rng.gen_range(0.0, 100.0));

    // random string
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    println!("gen string: {}", rand_string);

}


