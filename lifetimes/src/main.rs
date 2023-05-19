// Generic lifetime annotations describe the relation between the lifetimes of multiple references, and how they relate to each other.
// So they do not change the lifetime, they just explain the relation between different lifetimes.
// GLA always start with a '.
// GLA are a type of Generics.
// Convention -> start naming them with a lower level a -> 'a.
//     and continue from there.

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
mod bc;
mod elision;
mod finale;
mod structs;

fn main() {
    let r;
    let x = 5;
    r = &x;

    println!("{:p}", r);

    //

    // The borrow checker works like this:
    // We first use GLA in fn longest() to reference the return value, to be the same as the smallest lifetime of the argument(s).
    // The borrow checker then simply compares the arguments (string1 and string2) and pick the smallest lifetime, and looks
    // if that is still valid at the last place where we use 'result'.
    // both string1 and string2 live until the end of this function, therefore it knows that everything is safe!
    // Basically, result inherits the smallest lifetime of the arguments being passed in to fn longest()
    let string1 = String::from("abcd");
    let string2 = String::from("abc");

    let result = longest(&string1, &string2);
    println!("longest text is: {}", result);

    bc::example1();
    structs::main();

    let a = "asd";

    // Static lifetime, means that the reference can live as long as the duration of the program.
    // All string literals "&str" have a static lifetime. This is because they are stored in the program's binary.

    let s: &'static str = "I have a static lifetime.";
}

// Below means -> The lifetime of the returned reference, will be the same as the smallest lifetime of the argument!
// ...so if x has a smaller lifetime than y, then the returned reference lifetime will be the same as the lifetime of x.
// We define our lifetime, and reference it (annotate with) to a and b like so:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }

    y
}

// This will not work!
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         return x;
//     }

//     y
// }
