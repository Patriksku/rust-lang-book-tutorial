// Drop trait can be implemented on any type, and it allows you to customize what happens
// when a value goes out of scope.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping '{}'.", self.data);
    }
}

// Drops the out of scope data in a stack like fashion.
pub fn main() {
    let c = CustomSmartPointer {
        data: "my stuff".to_string(),
    };

    let d = CustomSmartPointer {
        data: "other stuff".to_string(),
    };

    // If we want to free up memory earlier than when that data gets out of scope, for instance
    // when we are working with locks, we can it like so:
    drop(c);

    println!("CustomSmartPointers created.");
}