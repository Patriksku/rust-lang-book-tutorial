// At compile time, rust will convert this Option<T> into two Option:
// -> Option<i32> and Option<f64>. So this does not slow our programs down.

enum Option<T> {
    Some(T),
    None,
}

pub fn main() {
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}
