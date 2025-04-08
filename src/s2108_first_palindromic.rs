use std::mem;


struct Solution;

impl Solution {

    pub fn first_palindrome(words: Vec<String>) -> String {
        let mut result = String::new();

        'parent: for mut string in words {
            let mut ptr1 = 1;
            let split: Vec<&str> = string.split("").collect();
            let mut ptr2 = split.len() - 2;
            println!("testing {} {}", string, split.len());
    
            for i in 0..split.len()/2+1 {
                println!("{} {:?} ({} == {})",i, split, ptr1, ptr2);
                if ptr1 == ptr2 || ptr1 > ptr2{
                    println!("dapet {}",i);
                    mem::swap(&mut result, &mut string);
                    break 'parent;
                }
                if split[ptr1] != split[ptr2] {
                    println!("break {} == {} ", split[ptr1], split[ptr2]);
                    break;
                }
                ptr1 += 1;
                ptr2 -= 1;
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::first_palindrome(utils::vec_string(vec!["abc","car","ada","racecar","cool"])), "ada");
        assert_eq!(Solution::first_palindrome(utils::vec_string(vec!["notapalindrome","racecar"])), "racecar");
        assert_eq!(Solution::first_palindrome(utils::vec_string(vec!["def","ghi"])), "");
    }
}