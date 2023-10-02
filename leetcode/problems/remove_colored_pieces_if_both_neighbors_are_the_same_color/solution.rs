impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut ret = 0;
        for w in colors.as_bytes().windows(3) {
            if w.iter().all(|&c| c == 'A' as u8) {
                ret += 1;
            } else if w.iter().all(|&c| c == 'B' as u8) {
                ret -= 1;
            }
        }
        ret > 0
    }
}