fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // dot notation to access
    user1.email = String::from("anotheremail@example.com");

    // you can also spread data from a similar struct
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // but this means user1 can no longer be used it has been moved to user2
    // if only active and sign_in_count had been copied we could still use user1
    // because those vals implement copy over trait
}

// we can use a function to build a struct

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// shorthand so you don't have to repeat vars used in the struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

let user2 = build_user(
    String::from("kyle@mail.com"),
    String::from("kyle1")
);

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// structs are new unique types, even if similar cannot be used for one another
// funciton taking Point will not accept Color

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
// To define AlwaysEqual, we use the struct keyword, the name we want, and then a semicolon. No need for curly brackets or parentheses! Then we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses. Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior! You’ll see in Chapter 10 how to define traits and implement them on any type, including unit-like structs.

// rectancle function using tuples
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
// with structs

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// you can print the rect struct!
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}

// using dbg we can return the struct and the value of a certain param
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle
{
    // reference self first, what is being implemented
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle
{
    fn square(size: u32) -> Rectangle
    {
        Rectangle {
            width: size,
            height: size
        }
    }
}

// call with rect.area();
// call with rect.can_hold(&rect1)
// call with let rect3 = Rectangle::square(25);
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

//enums
enum IpAddrKind {
    V4(u8,u,u8,u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
// benefit of enum is all of these will be one type
enum Message {
    Quit,
    Move{x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_function() {
        println!("Let's get rusty!");
    }
}
fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::fromt("127.0.0.1");
    // }
    let localhost = IpAddrKind::V4(127, 0, 0, 1);

}
// can take either type
fn route(ip_kind: IpAddrKind)
{

}

// no null value only option enum

fn main() {
    // forces us to handle None case incase it doesn't exist
    // actually here by default
    enum Option<T>{
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("string");
    let absent_number: Option<i32> = None;

    // int
    let x: i8 = 5;
    // int or none
    let y: Option<i8> = Some(5);

    // unwrap - if value we use this or if no value we use this
    let sum = x + y.unwrap_or(0);
}

enum UsState {
    Alabama,
    Arkansas,
    Arizona,
    California,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) => u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// matcha and option
let six = plus_one(five);

fn plus_one(x: Option:<i32>) -> Option<i32> {
    match x {
        None => None,
        // can't just return i, because it is some or none
        // must be wrapped in some
        Some(i: i32) => Some(i + 1),
    }
}
// wildCard matches everything else

fn plus_two(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i: 32) => Some(i + 2),
        _ => (),
    }
}

// only specify pattern you care about with if let

if let Some(3) = some_value {
    println!("Three!");
}