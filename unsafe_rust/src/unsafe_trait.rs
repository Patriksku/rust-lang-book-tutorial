/*
A trait is unsafe, when at least one of its methods is unsafe.
*/
unsafe trait Foo {
    // methods
}

unsafe impl Foo for i32 {
    // method implementations
}

pub fn main() {}
