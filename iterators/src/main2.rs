#[derive(Debug, PartialEq)]
pub struct Shoe {
    size: u32,
    style: String,
}

// The closure has access to the variable in its environment.
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: "sneaker".to_string(),
            },
            Shoe {
                size: 13,
                style: "sandal".to_string(),
            },
            Shoe {
                size: 10,
                style: "boot".to_string(),
            },
        ];
        
        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: "sneaker".to_string(),
                },
                Shoe {
                    size: 10,
                    style: "boot".to_string(),
                },
            ]
        );
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter { 
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// So zip operation will return [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]. 
// Then multiply each pair with map and we got [0, 2, 6, 12, 20]. 
// Filter those who can divide by 3: [6, 12]. 
// And it's sum is 18.
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)|a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
// So, by implementing the next method, we get access to a lot of other methods, like shown above!

pub fn main() {
}







