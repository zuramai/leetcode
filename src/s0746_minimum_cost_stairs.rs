struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut index_cost = cost.clone();
        let len = cost.len();
        for i in 2..len {
            index_cost[i] = i32::min(index_cost[i-1], index_cost[i-2]) + cost[i];
        }
        i32::min(index_cost[len-1], index_cost[len-2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10,15,20]), 15);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]), 6);
    }
}