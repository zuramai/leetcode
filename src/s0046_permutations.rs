struct Solution {}
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Solution::backtrack(&mut result, vec![], nums);
        result
    }

    pub fn backtrack(list: &mut Vec<Vec<i32>>, mut temp_list: Vec<i32>, nums: Vec<i32>) {
        if temp_list.len() == nums.len() {
            list.push(temp_list);
        } else {
            for i in 0..nums.len() {
                if temp_list.contains(&nums[i]) {
                    continue;
                };
                temp_list.push(nums[i]);
                Solution::backtrack(list, temp_list.clone(), nums.clone());
                temp_list.remove(temp_list.len() - 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
