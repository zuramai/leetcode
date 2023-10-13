#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let list_to_reversed_num = |list: Option<Box<ListNode>>| {
          let mut curr = &list;
          let mut nums = String::from("");
          while let Some(c) = curr {
            nums.push_str(&c.val.to_string());
            curr = &c.next;
          }
          println!("nums {:?}", nums);
          
          nums.chars().rev().collect::<String>().parse::<i64>().unwrap()
        };
        
        let num1 = list_to_reversed_num(l1);
        let num2 = list_to_reversed_num(l2);
        println!("num1 {:?}", num1);
        println!("num2 {:?}", num2);
        let total = num1 + num2;
        println!("total {:?}", total);

        let num_str = total.to_string();
        let num_chars = num_str.chars();

        let mut list_result: Option<Box<ListNode>> = None;

        for c in num_chars {
          let num = c.to_digit(10).unwrap() as i32;
          println!("current char {:?}", c);
          if list_result.is_none() {
            list_result = Some(Box::new(ListNode::new(num)));
          }
          else {
            let temp = *list_result.unwrap();
            let new_node = Some(Box::new(ListNode {
              next: Some(Box::new(temp)),
              val: num
            }));
            list_result = new_node;
          }
        }

        list_result
    }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test() {

    let l1 = Some(Box::new(ListNode {
      val: 9,
      next: None
    }));
    let l2 = Some(Box::new(ListNode {
      val: 1,
      next: Some(Box::new(ListNode {
        val: 9,
        next:  Some(Box::new(ListNode {
          val: 9,
          next: Some(Box::new(ListNode {
            val: 9,
            next:  Some(Box::new(ListNode {
              val: 9,
              next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                  val: 9,
                  next:  Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                      val: 9,
                      next: Some(Box::new(ListNode::new(9))),
                    }))
                  })),
                })),
              })),
            })),
          }))
        })),
      }))
    }));
    println!("Result: {:?}", Solution::add_two_numbers(l1, l2));
  }
}