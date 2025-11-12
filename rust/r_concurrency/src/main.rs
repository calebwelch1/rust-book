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