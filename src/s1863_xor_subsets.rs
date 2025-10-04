struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;

        // get all subsets;
        let subsets = nums.iter().fold(vec![vec![]], |mut acc, curr| {
            acc.extend(
                acc.iter().map(|a| {
                    let mut a = a.clone();
                    a.push(curr);
                    a
               }).collect::<Vec<_>>()
            );
            acc
        });

        dbg!(&subsets);
        
        subsets.iter().map(|sub| sub.iter().fold(0, |mut acc,curr| acc ^ *curr)).sum()

    }
}


#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::subset_xor_sum(vec![1,3]), 6);
        assert_eq!(Solution::subset_xor_sum(vec![5,1,6]), 28);
        assert_eq!(Solution::subset_xor_sum(vec![3,4,5,6,7,8]),480);
    }
}