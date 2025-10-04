use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::TreeNode;

struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![vec![]; 100];

        fn traverse(result: &mut Vec<Vec<i32>>, level: usize, node: Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = node  {
                result[level].push(node.borrow().val);
                traverse(result, level + 1, node.borrow().left.clone());
                traverse(result, level + 1, node.borrow().right.clone());
            } else {
                return;
            }
        }
        traverse(&mut result, 0, root);
        result.into_iter().filter_map(|v: Vec<i32>| v.last().copied()).collect::<Vec<i32>>()
    }
}