fn main() {
    println!("Blabla");
    another_function();
    parameter_function(true, 32);
    print_labeled_measurement(5, 'h');

    // statements dont return values
    // Expressions evailate values; calling a function, calling a macro and a new scope are an expressions.
    let y = {
        let x = 5;
        x + 1
    };
    println!("y is: {y}");

    // {
      //  let x = 5;
     //   x + 1
    //}; expression
    
    let five = five();
    println!("{five}");
}

fn another_function() {
    println!("Another function");
}

fn parameter_function(x: bool, y: i32) {
    println!("Got x: {x}, y: {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 { // -> return type
    let temp_value = 4 + 1;
    temp_value
}