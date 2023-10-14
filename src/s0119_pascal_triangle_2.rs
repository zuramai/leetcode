struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = vec![vec![1]];

        for row in 1..row_index+1 {
            let mut new_row: Vec<i32> = result.last().unwrap().windows(2).map(|a| a[0] + a[1]).collect();
            result.push([&[1], new_row.as_slice(), &[1]].concat());
        }
        result.last().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::get_row(5), vec![1,4,6,4,1]);
    }
}