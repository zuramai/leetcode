// Definition for a binary tree node.

use std::mem;
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::TreeNode;
type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution {}
impl Solution {
    pub fn solve(node: &Node, res: &mut Vec<i32>, level: i32) {
        if let Some(node) = node {
            match res.get((level-1) as usize) {
                Some(ref mut val) => {
                    if &node.borrow().val > val {
                        println!("{} > {}", node.borrow().val, val);
                        res[(level-1) as usize] = node.borrow().val;
                    }
                },
                None => {
                    res.insert((level-1) as usize, node.borrow().val)
                }
            }
            Solution::solve(&node.borrow().left, res, level + 1);
            Solution::solve(&node.borrow().right, res, level + 1);
        }

    }
    pub fn largest_values(root: Node) -> Vec<i32> {
        let mut data: Vec<i32> = vec![];

        Solution::solve(&root, &mut data, 1);
        data
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        // assert_eq!(Solution::largest_values(vec![1,2,3,1]), true);
        // assert_eq!(Solution::largest_values(vec![1,2,3,4]), false);
    }
}