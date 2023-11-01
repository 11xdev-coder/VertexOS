fn main() {
    // Stack is organized, has data with fixed size
    // Heap is less organized, has data with size that can change
    // Each value has an owner
    // Only one owner at a time
    // when owner is out of scope, value is dropped

    { // s is not valid, it is not declared
        let _s = "yay"; // valid from now

        // do stuff with s
    } // scope over, s is not valid

    let mut some_string = String::from("He"); // create a new string using  "from"
    some_string.push_str("-He");  // add another string to already existing string, this results in "He-He"
    println!("{some_string}");  

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
}  // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
