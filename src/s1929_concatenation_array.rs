use std::mem;

struct Solution;
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let nums2 = nums.clone();
        nums.extend_from_slice(&nums2);
        nums
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::get_concatenation(Vec::from([1,2,3])),
            Vec::from([1,2,3,1,2,3])
        );
    }
}