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


pub struct Node {
    pub val: int32,
    pub next: Option<Box<Node>>,
    pub prev: Option<Box<Node>>,
}

pub struct LinkedList {
    pub head: Option<Box<Node>>,
    pub length: int32,
    pub tail: Option<Box<Node>>,
}

impl LinkedList {
     pub fn new() -> Self {
        LinkedList { head: None, length: 0 }
    }

    pub fn add(&mut self, val: i32) {
        let new_node = Box::new(Node { val, next: None });

        match self.head.as_mut() {
            // If the list has a head, traverse to the end
            Some(mut node) => {
                while let Some(ref mut next_node) = node.next {
                    node = next_node;
                }
                node.next = Some(new_node);
            }
            // If list is empty, new node becomes head
            None => {
                self.head = Some(new_node);
            }
        }

        self.length += 1;
    }

    fn length(&self) -> int32 {
        self.length
    }
}

let mut list = LinkedList::new();