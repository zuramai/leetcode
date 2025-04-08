
struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut stack = 0;
        // loop vec
        for log in logs.iter() {
            let dir = log.trim_end_matches("/");
            if dir == "." {
                continue;
            }
            
            if dir == ".." {
                if stack > 0 {
                    stack -= 1
                }
            } else if dir == "/" {
                stack = 0
            } else {
                stack += 1
            }
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::min_operations(utils::vec_string(vec!["d1/","d2/","../","d21/","./"])), 2);
        assert_eq!(Solution::min_operations(utils::vec_string(vec!["d1/","d2/","./","d3/","../","d31/"])), 3);
        assert_eq!(Solution::min_operations(utils::vec_string(vec!["d1/","../","../","../"])), 0);
    }
}