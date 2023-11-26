struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        
        let chars: Vec<char> = s.chars().collect();
        let mut matrix: Vec<String> = vec!["".into(); num_rows as usize];
        
        let mut is_straight = true; 
        let mut current_row: i32 = 0; 

        for character in chars.iter() {
            matrix[current_row as usize].push(*character);
            
            if current_row == num_rows-1 {
                is_straight = false;
            } else if current_row == 0 {
                is_straight = true;
            }
            current_row += match is_straight {
                false => -1,
                true => 1
            };
        }

        // Transform matrix to string
        matrix.join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 4),
            "PINALSIGYAHRPI".to_string()
        );
        assert_eq!(
            Solution::convert("A".into(), 1),
            "A".to_string()
        );
        assert_eq!(
            Solution::convert("AB".into(), 1),
            "AB".to_string()
        );
    }
}