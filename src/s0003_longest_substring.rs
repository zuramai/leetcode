use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start_i = 0;
        let mut end_i = 1;
        if s.is_empty() {
            return 0;
        }
        let mut max_len: i32 = 1;
        let mut set: Vec<char> = Vec::new();
        let mut indexes: HashMap<char, usize> = HashMap::new();
        let chars = s.chars();
        

        for (i, c) in chars.enumerate() {
            if set.contains(&c) {
                let c_index = set.iter().position(|v| v == &c).unwrap();
                start_i += i;
                
                set.drain(0..c_index+1 as usize);
            }
            set.push(c);
            indexes.insert(c, set.len()-1);
            if set.len() as i32 > max_len {
                max_len = set.len() as i32;
            }

        }
        
        end_i += 1;
        
        return max_len;
    }
}