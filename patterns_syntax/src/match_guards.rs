// A match guard is an additional condition specified after the pattern in a match arm,
// that must also match, along with the pattern, for the arm to be chosen.
//
// Useful for expressing complex ideas that patterns themselves can not express.
pub fn main() {
    println!();

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    match_guard_shadowing();
    multiple_patterns();
}

fn match_guard_shadowing() {
    let x = Some(5);
    let y = 10;

    // Match one any Some() variant for x, then bind that value to 'n', and if
    // the values is equal to the value of y, execute the first arm.
    // So basically, if x == y. Helpful if we do not want to shadow. In below example,
    // We can not use 'Some(y)' in the second arm, as that would shadow outer y.
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn multiple_patterns() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
