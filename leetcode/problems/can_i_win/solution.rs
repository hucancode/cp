use std::collections::HashMap;

impl Solution {
    fn check(f: &mut HashMap<(bool, i32), bool>, my_turn: bool, n: i32, mask: i32, remaining: i32) -> bool {
        let key = (my_turn, mask);
        if let Some(ret) = f.get(&key) {
            return *ret;
        }
        let can_win = (1..=n)
            .filter(|&i| (mask & (1<<i)) == 0)
            .any(|i| i >= remaining || 
                my_turn == Self::check(f, !my_turn, n, mask | (1<<i), remaining - i));
        //println!("my turn = {my_turn}, can win = {can_win}, remaining = {remaining}, mask = {mask:b}");
        let ret = if can_win {my_turn} else {!my_turn};
        f.insert(key, ret);
        ret
    }
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if (1..=max_choosable_integer).sum::<i32>() < desired_total {
            return false;
        }
        let mut f: HashMap<(bool, i32), bool> = HashMap::new();
        Self::check(&mut f, true, max_choosable_integer, 0, desired_total)
    }
}