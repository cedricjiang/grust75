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
use std::mem;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut head = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut head;

        // do the merge iteratively, a bit harder than recursion
        while list1.is_some() && list2.is_some() {
            let mut holder: Option<Box<ListNode>> = None;
            if (list1.as_ref().unwrap().val < list2.as_ref().unwrap().val) {
                mem::swap(&mut holder, &mut list1.as_mut().unwrap().next);
                curr.as_mut().unwrap().next = list1;
                list1 = holder;
            } else {
                mem::swap(&mut holder, &mut list2.as_mut().unwrap().next);
                curr.as_mut().unwrap().next = list2;
                list2 = holder;
            }
            curr = &mut curr.as_mut().unwrap().next;
        }

        if list1.is_some() {
            curr.as_mut().unwrap().next = list1;
        } else {
            // list2 may be None, but it's fine - just no-op
            curr.as_mut().unwrap().next = list2;
        }

        head.unwrap().next
    }
}
