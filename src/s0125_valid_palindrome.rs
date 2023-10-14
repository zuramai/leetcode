struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered = s.chars().filter_map(|s|  match s.is_alphanumeric() {
            false => None,
            true => Some(s.to_ascii_lowercase())
        }).collect::<Vec<char>>();
        let mut result = true;

        let len = filtered.len();
        if len == 0 {
            return true
        }

        let last_index = len-1;
        for i in 0..len/2  {
            let current_char = filtered[i];
            if current_char.ne(&filtered[last_index-i]) {
                result = false;
                break;
            }
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".into()), true);
        assert_eq!(Solution::is_palindrome("race a car".into()), false);
        assert_eq!(Solution::is_palindrome(" ".into()), true);
        assert_eq!(Solution::is_palindrome("a".into()), true);
    }
}