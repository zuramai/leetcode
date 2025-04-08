struct Solution;
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes().windows(2).map(|a| a[0].abs_diff(a[1]) as i32).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::score_of_string("ada".to_string()), 6);
    }
}