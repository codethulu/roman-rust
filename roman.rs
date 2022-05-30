//program to convert from roman numerals to integers
use std::collections::HashMap;

struct Solution {
    
}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if s.len() > 15 {
            println!("character length must be between 0 and 15");
            return -1;
        }
        let dict = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        
        let mut result = 0;
        let mut i = 0;
        //while i less than string length
        while i < s.len() {
            let c = s.chars().nth(i).unwrap();
            let curr = dict.get(&c).unwrap();
            if i+1 < s.len() {
                let next = dict.get(&s.chars().nth(i + 1).unwrap()).unwrap();
                if curr < next {
                    result += next - curr;
                    i += 1;
                } else {
                    result += curr;
                }
            } else {
                result += curr;
            }
            i += 1;
        }

        return result;

    }
}

fn main() {
    let s = String::from("IX");
    let result = Solution::roman_to_int(s);
    println!("{}", result);
}