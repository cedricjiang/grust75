// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            // clone on Option (if it's Some) will clone the content inside,
            // which is Rc here. Cloning a shared pointer is considered cheap
            Some(r) => {
                Self::max_depth(r.borrow().left.clone())
                    .max(Self::max_depth(r.borrow().right.clone()))
                    + 1
            }
            None => 0,
        }
    }
}
