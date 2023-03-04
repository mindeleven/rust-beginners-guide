#[allow(unused_variables)]
#[allow(unused_assignments)]


use rand::Rng;
#[allow(unused_mut)]
fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 11);
    
    // if/else block
    if num >= 5 {
        println!("Number {} is greater than or equal to five.", num);
    } else {
        println!("Number {} is smaller than five.", num);
    }
    
    println!("--------------------------------");
    // if/else if/else block
    if num > 5 {
        println!("Number {} is greater than five.", num);
    } else if num == 5 {
        println!("Number {} is equal to five.", num);
    } else {
        println!("Number {} is smaller than five.", num);
    }

    // if statement can return result
    let res = if num >= 5 { true } else { false };
    println!("{}", res);
}
