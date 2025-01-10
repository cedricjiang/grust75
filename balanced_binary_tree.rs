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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced_with_height(root).0
    }

    fn is_balanced_with_height(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        match root {
            Some(r) => {
                // clone on Option (if it's Some) will clone the content inside,
                // which is Rc here. Cloning a shared pointer is considered cheap
                let (left_balanced, left_height) =
                    Self::is_balanced_with_height(r.borrow().left.clone());
                let (right_balanced, right_height) =
                    Self::is_balanced_with_height(r.borrow().right.clone());

                if !left_balanced || !right_balanced || (left_height - right_height).abs() > 1 {
                    (false, 0)
                } else {
                    (true, left_height.max(right_height) + 1)
                }
            }
            None => (true, 0),
        }
    }
}
