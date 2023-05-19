// Derefs allow you to treat pointers like regular references.
// -> Allows you to customize the behavior of the dereference operator '*'.

    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);  --> Here, '*y' means that it will follow the pointer to what is stored at that memory address.
    //                         So 'y' is a pointer, and by dereferncing it, we get the value that is stored at that address.
//
// Without the deref trait, the compiler only know how to dereference references.
//
// Why does deref return a reference, instead of the value itself in our MyBox implementation?
// Because of Rust's ownership system. Otherwise, it would move the ownership outside of our smart pointer.
// In most cases we don't want this.
//
// We can also use DerefMut trait to override the dereference operator for mutable references.

    use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// We define the dereference behavior here.
// This makes it possible for us to dereference our MyBox struct.
// When we dereference, we return a reference to the first item of our tuple in MyBox (we only have one).
impl<T> Deref for MyBox<T> {
    // Associated type (will be covered in chapter 19).
    type Target = T;

    // &Self::Target can be replaced with &T.
    fn deref(&self) -> &Self::Target {
        &self.0 // first item in our tuple.
    }
}

pub fn main() {
    let x = 5;
    let y = MyBox::new(x); // Just like our reference (ex. above), Box is pointing to a value stored somewhere in memory.

    assert_eq!(5, x);
    assert_eq!(5, *y); // Same as below:  assert_eq!(5, *(y.deref()));
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Above hello() call works because:
    //      - MyBox and String implement the deref trait. By dereferencing &MyBox<String> we get &String, 
    //      and by dereferencing it we get &str:
    //          &MyBox<String> -> &String -> &str
    //
    // These chained deref calls are called deref coercion.
    // Rust will do these automatically at compile time, when it sees that passed in type is not matching.
    // Without this, our: '&m' would have to be written as: &(*m)[..]);
}

// Implicit deref coercion with functions and methods.
// Deref coercion is a convenience feature that happens automatically for types that implement the deref trait.
// Deref coercion will convert a reference to one type, to a reference to a different type.
fn hello(name: &str) {
    println!("Hello, {}!", name);
}