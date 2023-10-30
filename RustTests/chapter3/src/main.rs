fn main() {
    const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3; // consts are ALWAYS IMMUTABLE (cant use mut keyword on them); type MUST be always pre-defined
    println!("Const: {THREE_HOURS_IN_SECONDS}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // mut makes variables that can change their values
    println!("The value of x is: {x}");

    // SHADOWING
    let y = 5;

    let y = y + 1; // define a variable with the same name, but different value

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let _spaces = "   "; // with shadowing we can change a type complepetly
    let _spaces = _spaces.len();

    // let mut spaces = "   "; // THIS WONT RUN because intial type was string
    // spaces = spaces.len();

    println!("----------------------DATA TYPES----------------------");
    // there are signed (can be negative) and unsigned (cant be negative) integers
    // each integer can store -2 in N-1 degree to 2 in N-1 degree where N - amount of bits the type uses
    // isize and usize depends on arch of your PC: 64 bits if you are on 64 bit arch, and 32 bits if on 32 bit arch

    // Number Literals	Example
    //  Decimal	        98_222
    //  Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'

    // when your variable stores from 0 to 255, and you try to change its value to 256, INTEGER OVERFLOW will occur
    // when compiling this on debug mode and if INTEGER OVERFLOW occurs, the code will PANIC

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient: f32 = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("{sum}; {difference}; {product}; {quotient}; {truncated}; {remainder}");
    
    // BOOL
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("{t}, {f}");

    // CHAR
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");

    // TUPLES
    // tuple has fixed length
    let _tup1: (i32, bool, f64) = (32, false, 54.02); // pre-defined types

    let tup2 = (500, 6.4, 1);
    let (val1, val2, val3) = tup2; // get individual elements from tuple
    println!("{val1}, {val2}, {val3}");

    // we can access a tuple's element directly:
    let tup3 = (54.4, false, "huh");
    let num = tup3.0;
    let boolStatement = tup3.1;
    let huh = tup3.2;
    println!("{num}, {boolStatement}, {huh}");
    
    // ARRAYS
    // arrays have fixed length; all elements MUST be SAME TYPE
    let a = [2,3,4,5];
    let preDefined: [bool; 3] = [true, true, false]; // all booleans, 3 elements
    let lazy = [3; 5]; // five 3's, same as [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];
    println!("{first}, {second}");
}
