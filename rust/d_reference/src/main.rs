fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// &s1 allows us to refer to the var without taking ownership

// this is borrowing, you cannot mutate a borrowed variable

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// however you can mutate a mutable reference
// if you have a mutable reference to a value
// there can be no other references to that value

// however with curly braces there is a new scope so you can
// make another mutable reference there
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;

// cannot have mutable and immutable reference to same value

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");

// slice in a string if spaces return contiguous chars as a word

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // to go over string by element must be converted to array of bytes

    for (i, &item) in bytes.iter().enumerate() { //iterate...(index, reference)
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

// solution...
// string slice reference to part of string
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// to start at 0 you can drop 0
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

// or if you want to include last byte of string you can drop ending num
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

// by dropping both you get entire string
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];

// function works now with slices
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // c# equivalent is char[] chars = s.ToCharArray();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.
let s = "Hello, world!";

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

// also works with arrays
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
