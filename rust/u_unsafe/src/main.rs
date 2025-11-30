fn main() {
    println!("Hello, world!");
}
// unsafe
// 1. dereference raw pointer
//  2. call unsafe function or method
//  3. access or modify mutable static variables
// 4. implement unsafe trait
// 5. access fields of unions

// 1. dereference raw pointer

let mut num = 5;

// immutable raw pointer
let r1 = &num as *const i32;
// mutable raw pointer
let r2 = &mut num as *mut i32;

// raw pointers allowed to be null and have no automatic cleanup

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

//  2. call unsafe function or method

fn main() {
    unsafe fn dangerous() {

    }

    unsafe {
        dangerous();
    };

    let mut v = vec![1, 2, 3, 4, 5, 6]

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    //  same as above...

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        assert!(mid <= len);

        // borrow checker thinks we're borrowing slice twice mutably which is not allowed
        // (&mut slice[..mid], &mut slice[mid..])

        // unsafe version

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }

    }
}



//  3. access or modify mutable stat,ic variables
// 4. implement unsafe trait
// 5. access fields of unions