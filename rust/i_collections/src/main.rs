fn main() {
}
// collections stored on heap so they are dynamic size
// vector store a variable number of values next to eachother
// string collection of characters
// hash map, associate a value with a key

let v: Vec<i32> = Vec::new();

//vec! holds values you give it

let v = vec![1, 2, 3];

//pushing
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}

// this won't work, adding new element means vec may need to be copied into new space in memory
// so after push first may not be in same position
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");

// get immutable references to each element and print them
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}

// altering mutable references to vector
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

// types in enum are considered enum this is how we can create a vector that holds different types
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
// Many of the same operations available with Vec<T> are available with String as well because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities. An example of a function that works the same way with Vec<T> and String is the new function to create an instance, shown in Listing 8-11.
let mut s = String::new();

let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();

//String::from and to_string do the same thing, so which one you choose is a matter of style and readability.
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שלום");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");

// grow a string with push_str
let mut s = String::from("foo");
s.push_str("bar");

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
//same
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
// =
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
// cannot index string s1 = s[0];

//hashmap
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
// access values in hashmap
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
// sets to 0 if no entry for that key
let score = scores.get(&team_name).copied().unwrap_or(0);
