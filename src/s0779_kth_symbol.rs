struct Solution {}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let res = 0;

        let mut nums = vec![0];

        for j in 1..n {
            let mut inserted = 0;
            nums.clone().iter().enumerate().for_each(|(i, f)| {
                println!("{j}: Insert {} at {}", if *f == 0 { 1 } else { 0 }, i+1);
                nums.insert(i+1+i as usize, if *f == 0 { 1 } else { 0 });
                inserted+=1;
                println!("num: {:?} ", nums);
            })
        }
        println!("num: {:?} index {}", nums, k-1);
        nums[(k-1) as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
        assert_eq!(Solution::kth_grammar(3, 3), 1);
        assert_eq!(Solution::kth_grammar(4, 4), 0);
    }
}