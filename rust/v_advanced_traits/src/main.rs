fn main() {
    println!("Hello, world!");
}
// associated types

pub trait Iterator<T> {
    type Item;
    fn next(&mut self) -> Option<T>;
}

// since we don't know type of item until iterator trait is implemented
// takes type of iterator

// both associated types and generics allow us to define type without concrete
// generic only 1 per implementation, associated multiple

struct Counter {}

impl Iterator<u32> for Counter {

    fn next(&mut self) -> Option<u32>{
        Some(0)
    }
}

impl Iterator<u16> for Counter {

    fn next(&mut self) -> Option<u16>{
        Some(0)
    }
}

// generic type parameters and operator overloading

use std::ops::Add;
// ops module has overloads

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point{
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// calling methods with the same name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        // implementation
    }
}

impl Pilot for Human {
    fn fly(&self) {
        // implementation
    }
}

impl Wizard for Human {
    fn fly(&self) {
        // implementation
    }
}

fn main() {
    let person = Human;
    person.fly()
    // implements human fly to call others...
    Pilot::fly(&person);
    // or
    <Human as Wizard>::fly();
}

// super traits

// traits depend on eachother
// anything that implements OutlinePrint should also implement Display trait

use std::fmt;

trait OutlinePrint: fmt::Display{
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        // etc...
    }
}