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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = 0;
        let mut q = Vec::new();
        q.push((root, false));
        while let Some((root, take)) = q.pop() {
            if let Some(mut node) = root {
                let mut node = node.borrow_mut();
                if node.left.is_none() && node.right.is_none() {
                    ret += if take { node.val } else { 0 };
                    continue;
                }
                q.push((node.left.take(), true));
                q.push((node.right.take(), false));
            }
        }
        return ret;
    }
}
