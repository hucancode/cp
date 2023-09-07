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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut head) = head {
            let mut ret = head;
            let mut list = ret.next.take();
            while let Some(mut node) = list {
                list = node.next.take();
                node.next = Some(ret);
                ret = node;
            }
            Some(ret)
        } else {
            None
        }
    }
}