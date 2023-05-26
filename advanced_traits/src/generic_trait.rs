pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

pub fn main() {}

struct Counter {}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}
