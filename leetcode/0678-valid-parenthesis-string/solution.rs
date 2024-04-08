impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        use std::collections::VecDeque;
        let mut flex = VecDeque::new();
        let mut open = Vec::new();
        // matching strategy:
        // - see close bracket, use open bracket whenever possible
        //   + when there are no open bracket to match, any flex bracket can be used.
        //   + use left most flex bracket, save the right brackets for later use
        // - after matching all close bracket, use flex bracket to match open bracket
        //   + a valid flex bracket must appear after its corresponding open bracket
        for (i,c) in s.chars().enumerate() {
            match c {
                '(' => open.push(i),
                ')' => {
                    if !open.is_empty() {
                        open.pop();
                    } else if !flex.is_empty() {
                        flex.pop_front();
                    } else {
                        return false;
                    }
                },
                _ => flex.push_back(i),
            }
        }
        if flex.len() < open.len() {
            return false;
        }
        open.iter().rev().zip(flex.iter().rev()).all(|(a,b)| a < b)
    }
}
