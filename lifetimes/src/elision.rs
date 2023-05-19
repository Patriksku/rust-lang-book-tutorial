// In some cases, the compiler can deterministically infer the lifetime annotations.
// The compiler does this by checking the 3 ELISION rules:

// 1. Each parameter that is a reference gets its own lifetime parameter.

// 2. If there is exactly one input lifetime parameter, that lifetime is
//    assigned to all output lifetime parameters.

// 3. If there are multiple input lifetime parameters, but one of the is
//    &self or &mut self, the lifetime of self is assigned to all output
//    lifetime parameters. --> (this rule only applies to methods).

// Lifetimes of arguments being passed in, are called 'input lifetimes',
// and lifetimes of the returned values are called 'output lifetimes'.
// Here, the compiler infers the lifetime annotations, so in reality, the function declaration looks like this
//     (based on the first 2 rules):
//         fn first_word<'a>(s: &'a str) -> &'a str { ...
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn main() {}
