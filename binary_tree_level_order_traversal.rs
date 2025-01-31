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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();

        if let Some(r) = root {
            queue.push_back(Some(r));
            // "abuse" None as level separator
            queue.push_back(None);
        }

        let mut result = Vec::new();
        let mut level = Vec::new();

        while let Some(item) = queue.pop_front() {
            match item {
                Some(node) => {
                    let n = node.borrow();
                    level.push(n.val);
                    // cloning an Option will clone its content which is Rc
                    // cloning Rc is cheap and necessary here
                    if n.left.is_some() {
                        queue.push_back(n.left.clone());
                    }
                    if n.right.is_some() {
                        queue.push_back(n.right.clone());
                    }
                }
                None => {
                    result.push(level);
                    level = Vec::new();
                    // if queue is empty, stop, don't add None
                    // otherwise you are stuck in a loop
                    if !queue.is_empty() {
                        queue.push_back(None);
                    }
                }
            }
        }

        result
    }
}
