use crate::utils::ListNode;


struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            next: head,
            val: 0
        });
        let mut end = &dummy.clone();
        let mut prev = &mut dummy;

        for _ in 0..n {
            if let Some(node) = &end.next {
                end = node;
            }
        }

        while end.next.is_some() {
            prev = prev.next.as_mut().unwrap();
            end = end.next.as_ref().unwrap();
        }

        let n  = prev.next.as_mut().and_then(|v| v.next.take());
        prev.next = n;

        return dummy.next;
    }
}

#[cfg(test)]
mod tests {
    use crate::s0019_remove_nth_linked_list::Solution;

    #[test]
    fn test_nth() {
        assert_eq!(
            Solution::remove_nth_from_end(Some(vec![1,2,3,4,5].into()), 2),
            Some(vec![1,2,3,5].into())
        );
    }
}