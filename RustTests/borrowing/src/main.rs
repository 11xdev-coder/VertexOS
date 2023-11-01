fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // we borrow s1 value, but not own it

    let s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // ALL references are mutable by default so we cant change them
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
