struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut maxes = vec![nums[0], 0];

        for i in 0..nums.len() {
            if i <= 1 {
                maxes[i] = nums[i];
                continue;
            }
            let num = maxes[0] + nums[i];
            maxes[0] = maxes[1].max(maxes[0]);
            maxes[1] = num;
        }
        return maxes[0].max(maxes[1]);
    }
}