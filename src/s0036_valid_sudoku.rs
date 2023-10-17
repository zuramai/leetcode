use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        println!("New Check: {:?}", board);
        let mut is_valid = true;
        let mut check_is_valid = |nums: &Vec<char>| {
            // convert to hashset
            let mut set: HashSet<char> = HashSet::new();
            for num in nums {
                let success_insert = set.insert(*num);
                if *num != '.' && !success_insert {
                    is_valid = false;
                    break;
                }
            }
        };

        // Check per row
        for row in &board {
            check_is_valid(row);
        }
        // Check per cols
        for i in 0..9 {
            let cols = (0..9).map(|j| board[j][i]).collect::<Vec<char>>();
            check_is_valid(&cols)
        }
        // Check subgrid
        for i in 0..3 {
            for j in 0..3 {
                let mut i2 = i * 3;
                let mut subgrid = vec![];
                while i2 < (i + 1) * 3 {
                    let mut j2 = j * 3;
                    while j2 < (j + 1) * 3 {
                        subgrid.push(board[i2][j2]);
                        j2 += 1;
                    }
                    i2 += 1;
                }
                check_is_valid(&subgrid);
            }
            let cols = (0..3).map(|j| board[j][i]).collect::<Vec<char>>();
            check_is_valid(&cols)
        }
        is_valid
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::is_valid_sudoku(vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']]), true);
    }
    #[test]
    pub fn test_false() {
        assert_eq!(Solution::is_valid_sudoku(vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']]), false);
        assert_eq!(Solution::is_valid_sudoku(vec![
            vec!['.','.','.','.','5','.','.','1','.'],
            vec!['.','4','.','3','.','.','.','.','.'],
            vec!['.','.','.','.','.','3','.','.','1'],
            vec!['8','.','.','.','.','.','.','2','.'],
            vec!['.','.','2','.','7','.','.','.','.'],
            vec!['.','1','5','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','2','.','.','.'],
            vec!['.','2','.','9','.','.','.','.','.'],
            vec!['.','.','4','.','.','.','.','.','.']]), false);
    }
}