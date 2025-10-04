use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // convert s1 to hash table
        let mut set = HashMap::new();
        for c in s1.chars() {
            *set.entry(c).or_insert(0) += 1;
        }


        let mut chars = s2.chars().collect::<Vec<char>>();
        'a: for v in chars.windows(s1.len()) {
            let mut s = set.clone();
            for c in v {
                if let Some(v) = s.get_mut(&c) {
                    *v -= 1;
                } else {
                    continue 'a;
                }
            }
            if s.into_values().all(|x| x == 0) {
                return true;
            }
        }
        return false;
    }
}