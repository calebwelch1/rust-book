use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input a guess");

    let mut guess = String::new();

    // variables are immutable by default
    // mut lets them become mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // & is reference, references are also immutable
    // so instead of &guess we use &mut guess to alter var from reference
    

    // convert guess to same type as secret_number
    let guess: u32 = guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    // only numbers plz
    // .expect("Please type a number.");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("you win!");
            break;
        }
    }

    println!("secret number is: {}", secret_number);

}
