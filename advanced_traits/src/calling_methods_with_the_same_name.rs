/*
Rust allows you to have two traits with the same method, and implement both those traits on one type.
It is also possible to implement a method on the type itself, with the same name as the methods inside the traits.

If you get into this situation, then you need to tell Rust which method you want to call.

Note: Methods have a reference to self, and as such, we can pass in a reference to the type we are working with.
But with associated functions (methods without a reference to self), this is not the case. In such cases,
the syntax would be different than the code example in this file, like so:

    Human::fly();
    <Human as Pilot>::fly();
    <Human as Wizard>::fly();
 */
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!")
    }
}

pub fn main() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}
