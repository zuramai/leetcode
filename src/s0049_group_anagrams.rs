use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let mut result_map: HashMap<String, HashMap<char, i32>> = HashMap::new();
        let mut keys_index: HashMap<String, usize> = HashMap::new();
        let mut iter = strs.into_iter();
        // convert each of the string into a hashmap
        while let Some(s) = iter.next() {
            let s_map = Solution::hashmap(s.clone());


            // check for equal hashmap
            let mut f = None;
            for (key ,val) in result_map.iter() {
                if val.eq(&s_map) {
                    f = Some(key);
                    break;
                }
            }


            // if exists, we don't need to insert to hashmap again, we'll just insert to the result
            if f.is_some() {
                dbg!(&keys_index);
                result[keys_index.get(f.unwrap()).unwrap().clone()].push(s.to_string());
            } else {
                // create new group and insert to result_map
                result_map.insert(s.clone(), s_map);
                keys_index.insert(s.to_string(), result.len());
                result.push(vec![s.to_string()]);
            }

        };

        result
    }
    fn hashmap(s: String) -> HashMap<char, i32> {
        let mut map = HashMap::new();

        let mut chars = s.chars();
        while let Some(v) = chars.next() {
            if let Some(mapv) = map.get_mut(&v) {
                *mapv += 1
            } else {
                map.insert(v, 1);
            }
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(
            Solution::group_anagrams(["eat","tea","tan","ate","nat","bat"].map(|v| v.to_owned()).to_vec()), 
            Vec::from([Vec::from(["bat"]),Vec::from(["nat","tan"]),Vec::from(["ate","eat","tea"])])
        );
    }
}