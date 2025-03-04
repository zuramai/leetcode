struct Solution {}
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: String = x.abs().to_string().chars().rev().collect();
        if x < 0 {
            res = "-".to_string() + &res;
        }
        res.parse::<i32>().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test1() {
        let res = Solution::reverse(123);
        assert_eq!(res, 321);
    }
    #[test]
    pub fn test2() {
        let res = Solution::reverse(-231);
        assert_eq!(res, -132);
    }
    #[test]
    pub fn test3() {
        let res = Solution::reverse(120);
        assert_eq!(res, 21);
    }
}
