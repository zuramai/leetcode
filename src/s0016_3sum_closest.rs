struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut closest_diff = i32::MAX;
        let mut closest_sum = i32::MAX;

        nums.sort();

        for (i, num) in nums.iter().enumerate() {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            
            while l < r {
                let sum = nums[l] + nums[r] + num;
                let diff = sum.abs_diff(target) as i32;
                // println!("diff {}, closest diff {}, i {}, l {} r {}, sum {}, closest sum {}", diff, closest_diff, i, l, r, sum, closest_sum);
                if diff < closest_diff {
                    closest_diff = diff.abs();
                    closest_sum = sum;
                }
                
                if sum < target {
                    l += 1;
                }
                else if sum > target {
                    r -= 1;
                } else {
                    return sum;
                }
            }

            
        }
        closest_sum
    }
}