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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = &root {
            let mut r = r.borrow_mut();

            // clone on Option (if it's Some) will clone the content inside,
            // which is Rc here. Cloning a shared pointer is considered cheap
            let tmp = Self::invert_tree(r.left.clone());
            r.left = Self::invert_tree(r.right.clone());
            r.right = tmp;
        }

        root
    }
}
