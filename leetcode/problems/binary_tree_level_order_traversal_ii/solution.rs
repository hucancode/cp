// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut q = VecDeque::new();
        if let Some(root) = root {
            q.push_back(root);
        }
        while !q.is_empty() {
            let mut vis = Vec::new();
            let mut next = VecDeque::new();
            while let Some(node) = q.pop_front() {
                let mut node = node.borrow_mut();
                vis.push(node.val);
                if let Some(left) = node.left.take() {
                    next.push_back(left);
                }
                if let Some(right) = node.right.take() {
                    next.push_back(right);
                }
            }
            ret.push(vis);
            q = next;
        }
        ret.into_iter().rev().collect()
    }
}