use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        (0..n).fold((0,1), |(a,b), _| (b, a+b)).0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}