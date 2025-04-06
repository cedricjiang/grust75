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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut result = 0;
        let mut pos = 0;

        Self::in_order_early_stop(root, k, &mut pos, &mut result);

        result
    }

    fn in_order_early_stop(
        root: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        pos: &mut i32,
        result: &mut i32,
    ) {
        if let Some(node) = root
            && *pos < k
        {
            Self::in_order_early_stop(node.borrow().left.clone(), k, pos, result);
            *pos += 1;
            if *pos == k {
                *result = node.borrow_mut().val;
                return;
            }
            Self::in_order_early_stop(node.borrow().right.clone(), k, pos, result);
        }
    }
}
