struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = "";
        let first_word = &strs[0];
        let first_word_length = &first_word.len();
        if first_word_length == &(0 as usize) {
            return result.into()
        }
        println!("CHECKING {:?}", strs);
        for i in (0 as usize)..*first_word_length+1 {
            let word_to_check = &first_word[0..i];
            if strs.iter().all(|word| word.starts_with(word_to_check)) {
                println!("MASUK {}", i);
                result = word_to_check;
            } else {
                break;
            }
        }
        result.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn longest_common_prefix() {
        assert_eq!(Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]), "fl".to_string());
        assert_eq!(Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]), "".to_string());
        assert_eq!(Solution::longest_common_prefix(vec!["reflower".into(),"flow".into(),"flight".into()]), "".to_string());
        assert_eq!(Solution::longest_common_prefix(vec!["a".into()]), "a".to_string());
        assert_eq!(Solution::longest_common_prefix(vec!["".into()]), "".to_string());
    }
    
}