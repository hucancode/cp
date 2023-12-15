use std::collections::HashSet;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut incoming_nodes = HashSet::new();
        let mut outgoing_nodes = HashSet::new();
        for mut uv in paths {
            if let Some(to) = uv.pop() {
                incoming_nodes.insert(to);
            }
            if let Some(from) = uv.pop() {
                outgoing_nodes.insert(from);
            }
        }
        for node in incoming_nodes
            .difference(&outgoing_nodes) {
            return node.to_string();
        }
        "".to_string()
    }
}