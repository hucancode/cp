impl Solution {
    pub fn smallest_sufficient_team(mut req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        req_skills.sort();
        let people: Vec<i32> = people.into_iter()
            .map(|skills| skills.iter().fold(0, 
                |acc, x| match req_skills.binary_search(x) {
                    Ok(i) => acc | 1<<i,
                    _ => acc,
                }))
            .collect();
        let m = 1<<req_skills.len();
        let n = people.len();
        let full_team: Vec<i32> = (0..n).map(|x| x as i32).collect();
        let mut f = vec![full_team.clone(); m];
        f[0] = Vec::new();
        for mask in 0..m {
            for i in 0..n {
                let next = mask | (people[i] as usize);
                if f[next].len() <= f[mask].len()+1 {
                    continue;
                }
                let team = f[mask].clone();
                f[next] = team;
                f[next].push(i as i32)
            }
        }
        return f[m-1].clone();
    }
}