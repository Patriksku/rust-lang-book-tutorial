/*
Generic type parameters, could specify a default, concrete type.
This allows implementors to not have to specify a concrete type, unless it is different than the default.

A great use-case for this is when customizing the behavior of an operator, a.k.a. operator overloading.
Rust allows you to customize the semantics of certain operators, that have associated traits,
in the standard library ops module.

One of the operators you can overload, is the Add operator.
E.g, we can overload the Add operator for our Point struct, by implementing the Add trait for Point.


In general, you can use Generic type parameters for two reasons:
    1. To extend a type without breaking existing code.
    2. To allow customization for specific cases, which most users will not need.

An example of the 2nd reason, is the Add-operator overloading for Millimeters below.
    Most of the time, we want to add the same types, but in certain situations, we want to be able to add different types.

Reason number 1 is the opposite. If you already have a type, and you want to add a generic parameter, but you do not want to
break existing code, then you can have a default concrete type.
*/

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// This is the implementation of the Add trait.
// There is a generic called Rhs (right-hand side).
// This is the type that is passed to the add() method.
//
// Rhs have a default concrete type which is going to be self, or the type that is going to be implementing the Add trait.
// This makes sense because typically, when we want to add two things together, for e.g. a Point with another Point,
// then the return type is going to be Point as well.
//
// Because Rhs has a default concrete type (self), we did not need to specify a concrete type, when we implemented Add for Point,
// because we were returning a Point.
//
// ->
// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

pub fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
