// If we would have only T, then both x and y would have to be of the same type.
// By adding <T, U> and replacing y: U, then they can either be the same or not.
struct Point<T> {
    x: T,
    y: T,
}

// the impl functions you will be able to call on your Point struct, will be based on the <> params!

// Available for Points of any type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Available for Points of type f64 (both x and y need to be f64, as they have the same 'T' char)
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

pub fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    // let p3 = Point { x: 5, y: 10.0 };

    // A more advanced example.
    let pb1 = PointB { x: 5, y: 10.4 };
    let pb2 = PointB { x: "Hello", y: 'c' };
    let pb3 = pb1.mixup(pb2);

    println!("pb3.x = {}, pb3.y = {}", pb3.x, pb3.y)
}

struct PointB<T, U> {
    x: T,
    y: U,
}

// A bit tricky example..
impl<T, U> PointB<T, U> {
    fn mixup<V, W>(self, other: PointB<V, W>) -> PointB<T, W> {
        PointB {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn enum_generic() {
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
