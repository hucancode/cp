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
        let mut ret = -1;
        if let Some(root) = root {
            let mut q = VecDeque::new();
            q.push_back(root);
            let mut max = i32::MIN;
            let mut level = 0;
            while !q.is_empty() {
                let n = q.len();
                level += 1;
                let mut score = 0;
                for _ in 0..n {
                    if let Some(node) = q.pop_front() {
                        score += node.borrow().val;
                        if let Some(node) = node.borrow().left.clone() {
                            q.push_back(node);
                        }
                        if let Some(node) = node.borrow().right.clone() {
                            q.push_back(node);
                        }
                    }
                }
                if score > max {
                    max = score;
                    ret = level;
                }
            }
        }
        ret
    }
}