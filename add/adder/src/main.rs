// '_' instead of '-'.
// To run this 'adder' package, we simply type: 'cargo run -p adder'
use add_one;
use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
