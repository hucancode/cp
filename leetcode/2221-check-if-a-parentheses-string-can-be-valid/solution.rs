impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len()%2 != 0 {
            return false;
        }
        let mut open = Vec::new();
        let mut flex = Vec::new();
        //let t: String = s.chars().zip(locked.chars()).map(|(c,l)| if l == '0' {'_'} else {c}).collect();
        //println!("{t:?}");
        for (i, (c, l)) in s.chars().zip(locked.chars()).enumerate() {
            if l == '0' {
                flex.push(i);
            } else {
                if c == '(' {
                    open.push(i);
                } else if !open.is_empty() {
                    open.pop();
                } else if !flex.is_empty() {
                    flex.pop();
                } else {
                    println!("cannot match close bracket at {i}");
                    return false;
                }
            }
        }
        //println!("{open:?}, {flex:?}");
        if open.len() > flex.len() {
            println!("not enough flex to match open bracket");
            return false;
        }
        while !open.is_empty() && !flex.is_empty() {
            let a = open.pop().unwrap();
            let b = flex.pop().unwrap();
            if a > b {
                println!("cannot match open bracket at {a}");
                return false;
            }
        }
        return true;
    }
}
