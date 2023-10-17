use crate::utils::ListNode;

struct Solution {}
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(x), None) | (None, Some(x)) => Some(x),
            (Some(mut l1), Some(mut l2)) => {
                if l1.val >= l2.val {
                    l2.next = Solution::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                } else {
                    l1.next = Solution::merge_two_lists(Some(l2), l1.next);
                    Some(l1)
                }
            } 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(
            vec![1,1,2,3,4,4], 
            Vec::<i32>::from(
                Solution::merge_two_lists(Some(Box::<ListNode>::from(vec![1,2,4])), Some(Box::<ListNode>::from(vec![1,3,4]))).unwrap())
            );
    }
}