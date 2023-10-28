struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![1; n as usize]; m as usize];

        dp[0][1] = 1;
        dp[1][0] = 1;

        for i in 1..(m as usize) {
            for j in 1..(n as usize) {
                dp[i][j] = dp[i-1][j] + dp[i][j-1];
            }
        }
        dp[(m-1) as usize][(n-1) as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}