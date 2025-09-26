fn main() {
    // mut to allow mutability
    let mut x = 3;

    // constants are always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //shadowing 1st var is shadowed by 2nd var with same name
    // i.e compiler only cares about last instance of var

    // inner scope indicated by {}
    // vars defined outside of inner scope will not be changed unless explicitly referenced;

    //first is string, second is num
    let spaces = "   ";
    let spaces = spaces.len();

//     8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize

// i indicates pos or neg
// u indicates probably pos

//  floats
let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // operations
     // addition
     let sum = 5 + 10;

     // subtraction
     let difference = 95.5 - 4.3;
 
     // multiplication
     let product = 4 * 30;
 
     // division
     let quotient = 56.7 / 32.2;
     let truncated = -5 / 3; // Results in -1
 
     // remainder
     let remainder = 43 % 5;

     // booleans
     let t = true;

     let f: bool = false; // with explicit type annotation

    //  chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuple variety of types into one compound type, fixed length
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // to get different values from tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // or dot notation
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // array elements must be of same type
    // arrays have fixed length
    let a = [1, 2, 3, 4, 5];

    // vector provided by standard library is allowed to grow and shrink

    // array type and num elements
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // bracket notation
    
    let first = a[0];
    let second = a[1];

    // fill
    let c = [3; 5];
    // = let c = [3, 3, 3, 3, 3];
}
