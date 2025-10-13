// fn main() {
//     println!("Hello, world!");
//     let num: i32 = 1234;
//     let uh = num as char;

//     println!("{}", uh);
// }

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0
        {
        return false;
        }
        else
        {
            let mut y = x.to_string();
            let mut middle: i32 = y.len() / 2;
            let mut right: i32 = 0;
            if x % 2 == 0
            {
                right = middle + 1;
            }
            else
            {
               right = middle + 2;
            }
            while middle >= 0 && right < y.len()
            {
                if y[middle] != y[right]
                {
                    return false;
                }
                middle -= 1;
                right += 1;
            }
            return true;
        }
    }
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let y = x.to_string();
        let mut left = 0;
        let mut right = y.len() - 1;

        while left < right {
            if y.chars().nth(left) != y.chars().nth(right) {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}


impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // Track if the number is negative
        let negative = x < 0;

        // Take the absolute value, convert to string, then reverse the characters
        let s: String = x.abs().to_string().chars().rev().collect();

        // Parse the reversed string to a 32-bit integer
        // Use match to safely handle possible overflows
        match s.parse::<i32>() {
            Ok(n) => {
                if negative {
                    -n
                } else {
                    n
                }
            }
            Err(_) => 0, // If it overflows, return 0
        }
    }
}


fn palindromeTWO(x: i32) -> bool {
    let mut y = x.to_string();
    let mut left = 0;
    let mut right = y.len() - 1;

    while left < right {
        if y.chars().nth(left) != y.chars().nth(right) {
            return false;
        }

        left += 1;
        right 0 -= 1;
    }

    true
}


