use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }

  
}

impl From<Box<ListNode>> for Vec<i32> {
  fn from(mut value: Box<ListNode>) -> Self {
    let mut value = Some(&value);
    let mut v: Vec<i32> = vec![];

    while let Some(next) = value {
      v.push(next.val);
      value = next.next.as_ref()
    }

    v
  }
}
impl From<Vec<i32>> for Box<ListNode> {
  fn from(mut value: Vec<i32>) -> Self {
    value.reverse();
    let mut tail: Option<Box<ListNode>> = None;
    for v in value {
      tail = Some(Box::new(ListNode {
        val: v,
        next: tail,
      }));
    }
    tail.unwrap()
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}