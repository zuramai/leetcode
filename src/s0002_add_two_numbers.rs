use std::boxed::Box;

use crate::utils::ListNode;

pub struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut total = 0;
        let mut i = 0;
        let mut leftover = 0;
        let mut output_root = None;
        let mut output_last_node: Option<&mut Box<ListNode>> = None;

        while l1.is_some() || l2.is_some() {
            let mut firstnum = 0;
            if let Some(first) = l1 {
                firstnum = first.val;
                l1 = first.next;
            }
            let mut secondnum = 0;
            if let Some(second) = l2 {
                secondnum = second.val;
                l2 = second.next;
            }
            let mut current_total = firstnum + secondnum + leftover;
            if current_total >= 10 {
                leftover = 1;
                current_total = current_total % 10;
            } else {
                leftover = 0;
            };

            let new_node = Some(Box::new(ListNode::new(current_total)));
            if i == 0 {
                output_root = new_node;
                output_last_node = output_root.as_mut();
            } else {
                if let Some(last_node) = output_last_node {
                    last_node.next = new_node;
                    output_last_node = last_node.next.as_mut();
                }
            }
            i += 1;
        }


        if leftover > 0 && let Some(last_node) = output_last_node {
            let new_node = Some(Box::new(ListNode::new(1)));

            last_node.next = new_node;
            output_last_node = last_node.next.as_mut();
        }

        output_root
    }
}
