impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let cost = |bit| (1<<(bit+1)) - 1;
        let mut f = vec![0; 31];
        let mut g = vec![0; 31];
        f[0] = n&1;
        g[0] = (n&1)^1;
        for i in 1..31 {
            let set = (n&(1<<i)) != 0;
            if(set) {
                g[i] = f[i-1];
                f[i] = g[i-1];// cost to make 1xxxx -> 11000
				f[i] += 1;// cost to make 11000 -> 1000
                f[i] += cost(i-1);// cost to make 1000 to become 0000
            } else {
                f[i] = f[i-1];
                g[i] = g[i-1];// cost to make 0xxxx -> 01000
				g[i] += 1;// cost to make 01000 -> 11000
                g[i] += cost(i-1);// cost to make 1000 to become 0000
            }
        }
        return f[30];
    }
}