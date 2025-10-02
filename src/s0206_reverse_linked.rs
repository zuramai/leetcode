use crate::utils::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use crate::s0206_reverse_linked::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_list(Some(vec![1,2,3,4,5].into())),
            Some(vec![5,4,3,2,1].into())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reverse_list(Some(vec![].into())),
            Some(vec![].into())
        );
    }
}