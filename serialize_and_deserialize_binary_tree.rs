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

struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            let val = node.borrow().val;
            let left = Self::serialize(&self, node.borrow().left.clone());
            let right = Self::serialize(&self, node.borrow().right.clone());

            format!("{val}({left},{right})")
        } else {
            String::new()
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            None
        } else {
            // if serialized correctly, these unwraps are safe
            let first_left_parenthesis_pos = data.find("(").unwrap();
            let val = data[0..first_left_parenthesis_pos].parse().unwrap();
            let body = &data[first_left_parenthesis_pos + 1..data.len() - 1];

            let mut mid_comma_pos = 0;
            let mut parenthesis_count = 0;

            for (i, c) in body.chars().enumerate() {
                match c {
                    ',' => {
                        if parenthesis_count == 0 {
                            mid_comma_pos = i;
                            break;
                        }
                    }
                    '(' => parenthesis_count += 1,
                    ')' => parenthesis_count -= 1,
                    _ => {}
                }
            }

            let mut node = TreeNode::new(val);
            node.left = Self::deserialize(&self, body[0..mid_comma_pos].to_string());
            node.right = Self::deserialize(&self, body[mid_comma_pos + 1..body.len()].to_string());

            Some(Rc::new(RefCell::new(node)))
        }
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */