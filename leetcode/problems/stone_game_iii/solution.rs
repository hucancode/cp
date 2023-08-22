use std::cmp::min;
use std::cmp::max;
impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let all:i32 = stone_value.iter().sum();
        let mut alice = vec![i32::MIN;n+1]; // alice[i] = skip i stone, alice is playing, what is the maxinum score for alice
        let mut bob = vec![i32::MAX;n+1]; // bob[i] = skip i stone, bob is playing, what is the maxinum score for alice
        alice[n] = 0;
        bob[n] = 0;
        for i in (0..n).rev() {
            let mut x = 0;
            for j in 0..min(n-i,3) {
                x += stone_value[i+j];
                alice[i] = max(alice[i], x + bob[i+j+1]);
                bob[i] = min(bob[i], alice[i+j+1])
            }
        }
        //println!("alice = {alice:?}, bob = {bob:?}");
        let a = alice[0];
        let b = all - a;
        if a > b {
            return String::from("Alice");
        } else if a == b {
            return String::from("Tie");
        }
        return String::from("Bob");
    }
}