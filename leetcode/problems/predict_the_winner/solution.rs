impl Solution {
    fn check(nums: &Vec<i32>, l: i32, r: i32, turn: bool, mut score1: i32, mut score2: i32) -> bool {
        if l > r {
            return score1 >= score2;
        }
        if turn {
            if turn == Self::check(nums, l+1, r, !turn, score1 + nums[l as usize], score2) {
                return turn;
            }
            if turn == Self::check(nums, l, r-1, !turn, score1 + nums[r as usize], score2) {
                return turn;
            }
        } else {
            if turn == Self::check(nums, l+1, r, !turn, score1, score2 + nums[l as usize]) {
                return turn;
            }
            if turn == Self::check(nums, l, r-1, !turn, score1, score2 + nums[r as usize]) {
                return turn;
            }
        }
        return !turn;
    }
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        Self::check(&nums, 0, nums.len() as i32 - 1, true, 0, 0)
    }
}