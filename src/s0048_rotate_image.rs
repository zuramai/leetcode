use std::ptr;


struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in i..matrix.len() {
                unsafe {
                    ptr::swap(&mut matrix[i][j], &mut matrix[j][i]);
                }
                // Replace var
                // let temp = matrix[i][j];
                // matrix[i][j] = matrix[j][i];
                // matrix[j][i] = temp;
            }
        }
        matrix.iter_mut().for_each(|v| v.reverse());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::rotate(&mut vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]),
            Solution::rotate(&mut vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]]),
        );
    }
}