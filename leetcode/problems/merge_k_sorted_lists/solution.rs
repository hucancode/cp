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

use std::collections::BinaryHeap;
use std::cmp::Ordering;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // `root` and `q` own it's value
        // `current` borrows value
        let mut current: Option<&mut Box<ListNode>> = None;
        let mut root: Option<Box<ListNode>> = None;
        let mut q: BinaryHeap<Box<ListNode>> = BinaryHeap::new();
        // `q` now take the ownership of the nodes
        // `lists` can't access it's element anymore
        for node in lists.into_iter().filter_map(|x| x) {
            q.push(node);
        }
        // `x` now take the ownership of the node, and that's ok
        while let Some(mut x) = q.pop() {
            // `node` now take the ownership of the next node
            // `x` can't access it's `x.next` element anymore
            if let Some(node) = x.next.take() {
                q.push(node);
            }
            // since `current` is a borrower, the assignment doesn't
            // transfer ownership, `node` and `current` is still fine
            // `node.next` now own a new value, and `current` borrow that new value
            if let Some(mut node) = current {
                node.next = Some(x);
                current = node.next.as_mut();
            } else {
                // `root` now own a new value, and `current` borrow that value
                root = Some(x);
                current = root.as_mut();
            }
        }
        return root;
    }
}