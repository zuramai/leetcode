struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            None => -1,
            Some(n) => n as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::str_str("sadbutsad".into(), "sad".into()),
            0
        );
    }
}