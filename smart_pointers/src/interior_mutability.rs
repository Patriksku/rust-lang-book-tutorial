// Interior Mutability
// is a design pattern in Rust that allows you to mutate data, even when there are immutable references
// to that data, which is typically disallowed by the borrowing rules. To mutate data, this pattern uses
// unsafe code inside a data structure, to bypass the typical rules around mutation and borrowing.
//
// Unsafe code is code that is not checked at compile time for memory safety (will learn more in chapter 19).
//
// Even though the borrowing rules are not enforced at compile time, we can still enforce them at run-time.
// 
// The RefCell smart pointer represents single ownership over the data it holds, much like the Box smart pointer.
// The difference is that the Box smart pointer enforces the borrowing rules at compile time, while the RefCell
// smart pointer enforces them at runtime.
// This means, that if you break the borrowing rules at runtime, your program will panic and exit.
//
// RefCell is useful when you are sure that the code is following the borrowing rules, but the compiler cannot understand
// or guarantee that.
//
// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
// If you want the value inside a Box<> smart pointer to be mutable, then the Box<> itself has to be mutable.
//
// Mutating a value inside an immutable value, is called the Interior Mutability pattern. And RefCell<> allow us to do this.
//
// 
// IN SHORT: the Interior Mutability pattern gives us flexibility, but we have to be careful and make sure that our code
// abides by the borrowing rules! Also, because we are checking the borrowing rules at runtime,
// it does give us a small performance hit.
//
// 
// NOTE: RefCell is only useful for single-threaded programs.

pub fn main() {
// Not allowed at compile time.
    // let a = 5;
    // let b = &mut a;

    // let mut c = 10;
    // let d = &c;
    // *d = 20;
    
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

// We have to use a lifetime annotation because we are borrowing T.
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker { 
                messenger, 
                value: 0, 
                max, }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger.send("Urgent warning: You've used up over 90 % of your quota.");
            } else if percentage_of_max >= 0.75 {
                self.messenger.send("Warning: You've used up over 75 % of your quota.");
            }
        }
    }

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]), }
        }
    }

    impl Messenger for MockMessenger {

        // The send() signature of the Messenger trait has an immutable &self reference, so we cannot push to the vector.
        // To solve this, we can wrap the necessary fields in the RefCell smart pointer.
        // Then, we can borrow the underlying immutable value, in this case the value of 'sent_messages',
        // as a mutable reference!
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // To get an immutable reference to the underlying immutable value, in this case the value of 'sent_messages',
        // we can call .borrow(). Notice that we use borrow_mut() above, which gives us a mutable reference.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


