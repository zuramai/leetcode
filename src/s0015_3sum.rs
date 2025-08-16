struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort();

        for (i, num1) in nums.iter().enumerate() {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            
            while l < r {

                if nums[l] + nums[r] + num1 == 0 {
                    result.push(vec![*num1, nums[l], nums[r]]);
                    l+=1;
                    while nums[l] == nums[l-1] && l < r {
                        l+=1;
                    }
                } else if nums[l] + nums[r] + num1 > 0 {
                    r-=1;
                } else if nums[l] + nums[r] + num1 < 0 {
                    l+=1;
                }
            }
        }

        result
    }
}