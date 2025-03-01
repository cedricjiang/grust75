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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == p || root == q {
            return root;
        }

        let r = root?;

        let left = Self::lowest_common_ancestor(r.borrow().left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(r.borrow().right.clone(), p.clone(), q.clone());

        if left.is_some() {
            if right.is_some() {
                Some(r)
            } else {
                left
            }
        } else {
            right
        }
    }
}
