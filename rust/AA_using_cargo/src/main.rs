fn main() {
    println!("Hello, world!");
}

//to make cargo new filename

// if in existing repo with no cargo file run cargo init

// cargo build -> target/debug/filename.exe

// also can use -> cargo run

// cargo check -> check your files compile but doesn't make executable

// git clone some project
// cd some project
// cargo build

// ownership rules

// 1. Each value in Rust has a variable that's called it's owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped
