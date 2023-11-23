struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut result = vec![];
        
        for i in 0..l.len() {
            let mut sorted = nums[(l[i] as usize)..((r[i]+1) as usize)].to_vec();
            sorted.sort();
            let mut first_diff = None;

            let current_result = sorted.windows(2).all(|v| {
                if first_diff.is_none() {
                    first_diff = Some(v[1] - v[0]);
                }
                if v[1] - v[0] != first_diff.unwrap() {
                    return false 
                }

                return true
            });          
            result.push(current_result)
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(vec![4,6,5,9,3,7], vec![0,0,2], vec![2,3,5]),
            vec![true,false,true]
        );
    }
}