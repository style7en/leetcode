//Definition for a binary tree node.
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
      right: None,
    }
  }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn is_same_tree(
  mut p: Option<Rc<RefCell<TreeNode>>>,
  mut q: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
  check(p, q)
}

fn check(mut p: Option<Rc<RefCell<TreeNode>>>, mut q: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if p.is_none() && q.is_none() {
    return true;
  }
  if p.is_none() || q.is_none() {
    return false;
  }
  let p_left = p.as_mut().unwrap().borrow_mut().left.take();
  let p_right = p.as_mut().unwrap().borrow_mut().right.take();
  let q_left = q.as_mut().unwrap().borrow_mut().left.take();
  let q_right = q.as_mut().unwrap().borrow_mut().right.take();
  return p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val
    && check(p_left, q_left)
    && check(q_right, p_right);
}
