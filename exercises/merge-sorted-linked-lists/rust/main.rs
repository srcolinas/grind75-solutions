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
        mut list1: Option<Box<ListNode>>, 
        mut list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;

        // While both lists have nodes, compare and attach the smaller one
        while let (Some(l1), Some(l2)) = (&list1, &list2) {
            if l1.val <= l2.val {
                // Take the node from list1
                let next = list1.as_mut().unwrap().next.take();
                current.next = list1;
                list1 = next;
            } else {
                // Take the node from list2
                let next = list2.as_mut().unwrap().next.take();
                current.next = list2;
                list2 = next;
            }
            // Move our cursor forward
            current = current.next.as_mut().unwrap();
        }

        // Attach the remaining nodes of whichever list isn't empty
        current.next = list1.or(list2);

        dummy.next
    }
}