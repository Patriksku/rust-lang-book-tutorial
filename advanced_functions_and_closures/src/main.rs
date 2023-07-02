/*
Function Pointers are functions that you pass as arguments to functions.
Useful e.g. when you want to pass a function that you have already defined,
instead of passing a Clojure.

Syntax: 'fn(type)'


Function pointers implement all three Clojure traits (Fn, FnMut, FnOnce).
So if a function expects a Clojure, you can pass in a Clojure or a function pointer.
Therefore, best practice is to write functions that accept Clojures.


One case where you would want to use a function pointer instead of a function pointers AND Clojures,
is if you are interfacing with external code that does not support Clojures.
E.g -> C functions can accept functions as arguments, but not Clojures.
 */

fn add_one(x: i32) -> i32 {
    x + 1
}

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// This is the 'best practice' version of above. Notice 'Fn()' instead of 'fn()'.
// This is a Clojure trait bound, rather than the function pointer type in function above.
// So we can pass in either a Clojuer, or a function pointer.
fn do_twice<T>(f: T, arg: i32) -> i32
where T : Fn(i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    clojure_or_predefined_function();
    example_initializer_function();

    let res = returning_clojures_from_functions(5);
    let func = res.as_ref();
    println!("{}",func(5));
}

// in the .map below, we could use either a Clojure of function pointer, e.g. 'ToString::to_string'.
fn clojure_or_predefined_function() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
    list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    println!("{:?}", list_of_strings);
}

// Another useful pattern that exploits an implementation detail of tuple structs, and 
// tuple struct enum variants. 
// Tuple structs use () to initialize values inside the tuple struct,
// which look like a function call. In fact, these initializers are implemented as functions
// that take in arguments and return an instance of that tuple struct.
//
// This means that we can use these initializers as function pointers.
// In below example, 'Status::Value' argument to .map is going to be treated like a function pointer,
// where the argument is going to be a u32, and the return value is going to be a Value(u32) variant.
// --> think 'fn(i32) -> i32'.
fn example_initializer_function() {
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// Returning clojures from functions
// Clojures are represented using traits, so we can't return a concrete type.
// Instead, we can say that we are going to return something, that implements some trait.
// --> fn returning_clojures_from_functions() -> impl Fn(i32) -> i32 {}
//
// Above syntax only works if we are returning one Clojure, this is because
// each Clojure is unique, even if they return the same thing.
//

// fn returning_clojures_from_functions() -> impl Fn(i32) -> i32 {
//     |x| x + 1
// }

// If we instead want to return different Clojures based on something,
// we can use Trait Objects. As we do not know the size of the Clojures
// being returned, we have to wrap them behind some sort of pointer.
fn returning_clojures_from_functions(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}