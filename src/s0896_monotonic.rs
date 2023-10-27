struct Solution;
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut is_increasing: Option<bool> = None;
        return nums.as_slice().windows(2).all(|a| {
            match is_increasing {
                Some(v) => {
                    if (v && a[0] > a[1]) || (!v && a[1] > a[0]) {
                        return false;
                    }
                    return true;
                },
                None => {
                    if a[0] > a[1] {
                        is_increasing = Some(false);
                    }
                    else if a[0] < a[1] {
                        is_increasing = Some(true);
                    }
                    return true;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::s0896_monotonic::Solution;

    #[test]
    pub fn test() {
        assert_eq!(Solution::is_monotonic(vec![1,2,2,3]), true);
        assert_eq!(Solution::is_monotonic(vec![6,5,4,4]), true);
        assert_eq!(Solution::is_monotonic(vec![1,3,2]), false);
    }
}