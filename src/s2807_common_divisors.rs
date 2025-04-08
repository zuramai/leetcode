use crate::utils::ListNode;


struct Solution;

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      println!("testing");
        if let Some(head) = head {
          let mut curr = head.clone();
          while curr.next.is_some() {
            let actual_next = head.next.clone().unwrap();
            let mut new_node = Box::new(ListNode::new(gcd(curr.val, actual_next.val)));

            new_node.next = Some(actual_next.clone());
            curr.next = Some(new_node);
            curr = actual_next;
          }
          return Some(head)
        }
        None
    }  
}
fn gcd(mut a: i32, mut b: i32) -> i32 {
  if b > a {
    (a,b) = (b,a);
  }
  dbg!("gcd");
  while b != 0 {
    let r = a % b;
    a = b;
    b = r;
    return a;
  }

  return 0;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
      dbg!("test");
        assert_eq!(Solution::insert_greatest_common_divisors(Some(vec![18, 6, 10, 3].into())), Some(vec![18,6,6,2,10,1,3].into()));
    }
}