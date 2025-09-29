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

// make vec and initialize with values
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

// safer way to access indexes, that wont crash! 
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
    // dereference with * and get value
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
// how you find out which type you are accessing in the vector
match &row[1] {
    SpreadsheetCell::Int(i) => println!("{}", i),
    _=> println!("Not an integer!")
};

// strings are stored as a collection of UTF-8 encoded bytes
// ASCII american standard for code interchange
// 7/8 bits of the ascii byte contain information so it only encodes english characters

// unicode is universal character set to solve different encodings for different languages

//utf-8 variable encoding 1-2-4 bytes any byte length

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

let hello = String::From("Hello, ");
let world = String::From("world!");
let hello_world = format!("{}{}",hello,world);
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

// because utf-8 has chars of 1-4 bytes. string[0] could only give us half a character in a different language
//bytes, scalar (full or part of char) CHAR type, graphene cluster - what we consider characters in english
// so we have to tell rust which one we want

for b in "नमस्ते".bytes() {
    println!("{}", b);
}

for c in "नमस्ते".chars() {
    println!("{}", c);
}
// to iterate over graphene clusters need to include
// [dependencies]
// unicode-segmentation = "1.7.1"

for g in "नमस्ते".graphemes(true){
    println!("{}", g);
}

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
// for (key, value) in &mapName
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

let mut pokemonLevels = HashMap::new();

pokemonLevels.insert(String::from("Bulbasaur"), 5);
// this overwrites the previous value of same key
pokemonLevels.insert(String::from("Bulbasaur"), 7);
// this checks if key is there if not it will default value, if it is there it does nothing
pokemonLevels.entry(String::from("Charmander")).or_insert(10);

let mut pokemonList = vec!["Charmander", "Squirtle", "Bulbasaur"];

let grassStarter: Option<String> = pokemonList.get(2);

match grassStarter {
    Some(grassStarter) => println!("Grass starter is {grassStarter}"),
    None => println!("No grass starter"),
}

// updating map based on older values

let text = "hello world wonderful world";
let mut map = HashMap::new();
// .split_whitespace splits words by space
for word in text.split_whitespace() {
    // or_insert returns mutable reference
    let count = map.entry(word).or_insert(0);
    // dereference and add
    *count +=1;
}

fn main() {
    for i in 1..=100 {
        match (i % 3, i % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", i),
        }
    }
}
 
fn anagrams(a_string: String, b_string: String) -> bool {

}

fn all_new_string(a_string: String) -> bool {
    use std::collections::HashMap;

    let mut dictionary = HashMap::new();

    for c in a_string.chars() {
        let count = dictionary.entry(c).or_insert(0);
        *count += 1

        if *count > 1 {
            return false;
        }
    }

    true
}

use std::collections::HashSet;

fn all_new_string(a_string: String) -> bool {
    let mut seen = HashSet::new();

    for c in a_string.chars() {
        if !seen.insert(c) {
            return false; // already exists
        }
    }

    true
}

use std::collections::HashMap;

fn are_anagrams(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut count1 = HashMap::new();
    let mut count2 = HashMap::new();

    for c in s1.chars() {
        *count1.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        *count2.entry(c).or_insert(0) += 1;
    }

    count1 == count2
}
