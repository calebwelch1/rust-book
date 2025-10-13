fn main() {
    println!("Hello, world!");
}
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// traits are a set of methods that are shared among different types

// basically abstract methods from c#
pub trait Summary {
    fn summarize(&self) -> String;
}

let article = NewsArticle {
    author: String::from("John Doe"),
    headline: String::from("The Sky is Fallling"),
    content: String::from("The sky is not actually falling")
}
pub fn notify(item: &impl Summary) {
    println!("Breaking News!, {}", item.summarize());
}
// same function
// Type must be something that implements Summary trait
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}

println!("Tweet Summary: {}", tweet.summarize());
println!("Article Summary: {}", article.summarize());
