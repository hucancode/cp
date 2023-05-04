impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut senate: Vec<char> = senate.chars().collect();
        let mut vr = 0;
        let mut vd = 0;
        while senate
            .windows(2)
            .any(|w| w[0] != w[1]) {
            let mut next = Vec::new();
            for &c in senate.iter() {
                if c == 'R' {
                    if vd > 0 {
                        vd -= 1;
                    } else {
                        next.push(c);
                        vr += 1;
                    }
                } else {
                    if vr > 0 {
                        vr -= 1;
                    } else {
                        next.push(c);
                        vd += 1;
                    }
                }
            }
            senate = next;
        }
        match senate.first() {
            Some('R') => String::from("Radiant"),
            _ => String::from("Dire"),
        }
    }
}