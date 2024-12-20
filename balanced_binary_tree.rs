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
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced_with_height(&root).0
    }

    fn is_balanced_with_height(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        match root {
            Some(r) => {
                let left_result = Self::is_balanced_with_height(&r.borrow().left);
                let right_result = Self::is_balanced_with_height(&r.borrow().right);

                if !left_result.0 || !right_result.0 || (left_result.1 - right_result.1).abs() > 1 {
                    (false, 0)
                } else {
                    (true, cmp::max(left_result.1, right_result.1) + 1)
                }
            }
            None => (true, 0),
        }
    }
}
