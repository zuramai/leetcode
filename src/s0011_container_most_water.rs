use std::cmp;

struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j): (usize, usize) = (0, height.len()-1);
        let mut water = 0;
        
        while i < j {
            water = cmp::max(water, (j - i) * cmp::min(height[i], height[j]) as usize);
            if height[j] < height[i] {
                j -= 1;
            } else {
                i += 1;
            }
        }

        water as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::max_area(vec![1,8,6,2,5,4,8,3,7]),
            49
        );
        assert_eq!(
            Solution::max_area(vec![1,1]),
            1
        );
    }
}