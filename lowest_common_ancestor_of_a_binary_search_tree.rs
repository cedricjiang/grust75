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
        // safe to unwrap, they cannot be None, same for root later
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        let mut result = root;

        loop {
            let tmp;
            {
                let mut node = result.as_ref().unwrap().borrow_mut();
                if p < node.val && q < node.val {
                    tmp = node.left.take();
                } else if p > node.val && q > node.val {
                    tmp = node.right.take();
                } else {
                    break;
                }
            }
            result = tmp;
        }

        result
    }
}
