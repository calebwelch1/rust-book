fn main() {
    println!("Hello, world!");
}
// pattern describes shape of data we are using, matches against it
// to make sure we are using correct data

enum Language {
    English,
    Spanish,
    Russian,
    Japanese
}

let language = Language::English;
// match on enum must be exhaustive
match language {
    Language::English => println!("hello world!"),
    Language::Spanish => println!("hola mundo!"),
    Language::Russian => println!("russian!"),
    // to bind to variable we match on us var
    // lang => println!("unsupported language! {:?}, lang")
    _ => println!("unsupported language")
}

let authorization_status: Option<&str> = None;
let is_admin = false;
let group_id: Result<u8, _> = "34".parse();

if let Some(status) = authorization_status {
    println!("Authorization status: {}", status);
} else if is_admin {
    // auth status admin
} else if let Ok(group_id) = group_id {
    if group_id > 30 {
        // authorization privleged
    } else {
        // authorization basic
    }
} else {
    // auth guest
}

// while let will run loop as long as it matches

let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top)
}

// for loops

let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}

// pattern match used in let statements as well
let x = 5;
let (x, y, z) = (1, 2, 3);

// irrefutable patterns always match
let x = 5;
// refutable patterns won't always match

let x: Option<&Str> = None;
if let Some(x) = x {
    println!("{}", x)
}

// Can only use irrefutable pattern:
// function parameters
// let statements
// for loops


// ---------------------- valid pattern syntax

// literal matching
let x = 1;
match x {
    1 => println!("one"),
    2 => println!("2"),
    3 => println!("3"),
    _ => println!("anything"),
}

match x {
    Some(50) = > println!("got50"),
    Some(y) = > println!("matched, y {:?}", y),
    _ = > println!("default case, x= {:?}", x),
}

// match on multiple patterns

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("other")
}

// match range

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else")
}

// pattern that destructures values

struct Point {
    x: i32,
    y: i32,
}
let = Point {x: 0, y: 7};
let Point {x: a, y: b} = p;
assert_eq!(0, a);
assert_eq!(7, b);

// match enum

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

match msg {
    Message::Quit => {},
    Message::Move {x, y} => {},
    Message::Write(text) => {},
    Message::ChangeColor(r, g, b) => {}
}

// to match but ignore a value...
fn foo(_: i32, y:i32)
{
    println!("this code only uses the y parameter: {}", y);
}

// prefixing a variable name with underscore binds value but lets u not use it

let _x = 5;
let y = 10;

// just match first and last in range
let numbers = (2, 4, 5, 6 ,7, 9, 10);
match numbers {
    (first, .., last) => {
        println!("first and last {}, {}", first, last);
    }
}

// match guard, additional matching on pattern

match x {
    Some(x) if x < 5 => println!("less than five: {}", x),
}

// match param of enum

enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello {id: 5};

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
    } => println!("found an id in range: {}", id_variable),
    Message::Hello {
        id: id_variable @ 10..=12,
    } => println!("found an id in range: {}", id_variable),
    Message::Hello {
        id
    } => println!("found another id {}", id_variable),
}