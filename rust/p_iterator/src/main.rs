// iterate over sequence of elements no matter how they are stored
// array, graph, hashmap whatever

// iterator trait
pub trait Iterator {
    type Item;

    // will go next until there is no next so it returns None
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let v1 = vec![1, 2, 3];

    // iterates over vector
    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("got: {}", value);
    }
}
// adaptors take in an iterator and return an iterator
// consumers take in an interator and return another type

// consumer
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

//adaptor
fn adaptor_map() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // map is like js map
    // it takes a closure that it calls over each item in sequence
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

//
struct Shoe {
    size: u32,
    style: String,
}

fn filter_example(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.inter_iter().filter(|s| s.size == shoe_size).collect()
}

// custom iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
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

fn call_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter_mut();
    //.iter() for iterator
    //.iter_mut() for mutable iterator
    //.into_iter() for own types

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}