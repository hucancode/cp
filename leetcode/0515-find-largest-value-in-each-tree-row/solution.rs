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
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        let mut q = Vec::new();
        if let Some(node) = root {
            q.push(node);
        }
        while !q.is_empty() {
            let mut next = Vec::new();
            let mut k = i32::MIN;
            while let Some(node) = q.pop() {
                let mut node = node.borrow_mut();
                k = k.max(node.val);
                if let Some(node) = node.left.take() {
                    next.push(node);
                }
                if let Some(node) = node.right.take() {
                    next.push(node);
                }
            }
            ret.push(k);
            q = next;
        }
        ret
    }
}
