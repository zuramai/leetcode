struct Solution{}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // convert to chars
        let xstr = x.to_string();
        let chars =  xstr.chars().collect::<Vec<char>>();
        let mut is = false;

        if chars.len() == 1 {
            return true;
        }
        for i in 0..chars.len()/2 {
            let c1 = chars.get(i).unwrap();
            let c2 = chars.get(chars.len() - i - 1).unwrap();
            if c1.ne(c2) {
                is = false;
                break;
            }
            is = true;
        }
        return is
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(2552), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
    }
}