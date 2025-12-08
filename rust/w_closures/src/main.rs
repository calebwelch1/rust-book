// can pass a function to another function
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice( f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
//  function or closure
fn do_twice<T>( f: T, arg: i32) -> i32 
where T : Fn(i32) -> i32{
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("Answer is: {}", answer);
}

// closure traits
// fn, fnmut, fnOnce
// immutable, mutable, takes ownership of values
// function pointer does all three


fn main() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    println!("{:?}", list_of_strings);

    // at .map can also pass a function pointer

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
    println!("{:?}", list_of_strings);
}

// return closure from function

fn returns_closure(a:i32 ) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a+b)
    } else {
        move |b| a - b
    }
}
