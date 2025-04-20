impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut count = vec![0;1001];
        for x in answers {
            count[1+x as usize] += 1;
        }
        count.into_iter()
            .enumerate()
            .skip(1)
            .map(|(total, answered)| (answered+total-1)/total*total)
            .sum::<usize>() as i32
    }
}
