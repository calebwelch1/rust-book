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


// deref trait *

use std::ops::Deref
// based on Box
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) - > MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // when deref operator is used... return first element in our tuple struct
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    // let y = &x;
    // Box
    // let y = Box::new(x);

    // must add ability to dereference in our MyBox struct
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // *y == y.deref()

    // this breaks because we are trying to assert equality
    // between an int and reference to an int
    // assert_eq!(5, y);

    assert_eq!(5, y);
}

// deref coercion converts reference of one type to a reference of another type


// drop trait
// drop trait helps us clean up resources when smart pointer goes out of scope

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::From("My stuff"),
    }
    let d = CustomSmartPointer {
        data: String::From("other stuff"),
    }
}

// you can customize which smartpointers are dropped first
// perhaps you want to do this for a lock

// to manually clean up a value early call drop function and pass value
// not c.drop() use drop(c)