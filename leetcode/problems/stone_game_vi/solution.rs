impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut v = alice_values.into_iter()
            .zip(bob_values.into_iter())
            .map(|(a,b)| (a+b, a, b))
            .collect::<Vec<(i32, i32, i32)>>();
        v.sort();
        let mut alice = 0;
        let mut bob = 0;
        for (i, (_,a,b)) in v.into_iter().rev().enumerate() {
            if i%2 == 0 {
                alice += a;
            } else {
                bob += b;
            }
        }
        if alice > bob {1}
        else if alice == bob {0}
        else {-1}
    }
}