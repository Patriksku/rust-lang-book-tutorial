use core::slice;

mod unsafe_trait;
/*
If we want to opt out of memory-safety guarantees, then we can use Unsafe Rust.

Two reasons for Unsafe Rust:
1. Static analysis is conservative by nature. So Rust will sometimes reject a valid program if it cannot
    guarantee that it is memory safe.
2. The underlying computer hw is inherently unsafe. So if Rust did not allow you to do unsafe
    operations, then you would not be able to do certain tasks. And Rust is a systems programming language,
    so it must allow you to do low-level systems programming, which sometimes requires unsafe code.

The 'unsafe' keyword gives you 5 abilites that you do not have in the safe Rust world:
1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static variable
4. Implement an unsafe trait
5. Access fields of unions

...
But unsafe does not turn off the borrow checker, or disable Rust safety checks.
...
By keeping the unsafe blocks small and only using them when necessary,
the possible surface of memory issues becomes small, and also easier to debug.

Another thing that is possible, is to enclose unsafe code in a safe abstraction,
and providing a safe API.

...

Differences between references and smart pointers, compared to raw pointers
- Raw pointers are allowed to ignore Rust's borrowing rules by having mutable and immmutable pointers,
    or multiple mutable pointers, to the same location in memory.
- Raw pointers are not guaranteed to point to valid memory.
- Raw pointers are allowed to be null.
- Raw pointers do not implement any type of automatic cleanup.


Using unsafe is not wrong or frowned upon. When you have a use case for unsafe, then use it!
Just make sure you are being safe with memory.
*/
fn main() {
    dereference_a_raw_pointer();
    call_an_unsafe_function_or_method();
    safe_abstraction();
    external_code_to_call();
    access_or_modify_mutable_static_variable();
    access_fields_of_unions();

    unsafe_trait::main();
}

// Unsafe Rust has two type of raw pointers that are similar to references:
// 1. A mutable raw pointer '*mut',
// 2. An immutable raw pointer '*const'.
// The '*' is NOT a dereference operator here, this is simply the syntax.
//
// Immutable, in the context of raw pointers, means that the pointer cannot be directly assigned,
// after it has dereferenced.
//
// We can create raw pointers without unsafe keyword, but we cannot dereference them without unsafe keyword.
fn dereference_a_raw_pointer() {
    let mut num = 5;

    // Immutable and mutable raw pointers, both pointing to the same place in memory!
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r1 is: {}", *r2);
    }
}

// What an unsafe fn means, is that when calling such a fn, we need to give it correct arguments,
// otherwise, we might encounter undefined behaviour.
// By calling an unsafe fn, you are saying that you have read the fn's doc, and you are taking
// responsibility for upholding the functions contracts.
fn call_an_unsafe_function_or_method() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn safe_abstraction() {
    let mut v = vec![1, 2, 3, 4, 5];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5]);
}

// fn split_at_mut_no_work(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);

//     // Borrow checker does not understand that we are mutably borrowing different parts of the slice.
//     // But our code is safe. So we could use unsafe here.
//     (&mut slice[..mid], &mut slice[mid..])
// }

// Recall that slices are a pointer to some data, along with the length of that data.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Functions that call external code
//
// Sometimes your Rust code may need to interact with code in a different language.
// For this purpose, Rust has the 'extern' keyword, which facilitates the creation
// and use of a foreign function interface (FFI).
//
// FFI is a way for a programming language to define a function that another language (foreign language),
// could call.
//
// Calling an extern block is always unsafe, as we do not know if the target language allows for unsafe code.

// Within an extern call, we define a name and a signature of the foreign function we want to call.
// 'C' here defines which application binary interface (ABI) that external function uses.
// The ABI defines how to call the function at the assembly level.
// The 'C' ABI is the most common ABI, and follows the C programming language ABI.
extern "C" {
    fn abs(input: i32) -> i32;
}

fn external_code_to_call() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(3));
    }
}

// We can also allow other languages to call our Rust functions, by using the extern keyword in a fn signature.
// We also have to add this 'no_mangle' annotation, to let the Rust compiler know, not to mangle the name of our fn.
// Mangleing is when the compiler changes the name of a fn, to give more information for other parts of the compilation process.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!")
}

// Global (static) variables can be problematic with Rust's ownership rules, e.g:
// If two threads are accessing the same mutable global state, then it can cause a data race.
// Also, we must annotate the type of our static variable.
//
// Static variables must have a static lifetime.
//
// Difference between static and const variables:
// - Static variables have a fixed address in memory.
// - Constants are allowed to duplicate their data, whenever they are used.
//      -> If you are referencing a constant throughout your code base, then the compiler can replace all those instances
//          of a constant, with a concrete value.
// - Static variables can be mutable, but accessing and modifying mutable static variables, is unsafe.
//
//
//
//
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn access_or_modify_mutable_static_variable() {
    add_to_count(3);

    unsafe { println!("COUNTER: {COUNTER}") }
}

// A union is similar to a struct, but only one field is used for each instance.
// Unions are primarily used to interface with C unions, and it is unsafe to access fields of a union,
// because Rust can not guarantee, what the type of data stored in the union is, for a given instance.
// EMPTY
fn access_fields_of_unions() {}
