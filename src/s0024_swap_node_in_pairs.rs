use std::mem;

use crate::utils::ListNode;

struct Solution {}

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut h) => {
                match h.next {
                    None => Some(h),
                    Some(mut n) => {
                        h.next = Self::swap_pairs(n.next);
                        n.next = Some(h);
                        Some(n)
                    }
                }
            },
            _ => head 
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
    }
}