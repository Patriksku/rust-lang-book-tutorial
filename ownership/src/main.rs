fn main() {
    // ***Ownership rules***
    // 1. Each value in Rust has a variable thats called its owner.
    // 2. The can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    {
        let a = String::from("hello"); // now stored on the heap as not str literal.
    }

    {
        let mut s = String::from("hello world");
        let hello = &s[..5];
        let world = &s[6..];

        let word = &s[..];
        println!("the first word is: {}", word);
        s.clear();
    }

    variables();

    let s = String::from("yep!");
    takes_ownership(s);
    // println!("{}", s); // wont work

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership(); // Ownership of string from gives_ownership() moves to s1.
    println!("s1 = {}", s1);

    let s2 = String::from("ajahua");
    let s2 = takes_and_gives_back(s2);
    println!("s2 = {}", s2);

    // ***Reference***
    // References do not take ownership of the underlying value.
    // Passing references as function params is called borrowing, ->
    // -> because we are borrowing the value, and not actually taking ownership of it.
    let s3 = String::from("hello");
    let len = calculate_length(&s3); // Here we pass a reference to s3 (without '&' s3 would be moved instead).
    println!("The length of '{}' is '{}'.", s3, len);

    // ***Mutable Reference***
    // Restriction 1: You can only have one mutable reference to a particular piece of data, ->
    // -> in a particular scope.
    // BIG PRO: Rust prevents data races at compile time.
    //
    // Restriction 2: You cant have a mutable reference, if an immutable reference already exists.
    // You can however, have multiple immutable references, because the underlying data is not going to change.
    let mut s4 = String::from("mut ref");
    mut_ref(&mut s4);
    println!("s4 = {}", s4);
}

fn variables() {
    // Copy was done here as integers, characters etc. have a Copy type/trait, which allows for Copy, instead of Move.
    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1; // Move, not shallow copy (s1 moved to s2)
                 // println!("{}", s1);

    let cop1 = String::from("word");
    let cop2 = cop1.clone(); // Copy
    println!("{}", cop2);
}

// Takes ownership of the string parameter, and then gets dropped after function returns.
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// Makes copy because of the integers copy trait.
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// Moves ownership of this string to caller.
fn gives_ownership() -> String {
    let s1 = String::from("yaba");

    s1
}

// Takes ownership of s2, and the gives the ownership back to the caller.
// This is a bit tedious, use reference example when possible.
fn takes_and_gives_back(s2: String) -> String {
    s2
}

// s is dropped after function return. That is ok because:
// s -> s3 -> heap.
// Referenes are immutable by default.
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

// Mutable reference, without taking ownership of the underlying value.
// s -> s3 -> heap.
fn mut_ref(s: &mut String) {
    s.push_str("erence");
    println!("mut ref = {}", s);
}
