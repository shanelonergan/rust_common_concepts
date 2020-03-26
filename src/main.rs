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
    // collection of multiple items that can have different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);

    // access individual values via index
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // ARRAYS
    // Every element must be the same type, and has a fixed length
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // FUNCTIONS
    another_function(5, 6); // hoisted from outside the main function

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // implicit return because this is an expression
}

fn plus_one(x: i32) -> i32 {
    x + 1 // placing a semicolon here would return an error because it becomes a statement
    // return x + 1; would work
}

