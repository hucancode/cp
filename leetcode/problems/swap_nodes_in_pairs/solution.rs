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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut a) = head {
            if let Some(mut b) = a.next.take() {
                if let Some(c) = Self::swap_pairs(b.next.take()) {
                    a.next.replace(c);
                }
                b.next.replace(a);
                Some(b)
            } else {
                Some(a)
            }
        } else {
            None
        }
    }
}