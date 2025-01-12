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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre_slow = &None;
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref()?.next.is_some() {
            pre_slow = slow;
            slow = &slow.as_ref()?.next;
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }

        match pre_slow {
            Some(node) => {
                // the type of Option<Box<ListNode>> makes the problem really
                // annoying. ideally it should use Rc / shared pointer so one
                // can just return a clone of `slow` with low cost. here what I
                // try to achieve is to find the middle node and "cut" from
                // there so the middle node will be returned to the caller
                // without cloning half of the list and the input list will end
                // at the cut point. that said, the input list is owned by this
                // function and will be destructed anyway...
                unsafe {
                    (*((node as *const Box<ListNode>) as *mut Box<ListNode>))
                        .next
                        .take()
                }
            }
            None => head,
        }
    }
}
