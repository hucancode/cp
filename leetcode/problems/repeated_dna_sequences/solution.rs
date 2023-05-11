use std::collections::HashSet;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut appeared: HashSet<i32> = HashSet::new();
        let mut dup: HashSet<i32> = HashSet::new();
        let gene_to_int = |c: char| {
            match c {
                'A' => 0b00,
                'C' => 0b01,
                'G' => 0b10,
                _ => 0b11,
            }
        };
        let int_to_gene = |x: i32| {
            match x {
                0b00 => 'A',
                0b01 => 'C',
                0b10 => 'G',
                _ => 'T',
            }
        };
        let int_to_dna = |mut x: i32| {
            let mut ret = String::new();
            while ret.len() < 10 {
                ret.insert(0, int_to_gene(x & 0b11));
                x >>= 2;
            }
            ret
        };
        let mask = (1<<20) - 1;
        let seq = s.chars()
            .take(10)
            .fold(0, |acc, c| acc<<2 | gene_to_int(c));
        appeared.insert(seq);
        s.chars()
            .skip(10)
            .fold(seq, |acc, c| {
                let acc = (acc<<2 | gene_to_int(c)) & mask;
                if(appeared.contains(&acc)) {
                    dup.insert(acc);
                } else {
                    appeared.insert(acc);
                }
                acc
            });
        dup.into_iter()
            .map(int_to_dna)
            .collect()
    }
}