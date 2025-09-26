fn main() {
    // methods are like functions fn myfunction(hi)-> u32{}
    // but in structs
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// implementation block
// everything in this block will be associated with Rectangel struct
impl Rectangle {
    fn area(&self) -> u32 {
        // self as reference to struct
        // &self because we don't want to take ownership just read the data not write to it.
        // if we want to change the data use &mut self
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

//can also name a method the same as a parameter in the struct
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

// can also just return the value in a fn as a getter which is not automatically implemented in structs

fn can_hold(rectone: &Rectangle, rectwo: &Rectangle) -> bool {
if rectone.width > rectwo.width && rectone.height > recttwo.height {
true
}
false
}

// can hold in a struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
