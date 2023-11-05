use std::collections::{HashMap, hash_map::Entry};

struct Solution;

impl Solution {
    pub fn get_winner(mut arr: Vec<i32>, k: i32) -> i32 {
        let mut wins = 0;
        let mut current_winner = arr[0];
        let mut i = 1;
        println!("k {k}, len {}", arr.len());
        if k > arr.len() as i32 {
            return *arr.iter().max().unwrap();
        }

        loop {
            if i >= arr.len() {
                i = 0;
            }
            let winner = match current_winner > arr[i] {
                true => current_winner,
                false => arr[i]
            };
            println!("win {}!", winner);
            wins+=1;
            if winner != current_winner {
                println!("RESET");
                wins = 1;
                current_winner = winner;
            }
            
            if wins == k {
                return current_winner;
            }
            i+=1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::get_winner(vec![2,1,3,5,4,6,7], 2), 5);
        assert_eq!(Solution::get_winner(vec![3,2,1], 10), 3);
        assert_eq!(Solution::get_winner(vec![1,25,35,42,68,70], 2), 3);
        assert_eq!(Solution::get_winner(vec![1,11,22,33,44,55,66,77,88,99], 1000000000), 70);
        
    }
}