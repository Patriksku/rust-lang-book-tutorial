mod bindings;
mod ignore_values;
mod match_guards;

fn main() {
    // ---
    // Literal matching
    // ---
    let x = 1;

    match x {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("_"),
    }

    // ---
    // Named variables matching
    // ---
    let x = Some(10);
    let _y = 10;

    // 'y' is the named variable here
    // Note, this is a new scope, so variables inside will shadow the variables outside of this scope.
    match x {
        Some(50) => println!("50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // ---
    // Matching multiple patterns
    // ---
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ---
    // Matching ranges of values
    // ---
    // Note: range operator only works on numbers and chars
    // ---
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    destructure_struct();
    destructure_struct_match();
    destructure_enum();
    destructure_enum_in_enum();
    destructure_example();

    ignore_values::main();
    match_guards::main();
    bindings::main();
}

struct Point {
    x: i32,
    y: i32,
}

/*
Patterns that destructure values

We can destructure:
    - Structs,
    - Enums,
    - Tuples,
    - References.
*/

fn destructure_struct() {
    let p = Point { x: 0, y: 7 };

    // let Point { x: a, y: b } = p;
    // assert_eq!(0, a);
    // assert_eq!(7, b);

    // It is more common to have variables inside of a pattern, match the field names:
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn destructure_struct_match() {
    let p = Point { x: 0, y: 7 };

    // When we are destructing a struct, we can use named variables, and literals
    match p {
        Point { x, y: 0 } => {
            println!("on the x axis at {}", x)
        }
        Point { x: 0, y } => {
            println!("on the y axis at {}", y)
        }
        Point { x, y } => {
            println!("on neither axis: ({}, {})", x, y)
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructure_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("Move to x: {} y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color: red {}, green {}, blue {}", r, g, b);
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum AdvMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructure_enum_in_enum() {
    let msg = AdvMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        AdvMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color: red {}, green {}, and blue {}", r, g, b);
        }
        AdvMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color: hue {}, saturation {}, and value {}", h, s, v);
        }
        _ => println!("hmm.."),
    }
}

fn destructure_example() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
