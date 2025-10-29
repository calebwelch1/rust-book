// smart pointers are data structures that act like a pointer, and also keep additional meta data
// both String's and vectors are smart pointers

// box use cases
// a type whos exact size cannot be known at compile time but want to use value of that type in context that involves knowing exact size
// large amount of data with ownership transfer while not copying data
// only care value imiplements specific trait rather than being type - trait object
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Cons list is like linked list, one cell with value, second cell with pointer to next tuple
// This data type is going to have infinite size because it recursively includes itself
// Box<List> solves this, the Box smart pointer is a fixed sized pointer
// that points to arbitrary amount of data on heap but on stack it is fixed
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Nil)))));
}