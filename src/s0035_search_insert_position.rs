struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            if nums[i] >= target   {
                break;
            }
            res+=1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::search_insert(vec![1,2,4], 3), 2);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
        
    }
}