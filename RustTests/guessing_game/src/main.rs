use std::io; // std - standard library ; io - input/output library
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100); // range expression - start..=end
    // println!("Secret number {secret_number}");

    loop { // looping infinitely
        println!("Enter your guess: ");
        // let - define a variable    
        // mut - make a value mutable
        // immutable - value cant change
        // mutable - can change
    
        // :: - structure methods
        // . - methods
    
        let _apples = 5; // immutable
        let mut guess = String::new(); // mutable
        // String::new() - return a new (empty) string
    
        io::stdin() // use the io library
            .read_line(&mut guess)  // read the input and store it in guess; & indicates that it is a reference (data can be changed and stored without copying it into memory); refernces are immutable by default, so we placed mut here to make it mutable
            .expect("Failed to read"); // should always add this line, if we get an error from reading this line, this will print out
    
        // using Shadowing - reusing the variable
        let guess:u32 = guess.trim() // trim eliminates any not needed spaces or symbols like \n or \r, so our variable can be safely parsed
            .parse() {  // parsing or converting the string to another type
                Ok(num) => num, // since parse return a `Result`, we can use Ok and Err values; Ok - if parsed successfully
                Err(_) => continue, // Err - if parsing failed
            };
            
    
        println!("You guessed {guess}");
    
        match guess.cmp(&secret_number){ // match is like switch statement, match variable_1.cmp($variable_compare_to) { Ordering checks => execute_if_true, }
            Ordering::Less => println!("Too small"), // if our value is less than variable_compare_to, this runs
            Ordering::Greater => println!("Too big"), // and so on
            Ordering::Equal => { // brackets for multiple lines
                println!("You won");
                break; // breaking from the loop
            },
        }
    }    
}