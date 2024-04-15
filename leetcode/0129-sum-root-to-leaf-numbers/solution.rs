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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = Vec::new();
        q.push((root, 0));
        let mut ret = 0;
        while let Some((root, x)) = q.pop() {
            if let Some(mut root) = root {
                let mut root = root.borrow_mut();
                let x = x*10 + root.val;
                if root.left.is_none() && root.right.is_none() {
                    ret += x;
                    continue;
                }
                q.push((root.left.take(), x));
                q.push((root.right.take(), x));
            }
        }
        ret
    }
}
