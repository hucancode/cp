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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut q = vec![root];
        let mut ret = Vec::new();
        let mut current = Vec::new();
        let smaller = |a: &Vec<i32>, b: &Vec<i32>| {
            if let Some((i,j)) = a.iter().rev()
                .zip(b.iter().rev())
                .skip_while(|(i,j)| i == j)
                .next() {
                i < j
            } else {
                a.len() < b.len()
            }
        };
        while let Some(item) = q.pop() {
            match item {
                None => {
                    current.pop();
                },
                Some(node) => {
                    let mut node = node.borrow_mut();
                    current.push(node.val);
                    q.push(None);
                    if(node.left.is_none() && node.right.is_none()) {
                        if ret.is_empty() || smaller(&current, &ret) {
                            ret = current.clone(); 
                        }
                    }
                    if node.left.is_some() {
                        q.push(node.left.take());
                    }
                    if node.right.is_some() {
                        q.push(node.right.take());
                    }
                }
            }
        }
        ret.into_iter()
            .rev()
            .map(|x| (x as u8 + 'a' as u8) as char)
            .collect()
    }
}
