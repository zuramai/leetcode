struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        if n == 1 {
            return 1;
        }
        let w: i64 = ((n-1) as i64 * 2) - 1;
        let res = Solution::colored_cells(n-1) + 4 + (w - 1) * 2;
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test1() {
        assert_eq!(Solution::colored_cells(1), 1);
    }
    #[test]
    pub fn test2() {
        assert_eq!(Solution::colored_cells(2), 5);
    }
    #[test]
    pub fn test3() {
        assert_eq!(Solution::colored_cells(3), 13);
    }
}