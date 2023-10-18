use std::collections::HashSet;


struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut map = HashSet::new();
        nums.retain(|num| map.insert(*num));
        map.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1,1,2]), 2);
        assert_eq!(Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
    }
}