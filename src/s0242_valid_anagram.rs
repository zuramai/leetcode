use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut chars1 = s.chars();
        let mut chars2 = t.chars();
        let mut map = HashMap::new();
        while let Some(i) = chars1.next() {
            if let Some(k) = map.get_mut(&i) {
                *k += 1
            } else {
                map.insert(i, 1);
            }
        };
        dbg!(&map);
        
        while let Some(i) = chars2.next() {
            if let Some(k) = map.get_mut(&i) {
                *k -= 1
            } else {
                map.insert(i, -1);
            }
        };

        dbg!(&map);

        map.values().all(|v| v == &0)
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::is_anagram("nagaram".into(), "anagram".into()), true);
        assert_eq!(Solution::is_anagram("rat".into(), "car".into()), false);
    }
}