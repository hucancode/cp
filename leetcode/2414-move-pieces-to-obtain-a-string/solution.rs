impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let start = start.chars()
            .enumerate()
            .filter(|&(i, c)| c != '_');
        let target = target.chars()
            .enumerate()
            .filter(|&(i,c)| c != '_');
        if start.clone().count() != target.clone().count() {
            return false;
        }
        start.zip(target)
            .all(|((i, a), (j, b))| a == b && if a == 'L' { i >= j } else { i <= j })

    }
}
