fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');
    // expressions

    let y = {
        let x = 3;
        x + 1
    };
    println!("y is {y}");
    let t = five();
    println!("value of t is {t}");
}

fn another_function(x: i32) {
    println!("Another function! {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurements is: {value}{unit_label}");
}

// return functions
fn five() -> i32 {
    5
}

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
// remember if its returned no semicolon!
//     x + 1
// }


// if

fn main() {
    let number = 3;

    // in js we can go if (number) { return this }
    // in rust we need a boolean condition

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    //LOOPS

    // this loop will run forever
    loop {
        println!("again!");
    }
 
}

// this is how we can break from loops
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // this will break the inner loop
                break;
            }
            if count == 2 {
                // because this is tagging the outer loop, it will break the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// while loops
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// but using a for loop is more efficient!
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// here is a way to countdown a number
// .rev() reverses the range we create

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// ownership

// All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

// The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

// ownership rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.