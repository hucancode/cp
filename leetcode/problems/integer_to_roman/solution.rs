impl Solution {
    pub fn int_to_roman(num: i32) -> String {
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
        for (i, s) in dict {
            if(num >= i) {
                return s.to_owned()+&Solution::int_to_roman(num-i);
            }
        }
        return String::from("");
    }
}