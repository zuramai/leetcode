struct Solution;

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        return 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::min_capability(vec![2,3,5,9], 2), 5);
    }
}