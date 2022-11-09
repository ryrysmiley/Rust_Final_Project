fn main() {
    println!("Data Types: We must know the types of all variables at compile time.");
    
    //needs the u32 to change the string to something the user specifies
    //let guess: u32 = "42".parse().expect("Not a number!");

    //scalar types
    println!("Scalar type represents a single value.");
    println!("Rust has four primary scalar types:");
    println!("integers, floating-point numbers, Booleans, and characters");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}