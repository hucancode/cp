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
    pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut i = 1;
        let mut node_a = list1.as_mut();
        let mut node_b = None;
        let mut node_c = None;
        while let Some(node) = node_a {
            if i >= a {
                node_b = node.next.take();
                node.next = list2;
                node_c = node.next.as_mut();
                break;
            }
            node_a = node.next.as_mut();
            i += 1;
        }
        while let Some(node) = node_b {
            node_b = node.next;
            i += 1;
            if i > b {
                break;
            }
        }
        while let Some(node) = node_c {
            if node.next.is_some() {
                node_c = node.next.as_mut();
            } else {
                node.next = node_b;
                break;
            }
        }
        return list1;
    }
}
