// Iterators allow us to iterate over any data structure in a unified manner.
// The iterator trait has various methods with default implementations.
// Two broad categories of methods:
// - Adapters, which take in an iterator and return another iterator,
// - Consumers, which take in an iterator, and return some other type (e.g. collection, integer, etc).

mod main2;
pub trait Iterator {
    // This is called an associated type.
    type Item;

    // This is the only method we are REQUIRED to implement, next().
    // All other methods have default implementations.
    // We also need '&mut self' as the iterators internal state is changed as we use it.
    fn next(&mut self) -> Option<Self::Item>;
    
}

#[test]
fn iterator_demonstration() {
    let vec = [1, 2, 3];

    let mut v1_iter = vec.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let vec = [1, 2, 3];
    let mut v1_iter = vec.iter();
    
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}


fn main() {
    let v1 = [1, 2, 3];

    // Iterators are 'lazy' in Rust, so nothing really happens UNTIL we actually use the iterator.
    // They also return immutable references. If we want mut instead, we use .iter_mut();
    // If we want own types, we would use .into_iter();
    let iter = v1.iter();

    // All the logic is incapsulated in the iterator, so we don't have to explicitly specify how
    // to iterate by writing extra code.
    for value in iter {
        println!("Got: {}", value);
    }

    adapter();
    main2::main();
}

// Adapter example
fn adapter() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
