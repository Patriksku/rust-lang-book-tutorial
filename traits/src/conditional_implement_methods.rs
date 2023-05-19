use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// This implementation block will work on any type Pair.
impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This implementation block will only be available on Pair structs where the type of 'x' and 'y' implement Display and PartialOrd.
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket implementations:
// EXTRA (used a lot in the standard rust lib)
// Here, we are implementing the ToString trait, on any type T, that implements the Display trait.
// impl<T: Display> ToString for T {
//     // --snip--
// }
