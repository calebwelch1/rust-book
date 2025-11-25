// no inheritance but polymorphism

// gui that must draw components
// and allow users to extend components

use s_OOP::{Screen, Button, Draw};

// extending in a consumer...
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}

// gui can take list of drawable components,
// of different types
// each inheriting a similar trait "draw"
// while being of different types

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe")
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 25,
                label: String::from("ok")
            })
        ],
    };

    Screen.run():
};

// static vs dynamic dispatch
// must use dyn because compiler doesn't know all concrete objects or size at compile time
// so done at runtime, runtime cost but upside is dynamic code

// State design pattern in rust ----------------------

use s_OOP::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

