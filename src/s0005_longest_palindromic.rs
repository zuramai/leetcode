struct Solution {}

impl Solution {
    pub fn is_palindrome(s: &&[u8]) -> bool {
        println!("Checking {s:?}");
        let iter = s.iter();
        iter.clone().eq(iter.rev())
    }
    pub fn longest_palindrome(s: String) -> String {
        let mut longest = String::new();

        let mut window_size = s.len();

        while window_size > 0 {
            let find = s.as_bytes().windows(window_size).find(|s| {
                Solution::is_palindrome(s)
            });
            match find {
                Some(f) => return String::from_utf8(f.to_vec()).unwrap_or("".into()),
                None => window_size -= 1
            }
        }

        longest
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::longest_palindrome("babad".into()), "bab");
        assert_eq!(Solution::longest_palindrome("ac".into()), "a");
        assert_eq!(Solution::longest_palindrome("a".into()), "a");
        assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".into()), "bb");
    }
}