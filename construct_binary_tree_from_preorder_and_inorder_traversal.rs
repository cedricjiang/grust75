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
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_slice(&preorder, &inorder)
    }

    fn build_tree_slice(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let mut root = TreeNode::new(preorder[0]);

        // unwrap is safe because preorder and inorder are guaranteed to be actual traversal result
        // not only the initial input but also the correct process guarantees that every step in recursion
        let root_in_order_pos = inorder.iter().position(|&x| x == root.val).unwrap();

        root.left = Self::build_tree_slice(
            &preorder[1..root_in_order_pos + 1],
            &inorder[..root_in_order_pos],
        );
        root.right = Self::build_tree_slice(
            &preorder[root_in_order_pos + 1..],
            &inorder[root_in_order_pos + 1..],
        );

        Some(Rc::new(RefCell::new(root)))
    }
}