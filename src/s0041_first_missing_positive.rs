
struct Solution{}
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut min = 0;
        nums.sort();
        nums.iter().for_each(|num| {
            if *num == (min+1) as i32 {
                min += 1;
            }
        });
        min+1
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::first_missing_positive(vec![2,1]),
            3
        );
        assert_eq!(
            Solution::first_missing_positive(vec![1,2,0]),
            3
        );
        assert_eq!(
            Solution::first_missing_positive(vec![3,4,-1,1]),
            2
        );
    }
}