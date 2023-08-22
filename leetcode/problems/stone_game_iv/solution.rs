impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let squares: Vec<usize> = (1..).map(|x| x*x)
            .take_while(|&x| x <= n)
            .collect();
        let mut alice = vec![false;n+1];
        let mut bob = vec![true;n+1];
        for i in (0..n).rev() {
            alice[i] = squares.iter()
                .take_while(|&j| i+j <= n)
                .any(|j| bob[i+j]);
            bob[i] = squares.iter()
                .take_while(|&j| i+j <= n)
                .all(|j| alice[i+j]);
        }
        //println!("alice {alice:?} bob {bob:?}");
        return alice[0];
    }
}