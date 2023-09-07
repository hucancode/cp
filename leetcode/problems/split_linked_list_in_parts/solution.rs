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
    pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;
        let mut n = 0;
        let mut node = &head;
        while let Some(x) = node {
            node = &x.next;
            n += 1;
        }
        let mut remainder = n%k;
        let mut len = n/k;
        let mut head = head;
        let mut ret = vec![None;k];
        for i in 0..k {
            let n = len + if i < remainder {1} else {0};
            ret[i] = head;
            let mut node = &mut ret[i];
            for _ in 0..n {
                if let Some(x) = node {
                    node = &mut x.next;
                }
            }
            head = node.take();
        }
        return ret;
    }
}