struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut new = String::new();
        let mut rev = s.trim().split_whitespace();
        while let Some(word) = rev.next_back() {
            new.push_str(format!(" {}",word).as_str());
        }
        dbg!("{}",&s);

        new.trim_start().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::reverse_words("the sky is blue".into()), "blue is sky the".to_string());
        assert_eq!(Solution::reverse_words("  hello world  ".into()), "world hello".to_string());
        assert_eq!(Solution::reverse_words("a good   example".into()), "example good a".to_string());
    }
}