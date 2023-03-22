impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let dict: Vec<(i32, &str)> = vec![
            (1000, "M"), 
            (900, "CM"), 
            (500, "D"), 
            (400, "CD"), 
            (100, "C"), 
            (90, "XC"), 
            (50, "L"), 
            (40, "XL"), 
            (10, "X"), 
            (9, "IX"), 
            (5, "V"), 
            (4, "IV"), 
            (1, "I")
        ];
        for (i, v) in dict {
            if(s.starts_with(v)) {
                return i + Solution::roman_to_int(s[v.len()..].to_string());
            }
        }
        return 0;
    }
}