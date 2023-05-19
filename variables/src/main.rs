fn main() {
    let x = 5000;
    println!("The value of x is: {}", x);
    let x = "str";
    println!("The value of x is: {}", x);

    // More readable
    const SUBSCRIBER_COUNT: u32 = 100_000;

    // ***Two data types: Scalar and Compound.***
    // Scalar represent a single value.
    // Compound represent a group of values.

    // ***Scalar***
    // Integers
    // Floating-point numbers
    // Booleans
    // Character

    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let r = b'A'; // Byte (u8 only)

    let f: u8 = 255;
    let g = 2.0;
    let h: f32 = 3.0;

    let i = false;

    let j = 'z';

    // ***Compund***
    let tup = ("Rust", 100_000);

    // Unstructure tuple into variables
    let (name, count) = tup;

    // Dot notation
    let sub_count = tup.1;

    // Arrays are fixed sized.
    let arr = [200, 404, 500];
    let ok = arr[0];

    // Array with 8 values all set to 0.
    let arrWithValues = [0, 8];

    let sum = my_function(5, 10);
    println!("{}", sum);

    let number = 1;

    // Condition must be a boolean.
    if number < 10 {
        println!("1st condition");
    } else if number < 20 {
        println!("2nd condition");
    } else {
        println!("3rd condition");
    }

    // If statement inside of 'let'
    let condition = true;
    let numb = if condition { 5 } else { 6 };
}

// snake-case convention
fn my_function(x: i32, y: i32) -> i32 {
    println!("x: {} y: {}", x, y);
    return x + y;
    // We can also just type 'x + y', as the last expression in a function is explicitly returned. (we omit semicolon in the last expression)
}

fn looper() {
    let mut counter = 0;

    // We can return values in loops like so.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("{}", result);
}

/*
asd
asd
*/

fn for_looper() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{}", element);
    }

    // last number exclusive
    for number in 1..4 {
        println!("{}!", number);
    }
}
