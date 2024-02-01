use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut out_adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut in_adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        for e in pairs.iter() {
            let u = e[0];
            let v = e[1];
            out_adj.entry(u).or_default().insert(v);
            in_adj.entry(v).or_default().insert(u);
        }
        //println!("in: {in_adj:?}, out: {out_adj:?}");
        let mut vis = Vec::new();
        let mut st = Vec::new();
        if let Some(&u) = out_adj.keys().find(|&u| {
            match (out_adj.get(u), in_adj.get(u)) {
                (Some(a), Some(b)) => a.len() > b.len(),
                (Some(a), None) => a.len() > 0,
                _ => false
            }
        }) {
            //println!("{u} as more out degree than in degree");
            st.push(u);
        } else {
            if let Some(&u) = out_adj.keys().last() {
                //println!("all vertex has balanced in-out degree, pick vertex {u}");
                st.push(u);
            }
        }
        while let Some(&u) = st.last() {
            let degree = out_adj.entry(u).or_default().len() + 
                in_adj.entry(u).or_default().len();
            if degree == 0 {
                st.pop();
                //println!("visit {u}");
                vis.push(u);
            } else {
                if let Some(&v) = out_adj.entry(u).or_default().iter().last() {
                    in_adj.entry(v).or_default().remove(&u);
                    out_adj.entry(u).or_default().remove(&v);
                    st.push(v);
                }
            }
        }
        vis.windows(2)
            .rev()
            .map(|a| vec![a[1], a[0]])
            .collect()
    }
}