#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    // a slice is a pointer to a block of memory
    // size of a slice is determined on runtime
    // can be used on arrays, vectors and strings
    // slices are indexed
    let numbers = [1, 2, 3, 4, 5];
    // slice &numbers[1..4] starts at position 1 
    // and ends before position 4
    let slice = &numbers[1..4];
    println!("{:?}", slice);

    // mutable slices allow us to change values
    let mut colors = ["red", "green", "blue", "pink"];
    println!("{:?}", colors);
    // changing slices impacts and mutable array
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);

}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
    // colors_slice[2] = "brown";
} 