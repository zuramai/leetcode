struct Solution {}

#[derive(PartialEq, Debug)]
enum ParenthesesType {
    Curly,
    Square,
    Normal
}

#[derive(Debug)]
struct Parentheses (ParenthesesType, bool);

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut valid = true;

        let mut stack: Vec<Parentheses> = vec![];

        for c in s.chars() {
            let newP = match c {
                '{' => Parentheses (ParenthesesType::Curly, true),
                '[' => Parentheses (ParenthesesType::Square, true),
                '(' => Parentheses (ParenthesesType::Normal, true),
                '}' => Parentheses (ParenthesesType::Curly, false),
                ']' => Parentheses (ParenthesesType::Square, false),
                ')' => Parentheses (ParenthesesType::Normal, false),
                _ => Parentheses (ParenthesesType::Normal, false),
            };


            if let Some(last) = stack.last() {
                // The parentheses is not closing the last opened parentheses. Wrong closing
                if newP.1 == false && last.0 != newP.0 {
                    valid = false;
                    break;
                }

                // If valid closing, pop the stack
                if last.0 == newP.0 && !newP.1 {
                    stack.pop();
                }
            } else if !newP.1 {
                valid = false;
                break;
            }

            if newP.1 {
                stack.push(newP);
            }
        };

        if stack.len() > 0 {
            valid = false;
        }

        valid
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::is_valid("()".into()), true);
        assert_eq!(Solution::is_valid("(){}[]".into()), true);
        assert_eq!(Solution::is_valid("(]".into()), false);
        assert_eq!(Solution::is_valid("[".into()), false);
        assert_eq!(Solution::is_valid("]".into()), false);
        assert_eq!(Solution::is_valid("{[]}".into()), true);
        assert_eq!(Solution::is_valid("[[[]".into()), false);
    }
}