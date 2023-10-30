use std::io;

fn main() {
    let a: [i32; 5] = [1,2,3,4,5];

    println!("Enter an array index:");

    let mut array_index = String::new();

    io::stdin()
        .read_line(&mut array_index)
        .expect("Failed to read");

    let array_index: usize = array_index.trim()
        .parse()
        .expect("Failed to parse");

    let element = a[array_index];
    println!("Array at {array_index} = {element}");
}
