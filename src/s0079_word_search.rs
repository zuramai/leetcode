struct Solution;

type Position = (usize, usize);

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let mut ss = vec![];
        let chars: Vec<char> = word.chars().collect();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == chars[0] {
                    ss.push((i, j));
                }
            }
        }
        for s in ss.iter() {
            let mut visited = vec![];
            println!("start");
            if Self::search(&mut board, &chars, visited, s, 0) == true {
                return true
            }
            println!("end");
        }

        return false
    }
    
    fn search(board: &mut Vec<Vec<char>>, word: &Vec<char>, mut visited: Vec<Position>, pos: &Position, word_index: usize) -> bool {
        if pos.0 >= board.len() || pos.1 >= board[0].len()  {
            return false;
        }
        println!("checking {}", word[word_index]);
        if board[pos.0][pos.1] == word[word_index] {
            visited.push(*pos);
            println!("pos {:?}, {}, {}", pos, &board[pos.0][pos.1], word[word_index]);
            if word.len() - 1 == word_index {
                return true;
            }

            if (!visited.contains(&(pos.0 + 1, pos.1)) && Solution::search(board, word, visited.clone(), &(pos.0 + 1, pos.1), word_index+1) ||
                (!visited.contains(&(pos.0, pos.1 + 1)) && Solution::search(board, word, visited.clone(), &(pos.0, pos.1 + 1), word_index+1))|| 
                (pos.0 > 0 && !visited.contains(&(pos.0 - 1, pos.1)) && Solution::search(board, word,visited.clone(), &(pos.0 - 1, pos.1), word_index+1))|| 
                (pos.1 > 0 && !visited.contains(&(pos.0, pos.1 - 1)) && Solution::search(board, word, visited.clone(),&(pos.0, pos.1 - 1), word_index+1))) {
                    return true;
                }
        } 
        return false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCCED".to_string()),
            true
        );
    }
    #[test]
    pub fn test2() {
        assert_eq!(
            Solution::exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCB".to_string()),
            false
        );
    }
    #[test]
    pub fn test3() {
        assert_eq!(
            Solution::exist(vec![vec!['a','a','a','a'],vec!['a','a','a','a'],vec!['a','a','a','a']], "aaaaaaaaaaaaa".to_string()),
            false
        );
    }
    #[test]
    pub fn test4() {
        assert_eq!(
            Solution::exist(vec![vec!['A','B','C','E'],vec!['S','F','E','S'],vec!['A','D','E','E']], "ABCESEEEFS".to_string()),
            true
        );
    }
}