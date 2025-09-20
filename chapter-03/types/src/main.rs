use std::io;

fn main() {
    // Tuple: Rust unit type is essentially an empty tuple
    let _unit_type = ();

    // Array
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // Panics if index is out of bounds, both for debug and release
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}