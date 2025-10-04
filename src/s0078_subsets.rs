use std::ops::Deref;

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.iter().fold(vec![vec![]], |mut acc, curr| {
            let n = acc.iter().map(|mut v| {
                let mut v = v.clone();
                v.push(*curr);
                v
            }).collect::<Vec<_>>();
            acc.extend(n);
            return acc; 
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::subsets(vec![1,2,3]),
            vec![vec![1,2,3],vec![1,2],vec![1,3],vec![1],vec![2,3],vec![2],vec![3]]
        );
    }
}