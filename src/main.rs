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
}
