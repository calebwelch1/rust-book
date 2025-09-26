// rand from dependencies
use rand::Rng;
// or use nested paths
use rand::{Rng, CryptoRnd, ErrorKind::Transient};

use std::io;
use std::io::Write;
// or 
use std::io::{self, Write};

let secret_number = rand::thread_rng().gen_range(1, 101);

// pulling from front_of_house.rs
mod front_of_house;
// mod children exist in folder of same name
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}