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

pub fn vec_str(vec: Vec<&str>) -> Vec<String> {
  vec.iter().map(|s| s.to_string()).collect()
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

pub fn vec_string(value: Vec<&str>) -> Vec<String> {
    value.iter().map(|v| v.to_string()).collect()
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