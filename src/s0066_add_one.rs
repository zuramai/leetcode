struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            match digits[i] {
                9 => {
                    digits[i] = 0;
                },
                _ => {
                    digits[i] += 1;
                    return digits;
                }
            }
        }
        digits.insert(0,1);
        digits
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::plus_one(vec![9]),vec![1,0]);
        assert_eq!(Solution::plus_one(vec![1,2,3]),vec![1,2,4]);
        assert_eq!(Solution::plus_one(vec![4,3,2,1]),vec![4,3,2,2]);
        assert_eq!(Solution::plus_one(vec![9,8,7,6,5,4,3,2,1,0]),vec![9,8,7,6,5,4,3,2,1,1]);
    }
}