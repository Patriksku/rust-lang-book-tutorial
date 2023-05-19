// Previously, we always used owned Structs. But if we want to be able to use a reference to a struct, we need lifetimes.

// Here, we are saying that our Struct can not outlive the reference passed into part.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Compiler, based on Elision rule 3, infers GLA for us so we do not have to specify them manually.
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find...");

    // So if we would try to use 'i' here AFTER the end of the lifetime of first_sentence, it would not work.
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
