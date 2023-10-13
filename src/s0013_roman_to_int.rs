use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let iter = chars.iter().enumerate();
        let mut total = 0;
        let mut cont = false;

        for (i, c) in iter {
            println!("giliran {}", c);
            if cont {
                cont = false;
                continue
            }
            let v = map.get(c).unwrap();
            let next_char = chars.get(i+1);
            
            if let Some(nc) = next_char {
                let next_value = map.get(nc).unwrap();
                if next_value > v {
                    total += next_value - v;
                    cont = true;   
                    continue;                 
                } 
                total += v;
                continue;
            } 
            total += v;    
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::roman_to_int("III".into()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
    }
}