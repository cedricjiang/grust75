// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;

        let mut heap = BinaryHeap::new();
        for (i, node) in lists.iter().enumerate() {
            if let Some(n) = node {
                heap.push(Reverse((n.val, i)));
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;

        while let Some(Reverse((_, list_index))) = heap.pop() {
            let mut node = lists[list_index].take().unwrap();
            lists[list_index] = node.next.take();

            if let Some(next) = &lists[list_index] {
                heap.push(Reverse((next.val, list_index)));
            }
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }

        dummy.next
    }
}