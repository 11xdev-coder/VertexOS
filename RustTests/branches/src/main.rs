fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("Value - {value}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break here acts like result because loop is an expression
        }
    };

    println!("Loop result - {result}");

    'loopone: loop { // loop labeling, assigning a "name" for the loop
        println!("loop1 is running!");

        loop {
            println!("loop2 is running");
            break 'loopone; // breaking from the first loop
            println!("test");
        }
        println!("loopone test!");
    }    

    let a = [10, 20, 30, 40, 50];

    for element in a { // for loop, like foreach in c#
        println!("the value is: {element}");
    }

    for number in (1..4).rev() { // reversed range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
