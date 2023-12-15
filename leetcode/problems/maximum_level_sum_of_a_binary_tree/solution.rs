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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let mut q = VecDeque::new();
        q.push_back(root);
        let mut max_sum = i32::MIN;
        let mut max_level = 1;
        let mut level = 0;
        while !q.is_empty() {
            level += 1;
            let mut sum = 0;
            let mut next = VecDeque::new();
            while let Some(node) = q.pop_front() {
                let mut node = node.borrow_mut();
                sum += node.val;
                if let Some(node) = node.left.take() {
                    next.push_back(node);
                }
                if let Some(node) = node.right.take() {
                    next.push_back(node);
                }
            }
            if sum > max_sum {
                max_sum = sum;
                max_level = level;
            }
            q = next;
        }
        max_level
    }
}