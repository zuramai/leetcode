use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for num in nums.iter() {
            *map.entry(num).or_insert(-1) += 1
        }
        let mut total = 0;
        for (_, value) in map.into_iter() {
            total += value * (value + 1) * 1/2;
        }
        total
    }
}

fn main() {
    println!("Result: {}",Solution::num_identical_pairs(vec![1,2,3,1,1,3]));
    println!("Result: {}",Solution::num_identical_pairs(vec![1,1,1,1]));
    println!("Hello, world!");
}
