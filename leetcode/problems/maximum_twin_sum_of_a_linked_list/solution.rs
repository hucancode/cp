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

use std::collections::VecDeque;
use std::cmp::max;
impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut q = VecDeque::new();
        while let Some(x) = head {
            q.push_back(x.val);
            head = x.next;
        }
        let mut ret = 0;
        while !q.is_empty() {
            let a = q.pop_front().unwrap_or(0);
            let b = q.pop_back().unwrap_or(0);
            ret = max(ret, a+b);
        }
        ret
    }
}