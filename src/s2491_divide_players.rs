
struct Solution;

impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let len = skill.len() - 1;
        let half = skill.len() / 2;
        let avg: i32 = skill.iter().sum::<i32>() / half as i32;
        skill.sort();        
        let mut new_vec = Vec::new();
        for i in 0..half {
            let pair = (skill[i] as i64, skill[len-i] as i64);
            // println!("{} + {} === {}", pair.0, pair.1, avg);
            if pair.0 + pair.1 != avg as i64 {
                return -1;
            }
            new_vec.push(pair);
        }

        let output = new_vec.iter().fold(0, |acc, &curr| acc + curr.0 * curr.1 );
        output

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::divide_players(vec![3,2,5,1,3,4]), 22);
        assert_eq!(Solution::divide_players(vec![3,4]), 12);
    }
}