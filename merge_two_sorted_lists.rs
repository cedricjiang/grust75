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

        let mut head = ListNode::new(0);
        let mut curr = &mut head;

        while let (Some(l1), Some(l2)) = (&list1, &list2) {
            if l1.val < l2.val {
                let tmp = list1.as_mut()?.next.take();
                curr.next = list1;
                list1 = tmp;
            } else {
                let tmp = list2.as_mut()?.next.take();
                curr.next = list2;
                list2 = tmp;
            }

            curr = curr.next.as_mut()?;
        }

        curr.next = list1.or(list2);

        head.next
    }
}
