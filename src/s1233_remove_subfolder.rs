struct Solution {}
impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::remove_subfolders(
            utils::vec_str(vec!["/a","/a/b","/c/d","/c/d/e","/c/f"])), 
            vec!["/a","/c/d","/c/f"]
        )
    }
}