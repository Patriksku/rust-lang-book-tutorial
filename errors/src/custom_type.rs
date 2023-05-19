// Not that the field in Guess is private, because it is not declared pub.
// Therefore, we can not create Guess directly. We have to use the new associated function below.
// And if we want to get the value, we have to use value() method.
// This is because we do not want anyone to be able to manipulate the value field.

// This way, we can be safe that the value in Guess will always be correct.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
