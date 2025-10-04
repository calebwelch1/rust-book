fn main() {
    println!("Hello, world!");
    let num_list = vec![34, 50, 25, 100, 65];
    let largest = largest_number(&num_list);
}
// generics, types, lifetimes to reduce code replication
// Vec<i32>
fn largest_number(number_list: &[i32]) -> i32 {
    let mut largest: i32 = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// if we want to find the largest character in a vector...
let char_list = vec!['y', 'm', 't', 'l'];

//rewrite using generics
// T: Type that can be ordered (i.e sorted/ greater than/ less than) + Copied
fn get_largest<T: PartialOrd + Copy>(list: Vec<T> ) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// multiple generics
fn multiple_generics<T, U, V>()->{}

struct Point {
    x: i32,
    y: i32,
}

// generic Point type

struct Point<T> {
    x: T,
    y: T,
}

struct PointThree<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1 = Point {x: 5, y: 10};
    // what if we want floats?
    let p2 = Point {x: 1.3, y: 8.4};
    // what if we want different types in ONE Point?
    let p3 = Point {x: 1, y: 8.4};
}

Struct Pnt<T> {
    x: T,
    y: T,
}

impl<U> Pnt<U> {
    fn x(&self) -> U {
        &self.x
    }
}