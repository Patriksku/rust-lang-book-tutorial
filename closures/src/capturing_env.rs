// Unlike functions, Closures have access to variables, that are defined within the scope,
// in which the closure is defined. The below example would not work if equal_to_x was a function,
// as it would not have access to x outside of its scope. Closures incur more overhead than functions,
// as they have access to "more in the environment".

// When you create a closure, Rust infers which of the three Fn traits to use, based on how you use
// the values inside the closures environment. We can force the closure to e.g. take ownership of the
// values it uses inside its environment, by using the move keyword in front of the closure.
// This is mostly useful when passing a closure from one thread to another thread, so that you can
// also pass the ownership of the variables between threads.

pub fn main() {
    let x = 4;

    let equal_to_x = |z | z == x;
    let y = 4;

    assert!(equal_to_x(y));
}

// Closures capture values from their environment in three ways, which
// directly map to the three ways a function could take in input parameters:
// 1. By taking ownership,
// 2. By borrowing mutably,
// 3. By borrowing immutably.

// These three ways are encoded in the function traits:
// FnOnce, FnMut, Fn.

// FnOnce takes ownership of the variables inside the closures environment.
// 'once' here means that closures can not take ownership of the same variables
// more than once. So these closures can only be called once.

// FnMut mutably borrows values,
// Fn immutably borrows values.

// We can use the 'move' keyword to take ownership of x inside the closure.
pub fn three_ways() {
    let x = vec![1, 2, 3];
    let equal_to_x = |z | z == x;
    // let equal_to_x = move |z | z == x;
    
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}