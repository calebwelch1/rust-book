use std::thread;

fn main() {
    // println!("Hello, world!");
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    })

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // prints 1 1 2 2 3 3 4 4 ...
    // sleeps for 1 millisecond and main thread runs, then that sleeps and spawned thread runs
    // spawned thread never finishes because main thread ends at 4
}

// wait for spawned thread to finish executing before ending program
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    })
    // if we place handle.join().unwrap here, main thread will wait for spawned thread to finish executing
    // before it runs itself

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // return value of spawn function handle.join -- also unwrap because it returns result type
    handle.join().unwrap();
}

// explicitly telling program to force variable ownership so 
// variable is not dropped before thread finishes running 
fn main() {


let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
        println!("here's the vector: {:?}", v);
    })
handle.join().unwrap();
}

// channels
// analagous to channel of water, send data downstream to other threads

use std::sync::mpsc;
// multi producer single consumer...
// multiple producers of messages but one consumer
use std::thread;

fn main() {
    mpsc::channel(); // returns tuple of sender and receiver
    // transmitter, receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    })

    let received = rx.recv().unwrap();
    // also rx.try_recv().unwrap(); will not block main thread execution
}

// ------------ sharing state

// Only one thread can lock (access) the data inside the mutex at a time.

// Other threads trying to lock it will block until the mutex becomes available.

// Rust wraps mutexes in safe abstractions that guarantee you cannot misuse them without the compiler noticing.

use std::sync::Mutex;

fn main() {
    let m: Mutex<i32> = Mutex::new(5);

    {
        // unwrap - if lock fails then panic
        // when mutexguard goes out of scope it auto releases lock
        let mut num = m.lock.unwrap();
        *num = 6;
    }
}
use std::thread;
use std::rc::Rc;
use std::synce::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    // counter will be owned by each iteration
    // must allow it to have multiple owners
    // need something like Rc but thread safe
    // -> Arc
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num +=1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}