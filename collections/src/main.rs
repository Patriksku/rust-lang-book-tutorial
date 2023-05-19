// Vectors grow in size (stored in heap).
// Therefore accessing e.g. out of bounds elements, result in runtime errors, instead of compile time errors.
// --> Because we do not know the size of a vector at compile time, they are stored on the heap so the could be of variable size.

// When we access elements in vectors, we are getting a reference to that element.
// Pushing an element might result in the vector growing, making all the elements jump one step.
// --> Therefore, we can not have a reference to an element, while simultaneously adding stuff to the vector.
mod hashmap;
mod string;
fn main() {
    // Array.
    let a = [1, 2, 3];

    // Initialize empty vector.
    let mut vEmpty: Vec<i32> = Vec::new();
    vEmpty.push(1);
    vEmpty.push(2);
    vEmpty.push(3);

    // Initialize vector with given values.
    let mut v = vec![1, 2, 3, 4, 5];

    // Accessing vector elements.
    // REFERENCE RULES! https://youtu.be/Zs-pS-egQSs?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&t=430
    let third = &v[2];
    // v.push(6);
    println!("The third element is {}", third);
    println!("The vector looks like so {:#?}", v);

    // GET METHOD OF ACCESSING VECTOR ELEMENTS
    // As opposed to above way of accessing, this method will not result in a runtime error if we try to access a non-existing element.
    match v.get(20) {
        Some(index) => println!("The third element is {}", index),
        None => println!("There is no third element."),
    }

    looping();
    enum_in_vector();
    string::main();
    hashmap::main();
}

fn looping() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }

    println!("{:#?}", v);

    // for i in &v {
    //     println!("i -> {}", i);
    // }
}

fn enum_in_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Float(0.24),
        SpreadsheetCell::Text(String::from("HiHello")),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer!"),
    }
}
