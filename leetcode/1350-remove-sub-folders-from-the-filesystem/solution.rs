use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<String, Trie>,
    is_leaf: bool,
}

impl Trie {
    pub fn insert(&mut self, value: &str) -> bool {
        if self.is_leaf {
            return false;
        }
        if let Some((value, rest)) = value.split_once('/') {
            return self.children
                .entry(value.to_string())
                .or_default()
                .insert(rest);
        } else {
            self.children
                .entry(value.to_string())
                .or_default()
                .is_leaf = true;
            return true;
        }
    }
}

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_by(|a,b| a.len().cmp(&b.len()));
        let mut ret = Vec::new();
        let mut root = Trie::default();
        for s in folder {
            if root.insert(&s) {
                ret.push(s);
            }
        }
        return ret;
    }
}
