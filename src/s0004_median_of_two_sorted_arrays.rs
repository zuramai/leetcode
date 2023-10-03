struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut n1 = nums1;
        let mut n2 = nums2;
        n1.append(&mut n2);
        n1.sort();
        
        let l = n1.len();
        
        let mut median = 0.0;
        if l % 2 == 0 {
            median = (n1.get(l/2-1).unwrap() + n1.get(l/2).unwrap()) as f64 / 2.0;
        } else {
            median = *n1.get(l/2).unwrap() as f64;
        }
        median as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        let res = Solution::find_median_sorted_arrays(vec![1,3], vec![2]);
        let res2 = Solution::find_median_sorted_arrays(vec![1,3], vec![2,4]);
        println!("res1: {}", res);
        println!("res2: {}", res2);
    }
}
