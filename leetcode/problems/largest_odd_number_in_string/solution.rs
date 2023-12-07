impl Solution {
    pub fn largest_odd_number(mut num: String) -> String {
        let odd = vec!['1','3','5','7','9'];
        while let Some(x) = num.chars().last() {
            if odd.iter().any(|&y| x == y) {
                break;  
            }
            num.pop();
        }
        num
    }
}