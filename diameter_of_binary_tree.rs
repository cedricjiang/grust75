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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::diameter_of_binary_tree_with_height(root).0;
    }

    fn diameter_of_binary_tree_with_height(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            Some(r) => {
                // clone on Option (if it's Some) will clone the content inside,
                // which is Rc here. Cloning a shared pointer is considered cheap
                let (left_diameter, left_height) =
                    Self::diameter_of_binary_tree_with_height(r.borrow().left.clone());
                let (right_diamter, right_height) =
                    Self::diameter_of_binary_tree_with_height(r.borrow().right.clone());
                let diameter = left_diameter
                    .max(right_diamter)
                    .max(left_height + right_height);
                let height = left_height.max(right_height) + 1;
                (diameter, height)
            }
            None => (0, 0),
        }
    }
}
