
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        // iterate through components and for reach call draw
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self)
    {
        // draw button
    }
}


// blog----------------

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
// new will create a post with the state set to draft
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // return contents of post if its in a published state
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // take() = takes state and leaves none
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

        pub fn approve(&mut self) {
        // take() = takes state and leaves none
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // should have no effect until pending review
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
       fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

struct Published {
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // should have no effect already published
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}