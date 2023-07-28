impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut s1 = 0;
        let mut s2 = 0;
        let n = player1.len();
        for i in 0..n {
            let mut x = 1;
            if (i >= 1 && player1[i-1] == 10) || (i >= 2 && player1[i-2] == 10) {
                x = 2;
            } else {
                x = 1;
            }
            s1 += player1[i] * x;
            if (i >= 1 && player2[i-1] == 10) || (i >= 2 && player2[i-2] == 10) {
                x = 2;
            } else {
                x = 1;
            }
            s2 += player2[i] * x;
        }
        if s1 > s2 {
            return 1;
        }
        if s2 > s1 {
            return 2;
        }
        return 0;
    }
}