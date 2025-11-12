fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

// dangling reference is a reference that points to invalid data
// using lifetimes we can say we can make sure variables we are using
// are in scope and not dropped before use

// convention is to begin with a to name lifetime then alphabetical

// &i32 - a reference
// &'a i32 - a reference with an explicit lifetime
// &'a mut i32 - a mutable reference with an explicit lifetime

// creates relationship to lifetime of multiple references...
// i.e returned reference will have the lifetime of smallest referenced var
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// structs with lifetime

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> {
        println!("Attention please {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::From("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find");
    let i = ImportantExcerpt {
        part: first_sentence,
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
