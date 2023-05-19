// Most OOP languages have 'Objects', 'Encapsulation', and 'Inheritance'.
//
// Objects
// Objects are made out of data, and methods that operate on that data.
// In Rust, structs and enums hold data, and you can use implementation blocks
// to provide methods on structs and enums.
//
// Encapsulation
// Implementation details of an object are hidden from the code, using that object.
// Instead of changing the internals directly, code outside of the object is limited
// to interacting with the object, through its public API. This enables the programmer
// to change the internals of an object, without changing code which uses that object.
//
// Inheritance
// The ability for an object, to inherit from another object's definition, gaining the data
// and behavior of that other object, without having to define the data and behaviour itself.
// Rust does not have this ability. Specifically, you can't define a struct that inherits
// fields and methods from another struct. Rust however, has some other tools you can use,
// depending on why you are reaching for inheritance.
// There are 2 main reasons for using inheritance:
// 1. Code sharing
// You can implement behavior on one type, and then all the other types that inherit from it,
// can re-use that behavior. In Rust, you can accomplish the same thing by using default trait
// method implementations. Note that there is a limitation. As of right now, traits can only
// define methods, not fields.
// 2. Polymorphism
// Allows you to substitute multiple objects for each other at runtime, if they share certain characteristics.
// In classical inheritance, that characteristic would be a Parent class. For e.g, you can have a base class
// called "Vehicle", and subclasses that inherit from that, such as "Truck", "Motorcycle", etc.
// Then you can define a function which takes in a Vehicle. And then at runtime, you pass in a
// "Truck", "Motorcycle", etc. to that function.
// Rust takes a different approach. In Rust, you can use generics to abstract away concrete types,
// and you can use trait-bounds to restrict the characteristics of those types. In addition to generics,
// Rust also provides trait objects, which are similar to generics, except they use dynamic dispatch,
// whereas generics use static dispatch.

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Hello, world!");
}
