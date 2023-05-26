pub fn main() {
    println!();
    foo(3, 4);
    println!();
    ignore_part_of_value();
    println!();
    multiple_ignores_within_one_pattern();
    println!();
    underscore_prefix();
    println!();
    ignore_with_range_syntax();
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn ignore_part_of_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // We are matching on a tuple here. So if both values are Some() variants,
    // we execute the first arm, otherwise the second arm gets executed.
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }

        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn multiple_ignores_within_one_pattern() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

// Var-names that start with an underscore are still assigned a value.
fn underscore_prefix() {
    let _x = 5;
    let y = 10;

    let s = Some("Hello!".to_string());

    // We do not care about the value stored in s, we only care if it is a Some() variant.
    // So remove replase '_s' with '_' from line below and compiler will not complain.
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

fn ignore_with_range_syntax() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    // We only care about 'x'.
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // Example with range syntax where we want to jump over some values.
    // Note: This must be written in an unanmbigous way, otherwise the compiler will complain.
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}
