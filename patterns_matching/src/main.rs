/*
Patterns are special syntax in Rust for matching against the structure of types.
A pattern consists of some combination of the following components:
* Literals
* Destructed
    - Arrays
    - Enums
    - Structs
    - Tuples
* Variables
* Wildcards
* Placeholders

These components describe the shape of the data we are working with,
which we can then match against values, to determine whether our program has the correct data
to continue running a particular piece of code.
 */
fn main() {
    // ---
    // Match expressions in different places below
    // ---
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    // We use patterns inside the arms of match expressions (left side of arm).
    match language {
        Language::English => println!("en"),
        Language::Spanish => println!("es"),
        Language::Russian => println!("ru"),
        // _ => println!("wtf"),
        lang => println!("Unsupported language! {:?}", lang),
    }

    // ---
    // Conditional if let expressions
    // ---
    // Downside of if let expressions, is that the compiler does not enforce
    // that they are exhaustive, so we could remove the last 'else' bellow and it would work,
    // even though this might introduce a defect.
    // ---
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    // So "Some(status)" is a pattern here, etc. etc.
    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    // Note, Shadowing occurs here,
    // as the variable inside of pattern is the same as the variable we are matching on.
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic")
        }
    } else {
        println!("Authorization status: guest");
    }

    // ---
    // while let Conditional Loops
    // ---

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // ---
    // for Loops
    // ---
    let v = vec!['a', 'b', 'c'];

    // Pattern here is '(index, value)'
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // ---
    // let Statements
    // ---
    // let PATTERN = EXPRESSION;
    // ---
    let x = 5;
    let (x, y, z) = (1, 2, 3);

    // ---
    // Function Parameters
    // ---

    let point = (3, 5);
    print_coordinates(&point);
    ireffutable_refutable();
}

// Deconstruction of tuple into vars 'x' and 'y'.
fn print_coordinates((x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
// ---Instead of below:
// fn print_coordinates(cords: &(i32, i32)) {
//     println!("Current location: ({}, {})", cords.0, cords.1);
// }

fn ireffutable_refutable() {
    // Irrefutable
    let x = 5;

    // Refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // The following can only accept irrefutable patterns:
    // - function parameters
    // - let statements
    // - for loops

    example1();
}

// Check errs/warns
fn example1() {
    let x: Option<&str> = None;
    let Some(x) = x;

    if let x = 5 {
        println!("{}", x);
    }
}
