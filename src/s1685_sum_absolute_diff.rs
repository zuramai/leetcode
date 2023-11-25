struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; nums.len()];

        for i in 0..res.len() {
            for j in 0..nums.len() {
                res[i] = res[i] + (nums[i]-nums[j]).abs() as i32;
            }
        }
        res 
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2,3,5]),
            vec![4,3,5]
        );
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![1,4,6,8,10]),
            vec![24,15,13,15,21]
        );
    }
}