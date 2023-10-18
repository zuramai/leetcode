struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|n| n != &val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::remove_element(&mut vec![3,2,2,3], 2),
            2
        );
    }
}