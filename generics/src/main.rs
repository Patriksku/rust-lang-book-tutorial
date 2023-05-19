// We can use Generics to reduce the amount of code that we have to write.
// And, they do not result in a performance hit!

mod performance;
mod struct_enum;
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['A', 'B', 'C', 'D', 'X'];

    println!("The largest number is {}", get_largest(number_list));
    println!("The largest char is {}", get_largest(char_list));

    struct_enum::main()
}

// We will learn about TRAITS in next tutorial.
// Works with ANY generic type that has the PartialOrd + Copy trait.
// E.g. strings, chars, ints...
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for element in list {
        if element > largest {
            largest = element;
        }
    }

    largest
}
