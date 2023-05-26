pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // A vector of trait objects. We define a trait object by first specifying
    // some sort of pointer, such as a reference or Box sp, then using the 'dyn'
    // keyword, followed by the relevant trait. dyn = Dynamic Dispatch.
    // Box will contain any type that implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/*
Same thing bellow but with Generics. As we can see, we are more restricted with the Generics implementation.
We could only use one type of components inside out vector, as opposed to above Trait Object implementation.
Below implementation is preferred if we have a homogeneous collection, because there is no Runtime cost.
 */
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // draw button
    }
}
