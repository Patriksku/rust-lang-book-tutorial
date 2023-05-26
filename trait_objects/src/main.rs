/*
Trait Objects
Rust does not support classical inheritance, however, it does achieve polymorphism, which
is the ability for code to work on multiple types of data, through Trait Objects.

Imagine you are building a GUI library. The goal of this library is to take in a list of
visual elements, such as buttons, text boxes, etc, and be able to draw them to the screen.
In addition, we would like users of our library to be able to extend our library. Meaning
that they could create their own visual components. This means that at compile time, we won't
know the full breadth of objects that are going to be used - however we do know that all these visual
objects are going to have a method called draw().
With classical inheritance, we would have a base class "VisualComponent" with draw(), and have all visual elements
inherit from it. That way they would all have access to draw(), and they could override it with their own implementation.

In Rust, we define shared behavior using traits.
We can view bellow as someone who is extending our gui lib.
Because we use Trait Objects, Rust will ensure at compile time, that any component in our component vector implements
the Draw trait.


Static vs. Dynamic dispatch
Monomorphization is a process where the compiler will generate non-generic implementations of functions,
based on the concrete types used, in place of generic types.
E.g. we have a function add() which takes two generic params and adds them.
We use this function with floats and integers. This results in the compiler generating a function called
'integer_add()', and another one called 'float_add()'. The compiler will then find all the call sites of the
add method, and replace it with the concrete implementation.
So essentially, we are taking a generic implementation, and substituting it with a concrete implementation.

This is called static dispatch.
Static dispatch is when the compiler knows the concrete functions you are calling at compile time.

The opposite is dynamic dispatch.
Dynamic dispatch happens when the compiler does not know the concrete functions you are calling at compile time.
So instead, it figures this out at runtime (extra cost).

When using trait objects, the compiler must use dynamic dispatch, because the compiler does not know all
the concrete objects that are going to be used at compile time. Instead, the compiler will add code to figure
out the correct method to call at runtime.
The upside is that you can write flexible code that can accept any object which implements a certain trait.


Object safety
You can only make object safe traits into trait bounds. What does it mean to be object safe?
There are 2 rules you have to follow to qualify.
A trait is object safe, when all the methods implemented on that trait, have these 2 properties:
    1. The return type is not self,
    2. There are no generic parameters.

If a trait does now have these two properties, then the Rust compiler can't figure out the concrete type of that trait,
and therefore does not know the correct method to call.


*/

use trait_objects::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("ok"),
            }),
        ],
    };

    screen.run();
}
