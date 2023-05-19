// Structs and enums are the building blocks for creating new types in Rust.

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("Hejo@gmail.com"),
        username: String::from("BogdanovOLOV"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("wallace123");

    let user2 = build_user(String::from("wagwan@mail.com"), String::from("wagwaan"));

    let user3 = User {
        email: String::from("bestbest@mail.com"),
        username: String::from("besmylah"),
        // Rest of params same as user2
        ..user2
    };

    struct_example()
}

fn build_user(email: String, username: String) -> User {
    // 'Field init shorthand syntax' for first two params.
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementations for structs are called Methods (as opposed to functions).
// We can have several implementation blocks.
// Methods are TIED to an instace of our struct.
// First argument in a method is always &self --> which is the instance the method is being called on.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Associated function (as opposed to Methods)
// We have no reference to self in these.
// Associated functions are NOT TIED to an instance of our struct.
impl Rectangle {
    /// Returns a rectangle of size = size*size.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn struct_example() {
    let rect = Rectangle {
        width: 50,
        height: 100,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 120,
        height: 580,
    };

    // Call to associated function.
    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("Our rectangle: {:#?}", rect);
    println!("The area of the rectangle is: {}", rect.area());
}
