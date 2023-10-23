struct Solution{}
impl Solution {
    pub fn is_power_of_four(n: i64) -> bool {
        n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::is_power_of_four(34), false);
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(5), false);
        assert_eq!(Solution::is_power_of_four(2), false);
        assert_eq!(Solution::is_power_of_four(1), true);
        assert_eq!(Solution::is_power_of_four(-2147483648), true);
        
    }
}