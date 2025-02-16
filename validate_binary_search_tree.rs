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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_with_bounds(root, None, None)
    }

    fn is_valid_bst_with_bounds(
        root: Option<Rc<RefCell<TreeNode>>>,
        left: Option<i32>,
        right: Option<i32>,
    ) -> bool {
        match root {
            Some(r) => {
                let root_val = r.borrow().val;
                if left.map_or(false, |v| root_val <= v) {
                    return false;
                }
                if right.map_or(false, |v| root_val >= v) {
                    return false;
                }
                // cheap to clone the Option to shared pointer
                Self::is_valid_bst_with_bounds(r.borrow().left.clone(), left, Some(root_val))
                    && Self::is_valid_bst_with_bounds(
                        r.borrow().right.clone(),
                        Some(root_val),
                        right,
                    )
            }
            None => true,
        }
    }
}
