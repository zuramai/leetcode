use std::mem;

struct Solution;

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a,b| b.cmp(a));
        println!("sorted {:?}", nums);

        if nums.len() <= 4 {
            return 0;
        } else {
            return nums[3] - nums.last().unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::min_difference(vec![5,3,2,4]), 0);
        assert_eq!(Solution::min_difference(vec![1,5,0,10,14]), 1);
        assert_eq!(Solution::min_difference(vec![3,100,20]), 0);
    }
}