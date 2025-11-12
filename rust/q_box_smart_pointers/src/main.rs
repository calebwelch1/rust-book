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

// --- Reference Counting

// sometimes a single value has multiple owners
// like a graph with multiple edges that point to the same node
// we need to remove everything pointing to that node, before it can be cleaned up

// reference counting keeps track of all pointers to a value then cleans it up when no more pointers are referencing it

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// interior mutability

// enforces borrowing rules at runtime
// box will catch borrowing rules at compile time
// advantage of runtime is some memorysafe attributes must be allowed
// because they cannot be caught at compile time
// The halting problem
std::cell::RefCell
// because refcell allows mutable borrows at runtime, you can mutate the value instide the refcell
// even when the refcell is immutable

fn main() {
    let a = 5;
    // cannot borrow a as mutable because it is not declarad as mutable
    let b = &mut a;


    let mut c = 10;
    let d = &c;
    // although c is mutable, d is immutable reference so cannot change it
    *d = 20;

}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T:Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a', T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max:usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
}

use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    data.borrow_mut().push(4);  // ✅ allowed
    println!("{:?}", data.borrow());  // prints [1, 2, 3, 4]

    // Violating borrow rules at runtime:
    let borrow1 = data.borrow();
    let borrow2 = data.borrow_mut();  // ❌ panic: already borrowed immutably
}

// reference counting...
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("Hello"));
    let b = Rc::clone(&a);
    println!("Count: {}", Rc::strong_count(&a)); // 2
    // when count reaches 0 memory is freed
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node { value: 1, next: None }));
    let b = Rc::new(RefCell::new(Node { value: 2, next: Some(Rc::clone(&a)) }));

    // Mutate through Rc -> RefCell
    a.borrow_mut().next = Some(Rc::clone(&b));
    println!("{:?}", a.borrow().next.as_ref().unwrap().borrow().value); // 2
}
// allows linked structure with shared mutable access


// reference cycle, create items that reference eachother in a cycle which creates a memory leakl


//ARC allows shared mutability across threads
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // 5
}

// Nodes

use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    parent:RefCell<Rc<Node>>,
    // vector of nodes wrapped in Rc smart pointer to allow variables outside tree to point to node
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

     let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // branch -> leaf through .children

    // how to let leaf know about parent?
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}

// These two are often combined as Rc<RefCell<T>>:

// Rc gives shared ownership (many owners)

// RefCell gives interior mutability (runtime-checked mutation)

// Together, they let you have shared, mutable state — for example, in tree or graph structures.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let first = Rc::new(RefCell::new(Node { value: 1, next: None }));
    let second = Rc::new(RefCell::new(Node { value: 2, next: None }));

    // Link first -> second
    first.borrow_mut().next = Some(Rc::clone(&second));

    // Mutate through shared reference
    second.borrow_mut().value += 10;

    println!("first: {:?}", first);
    println!("second: {:?}", second);
}
