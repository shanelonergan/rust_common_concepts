fn main() {
    // standard variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("the most points you can get is {}", MAX_POINTS);

    // shadowing
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // Integers and Floats
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is: {}. The value of y is: {}", x, y);

    // NUMERIC OPERATIONS
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // BOOLEANS
    let t = true;
    let f: bool = false; // with explicit type annotation

    // CHARACTERS
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // TUPLES
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);

    access individual values
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
