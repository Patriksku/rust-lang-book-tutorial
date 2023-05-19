enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 4 variants
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32), // Tuple of integers
}

impl Message {
    fn some_function() {
        println!("oh ya.");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let fourA = IpAddrKind::V4;
    let sixA = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(192, 168, 1, 1);

    // In Rust we do not have null :)
    // Instead, we use the Option<> enum, which has the:
    // - Some(), or
    // - None()
    // variants.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Handles both if y is Some(), or None.
    let sum = x + y.unwrap_or(0);

    println!();
    println!();
    println!();

    value_in_cents(Coin::Quarter(UsState::Alaska));

    println!();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!();

    // If let Syntax example
    // ---------------------
    let some_value = Some(3);
    // match some_value {
    //     Some(3) => println!("three"),
    //     _ => (), // Do nothing.
    // }
    // ---------------------
    // Could be rewritten as:
    // (Easier to read in revers -> "if some_value equals to Some(3)".)
    if let Some(3) = some_value {
        println!("three");
    }
    // This is read as: If some_value matches Some(3), print "three".
    // Match expression is exhaustive. With 'If let Syntax',
    // we only have to specify the pattern we care about. All other patterns are ignored.
    // --> A bit confusing, use whatever works for you. Just know that this exists.
}

/*
Match Expressions
Match allows to compare a value against a set of patterns.
Patterns could be literals, variables, wildcards, etc.

- Is exhaustive, we have to match all cases (useful for enums).
*/

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // Below serves as an "else". It gets executed if it is "any other pattern" than those specified.
        // _ => None
    }
}
