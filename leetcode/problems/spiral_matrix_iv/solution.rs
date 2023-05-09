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

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let m = m as usize;
        let mut ret = vec![vec![-1;n];m];
        let mut matrix: VecDeque<VecDeque<(usize, usize)>> = (0..m)
            .map(|r| {
                (0..n)
                    .map(|c| (r, c))
                    .collect()
            })
            .collect();
        while !matrix.is_empty() && head.is_some() {
            if let Some(q) = matrix.pop_front() {
                for (x,y) in q.into_iter() {
                    if let Some(node) = head {
                        ret[x][y] = node.val;
                        head = node.next;
                    }
                }
            }
            for arr in matrix.iter_mut() {
                if let Some((x, y)) = arr.pop_back() {
                    if let Some(node) = head {
                        ret[x][y] = node.val;
                        head = node.next;
                    }
                }
            }
            if let Some(q) = matrix.pop_back() {
                for (x,y) in q.into_iter().rev() {
                    if let Some(node) = head {
                        ret[x][y] = node.val;
                        head = node.next;
                    }
                }
            }
            for arr in matrix.iter_mut().rev() {
                if let Some((x, y)) = arr.pop_front() {
                    if let Some(node) = head {
                        ret[x][y] = node.val;
                        head = node.next;
                    }
                } 
            }
        }
        ret
    }
}