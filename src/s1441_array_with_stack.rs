struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut stack = vec![];
        let mut res = vec![];
        let ns: Vec<i32> = (1..n+1).collect();
        let mut i =0;
        let mut  j = 0;
        loop {
            stack.push(ns[i]);
            res.push("Push".into());
            if *stack.last().unwrap() != target[j] {
                stack.pop();
                res.push("Pop".into());
            } else {
                j+=1;
            }

            if stack.len() >= target.len() {
                break;
            }
            i += 1;
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::build_array(vec![1,3], 3), vec!["Push","Push","Pop","Push"]);
        assert_eq!(Solution::build_array(vec![1,2,3], 3), vec!["Push","Push","Push"]);
        assert_eq!(Solution::build_array(vec![1,2], 4), vec!["Push","Push"]);
    }
}