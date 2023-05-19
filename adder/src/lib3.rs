pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        // We can access internal_adder because child modules are able to access everything
        // in parent modules, even private fields.
        assert_eq!(4, internal_adder(2, 2))
    }
}
