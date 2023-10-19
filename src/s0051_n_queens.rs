struct Solution {}
impl Solution {
    pub fn solve_one() {

    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];



        res
    }   
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![vec![".Q..","...Q","Q...","..Q."],vec!["..Q.","Q...","...Q",".Q.."]]
        );
    }
}