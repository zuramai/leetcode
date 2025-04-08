use std::{ops::Index, str::Chars};

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let trim_whitespace = s
            .trim_start();
        let mut chars = trim_whitespace.chars();
        
        let first_char = trim_whitespace.chars().nth(0).unwrap_or('0');
        if first_char.eq(&'-')|| first_char.eq(&'+') {
            chars.next();    
        }
        
        let mut chars: String = chars.skip_while(|f| f == &'0').take_while(|f| f.is_numeric()).collect();
        if chars.len() > 10 {
            return match first_char == '-' {
                true => i32::MIN,
                false => i32::MAX
            }
        }
        if first_char == '-' {
            chars = '-'.to_string() + &chars; 
        }
            
        let res = chars.parse::<i64>().unwrap_or(0);
       
        dbg!(&res);

        let res: i32 = if res > i32::MAX as i64 {
            i32::MAX
        } else if res < i32::MIN as i64{
            i32::MIN
        } else {
            res as i32
        };
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        let res = Solution::my_atoi("42".to_string());
        assert_eq!(res, 42);
    }
    #[test]
    pub fn test2() {
        let res = Solution::my_atoi("-042".to_string());
        assert_eq!(res, -42);
    }
    #[test]
    pub fn test3() {
        let res = Solution::my_atoi("1337c0d3".to_string());
        assert_eq!(res, 1337);
    }
    #[test]
    pub fn test4() {
        let res = Solution::my_atoi("-91283472332".to_string());
        assert_eq!(res, -2147483648);
    }
    #[test]
    pub fn test5() {
        let res = Solution::my_atoi("".to_string());
        assert_eq!(res, 0);
    }
    #[test]
    pub fn test6() {
        let res = Solution::my_atoi("20000000000000000000".to_string());
        assert_eq!(res, 2147483647);
    }
    #[test]
    pub fn test7() {
        let res = Solution::my_atoi("-20000000000000000000".to_string());
        assert_eq!(res, -2147483648);
    }
    #[test]
    pub fn test8() {
        let res = Solution::my_atoi("  0000000000012345678".to_string());
        assert_eq!(res, 12345678);
    }
}
