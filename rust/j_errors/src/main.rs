//Option enum is how we use null values since rust has no null value
// Result enum is how we try/catch instead in rust it's Ok/Err

fn main() {
    // immediately quit program and print out error
    panic!("crash and burn");

    // backtrace lists all functions that lead up to erroring code
}
// backtrace example
// run
// RUST_BACKTRACE=1 cargo run
fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}
// recoverable errors
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;
use std::io::ErrorKind;
  fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file)=> file,
        // Err(error)=> panic!("Problem opening file: {:?}", error);
        // ErrorKind for specific errors
        Err(error)=> match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            }
            other_error => {
                panic!("Problem creating the file {:?}", other_error)
            }
        }
    }
  }
  // rewrite the above code with closures which makes it easier to read

   let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
   })

   use std::fs::File;
   use std::io;
   use std::io::Read;
// error propogation
   fn read_username_from_file() -> Result<String, io::Error> {
    let mut f: Result<File, Error> = File::open("hello.txt")?;
    // the ? after file open is shorthand for below
    // unwrap or expect method, if file opens f is returned and stored in f
    // if file fails to open function ends early and returns error

    // check type of f
    // let mut f = match f {
    //     Ok(file)=>file,
    //     Err(e)=>return Err(e),
    // };

    // make new string
    let mut s = String::new();
    // read contents of file and store it in string

    // match f.read_to_string(&mut s) {
    //     Ok(_) = Ok(s),
    //     Err(e)=> Err(e),
    // }

    // same here
    f.read_to_string(&mut s)?;
    Ok(s)
   }

   // even shorter function
// allows main to use ? operator
// -> Return a result that is either a unit or any error
 fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
 }

 // you can create a struct for validation
 // instead of using if /else block to test if guess is between 1 and 100
 // can implement a struct that panics ensuring guess will always be
 // between 1 and 100 for all other functions that may use guess

 pub struct Guess {
    value: i32,
 }

 impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
    // like have a private set and public get in C#
    // this allows obtaining value without being able to change it
    pub fn value(&self) -> i32 {
        self.value
    }
 }