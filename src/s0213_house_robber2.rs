struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let check = |n: &[i32]| {
            let mut maxes = vec![*n.first().unwrap_or(&0), 0];
            for i in 0..n.len() {
                if i <= 1 {
                    maxes[i] = n[i];
                    continue;
                }
                let curr = n[i] + maxes[0];
                maxes[0] = maxes[1].max(maxes[0]);
                maxes[1] = curr;
            }

            return maxes.into_iter().max();
        };
        if nums.len() == 1 {
            return nums[0];
        }
        check(&nums[0..nums.len()-1])
            .max(check(&nums[1..nums.len()])).unwrap()
    }
}