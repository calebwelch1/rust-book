fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

// dangling reference is a reference that points to invalid data
// using lifetimes we can say we can make sure variables we are using
// are in scope and not dropped before use

// convention is to begin with a to name lifetime then alphabetical

// &i32 - a reference
// &'a i32 - a reference with an explicit lifetime
// &'a mut i32 - a mutable reference with an explicit lifetime

// creates relationship to lifetime of multiple references...
// i.e returned reference will have the lifetime of smallest referenced var
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// structs with lifetime

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> {
        println!("Attention please {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::From("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find");
    let i = ImportantExcerpt {
        part: first_sentence,
    }
}

