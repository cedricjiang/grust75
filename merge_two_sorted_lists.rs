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
            if (list1.as_ref()?.val < list2.as_ref()?.val) {
                let tmp = list1.as_mut()?.next.take();
                curr.as_mut()?.next = list1;
                list1 = tmp;
            } else {
                let tmp = list2.as_mut()?.next.take();
                curr.as_mut()?.next = list2;
                list2 = tmp;
            }
            curr = &mut curr.as_mut()?.next;
        }

        if list1.is_some() {
            curr.as_mut()?.next = list1;
        } else {
            // list2 may be None, but it's fine - just no-op
            curr.as_mut()?.next = list2;
        }

        head?.next
    }
}
