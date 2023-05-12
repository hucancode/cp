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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut rabbit = head.clone(); // clone
        let mut turtle = head.as_mut(); // borrow mut
        for i in 0..n {
            if let Some(node) = rabbit {
                rabbit = node.next;
            }
        }
        if let Some(node) = rabbit {
            rabbit = node.next;
        } else {
            if let Some(node) = head {
                return node.next;
            }
            return None;
        }
        while rabbit.is_some() {
            if let Some(node) = rabbit {
                rabbit = node.next;
            }
            if let Some(node) = turtle {
                turtle = node.next.as_mut();
            }
        }
        if let Some(mut a) = turtle {
            if let Some(mut b) = a.next.take() {
                if let Some(c) = b.next.take() {
                    a.next.replace(c);
                }
            }
        }
        head
    }
}