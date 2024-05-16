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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut v = Vec::new();
        let mut q = Vec::new();
        if let Some(node) = root {
            q.push(node);
        }
        while let Some(node) = q.pop() {
            let mut node = node.borrow_mut();
            v.push(node.val);
            if let Some(left) = node.left.take() {
                q.push(left);
            }
            if let Some(right) = node.right.take() {
                q.push(right);
            }
            while v.iter()
                .rev()
                .take(2)
                .filter(|&&x| x < 2)
                .count() == 2 {
                let x = v.pop().unwrap();
                let y = v.pop().unwrap();
                let o = v.pop().unwrap();
                let z = match o {
                    2 => x|y,
                    _ => x&y,
                };
                //println!("evaluate {x} {o} {y} = {z}");
                v.push(z);
                //println!("stack = {v:?}");
            }

        }
        v.pop().is_some_and(|x| x == 1)
    }
}
