// anonymous functions, can be passed as params to other functions
// no need to type inputs or returns
// the first type passed into the closure will be the concrete type of the closure

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
// must use fn trait and fn, fnmut, fnOnce
// must use Generic for closure
struct Cacher<T>
where
T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
whereT: Fn(u32) ->u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher{
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                // caching value
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // anonymous function input params are inside |param|
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // }

    let mut cached_result = Cacher::new(|num|) {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    }

    if intensity < 25 {
        // instead of calling expensive closure here again and again
        // use cached value - memoization
        // println!("today, do {} pushups"), expensive_closure(intensity);
        // println!("today, do {} situps"), expensive_closure(intensity);
        println!("today, do {} pushups"), cached_result.value(intensity);
        println!("today, do {} situps"), cached_result.value(intensity);
    }
    else if intensity == 3 {
        println!("take a day off");
    } else {
        // println!("run for {} minutes", expensive_closure(intensity));
        println!("run for {} minutes", cached_result.value(intensity));

    }
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

// closure has access to variables defined outside of function

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;
    // move sends ownership to closure
    let equal_to_x = move |z| z == x;


    let y = 4;

    // this passes
    assert!(equal_to_x(y));
}