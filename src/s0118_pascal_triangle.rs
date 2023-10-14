struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1]];

        for row in 1..num_rows {
            let mut new_row: Vec<i32> = result.last().unwrap().windows(2).map(|a| a[0] + a[1]).collect();
            println!("row: {:?}", new_row);
            result.push([&[1], new_row.as_slice(), &[1]].concat());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::generate(5), vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]]);
    }
}