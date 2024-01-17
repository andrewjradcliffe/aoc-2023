fn combinations_inner(v: &mut Vec<Vec<usize>>, n: usize, k: usize, len: usize) {
    if len < k {
        let mut tmp = Vec::new();
        for src in v.iter() {
            let rhs = src[len - 1];
            for e in rhs + 1..n {
                let mut new = src.clone();
                new.push(e);
                tmp.push(new);
            }
        }
        v.clear();
        v.append(&mut tmp);
        combinations_inner(v, n, k, len + 1);
    }
}
pub fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    assert!(n >= k);
    let mut v = Vec::new();
    if k != 0 {
        let last = n - k + 1;
        for i in 0..last {
            v.push(vec![i]);
        }
        combinations_inner(&mut v, n, k, 1);
    } else {
        v.push(vec![]);
    }
    v
}
/// Works for `n <= 62` and any `k`.
pub fn binomial(n: u64, k: u64) -> u64 {
    if n < k {
        0
    } else {
        let mut m = n - k;
        let mut p = 1;
        let mut i: u64 = 0;
        while i < k {
            i += 1;
            m += 1;
            p = m * p / i;
        }
        p
    }
    // let mut m = n.wrapping_sub(k);
    // let mut p = 1;
    // let mut i: u64 = 0;
    // while i < k {
    //     i = i.wrapping_add(1);
    //     m = m.wrapping_add(1);
    //     p = m.wrapping_mul(p).wrapping_div(i);
    // }
    // p
}

/*
Variable-base positional number system representation.
*/
#[derive(Debug, Clone)]
pub struct Combinations {
    n: usize,
    k: usize,
    pub(crate) digits: Vec<usize>,
    initial: bool,
}
impl Combinations {
    /// Panics if `k > n`.
    pub fn new(n: usize, k: usize) -> Self {
        assert!(k <= n);
        let mut digits = Vec::with_capacity(k);
        for i in 0..k {
            digits.push(i);
        }
        Self {
            n,
            k,
            digits,
            initial: true,
        }
    }
    pub fn is_done(&self) -> bool {
        self.k == 0 || self.n == 0 || self.digits[0] > self.n - self.k
    }
    /*
    This will never overflow as the maximum value of `n` is `usize::MAX`, and
    the value at the last index is at most `n - 1`, hence, incrementing it
    by 1 will never result in wrapping around to `0`.
    N.B. the maximum value of `len + j` is `n - k + 1 + (k - 1) = n`.
     */
    pub fn next_combination_mut(&mut self) {
        if self.is_done() {
            return ();
        }
        let mut j = self.k - 1;
        let len = self.n - self.k + 1;
        self.digits[j] += 1;
        while self.digits[j] == len + j && j != 0 {
            j -= 1;
            self.digits[j] += 1;
        }
        j += 1;
        while j < self.k {
            self.digits[j] = self.digits[j - 1] + 1;
            j += 1;
        }
    }
    pub fn next_combination(&mut self) -> Option<Vec<usize>> {
        if self.initial {
            self.initial = false;
            Some(self.digits.clone())
        } else if self.is_done() {
            None
        } else {
            self.next_combination_mut();
            if self.is_done() {
                None
            } else {
                Some(self.digits.clone())
            }
        }
    }
    pub fn reset(&mut self) {
        self.initial = true;
        // self.digits.iter_mut().enumerate().for_each(|(i, v)| *v = i);
        for i in 0..self.k {
            self.digits[i] = i;
        }
    }
    pub fn count_remaining(&self) -> usize {
        if self.k == 0 {
            if self.initial {
                1
            } else {
                0
            }
        } else if self.k == 1 {
            self.n - self.digits[0]
        } else {
            let mut s: usize = 0;
            let mut j: usize = self.k - 1;
            let mut i: usize = 1;
            let e = self.digits[j] - self.digits[j - 1] - 1;
            j -= 1;
            let mut d_i = self.digits[j];
            let last = self.k - 1;
            while i < last {
                let d_i_1 = self.digits[j - 1];
                if d_i_1 != d_i {
                    s += binomial((self.n - d_i) as u64, (i + 1) as u64) as usize;
                    d_i = d_i_1 + 1;
                } else {
                    d_i = d_i_1;
                }
                j -= 1;
                i += 1;
            }
            s + binomial((self.n - d_i) as u64, self.k as u64) as usize - e
        }
    }
    pub fn linear_index(&self) -> usize {
        if self.k == 0 {
            0
        } else if self.k == 1 {
            self.digits[0]
        } else {
            let mut s: usize = 0;
            let mut j: usize = self.k - 1;
            let mut i: usize = 1;
            let e = self.digits[j] - self.digits[j - 1] - 1;
            j -= 1;
            let mut d_i = self.digits[j];
            let last = self.k - 1;
            while i < last {
                let d_i_1 = self.digits[j - 1];
                if d_i_1 != d_i {
                    s += binomial((self.n - 1 - d_i_1) as u64, (i + 1) as u64) as usize
                        - binomial((self.n - d_i) as u64, (i + 1) as u64) as usize;
                }
                d_i = d_i_1;
                j -= 1;
                i += 1;
            }
            s + binomial(self.n as u64, self.k as u64) as usize
                - binomial((self.n - self.digits[0]) as u64, self.k as u64) as usize
                + e
            // let j = self.k - 1;
            // let e = self.digits[j] - self.digits[j - 1] - 1;
            // let mut d_i = self.digits[j - 1];
            // let s: usize = self.digits[..j - 1]
            //     .iter()
            //     .rev()
            //     .zip(1..)
            //     .map(|(d_i_1, i)| {
            //         let x = if *d_i_1 != d_i {
            //             binomial((self.n - 1 - *d_i_1) as u64, i + 1) as usize
            //                 - binomial((self.n - d_i) as u64, i + 1) as usize
            //         } else {
            //             0
            //         };
            //         d_i = *d_i_1;
            //         x
            //     })
            //     .sum();
            // s + binomial(self.n as u64, self.k as u64) as usize
            //     - binomial((self.n - self.digits[0]) as u64, self.k as u64) as usize
            //     + e
        }
    }
}
impl Iterator for Combinations {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_combination()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binomial_works() {
        assert_eq!(binomial(0, 0), 1);
        assert_eq!(binomial(3, 4), 0);
        assert_eq!(binomial(7, 17), 0);
        assert_eq!(binomial(5, 0), 1);
        assert_eq!(binomial(7, 3), 35);
        assert_eq!(binomial(50, 11), 37353738800);
        assert_eq!(binomial(61, 30), 232714176627630544);
        assert_eq!(binomial(62, 31), 465428353255261088);
    }

    #[test]
    fn combinations_works() {
        let x = combinations(0, 0);
        assert_eq!(x.len(), 1);
        let x = combinations(3, 0);
        assert_eq!(x.len(), 1);
        let x = combinations(3, 2);
        assert_eq!(x.len(), 3);
        let x = combinations(4, 2);
        assert_eq!(x.len(), 6);
        let x = combinations(5, 3);
        assert_eq!(x.len(), 10);
        let x = combinations(10, 7);
        assert_eq!(x.len(), 120);
    }
    #[test]
    fn combination_iter_works() {
        for (n, k) in [
            (0, 0),
            (3, 0),
            (2, 1),
            (3, 2),
            (4, 1),
            (4, 2),
            (4, 4),
            (5, 3),
            (7, 3),
            (7, 4),
            (10, 7),
        ] {
            let mut x = combinations(n, k);
            x.sort_unstable();
            let comb = Combinations::new(n, k);
            let mut y: Vec<_> = comb.collect();
            y.sort_unstable();
            assert_eq!(x, y);
        }
    }
    #[test]
    fn count_remaining() {
        let mut x = Combinations::new(7, 4);
        // Many specific checks
        assert_eq!(x.count_remaining(), 35);
        for _ in 0..3 {
            x.next_combination_mut();
        }
        assert_eq!(x.count_remaining(), 32);
        x.next_combination_mut();
        assert_eq!(x.count_remaining(), 31);
        x.next_combination_mut();
        x.next_combination_mut();
        assert_eq!(x.count_remaining(), 29);
        x.reset();
        for _ in 0..8 {
            x.next_combination_mut();
        }
        assert_eq!(x.count_remaining(), 27);
        assert_eq!(x.digits, vec![0, 1, 4, 6]);
        x.next_combination_mut();
        assert_eq!(x.count_remaining(), 26);
        assert_eq!(x.digits, vec![0, 1, 5, 6]);
        x.next_combination_mut();
        assert_eq!(x.count_remaining(), 25);
        assert_eq!(x.digits, vec![0, 2, 3, 4]);
        x.reset();
        for _ in 0..14 {
            x.next_combination_mut();
        }
        assert_eq!(x.count_remaining(), 21);
        assert_eq!(x.digits, vec![0, 2, 4, 6]);
        x.reset();
        for _ in 0..17 {
            x.next_combination_mut();
        }
        assert_eq!(x.count_remaining(), 18);
        assert_eq!(x.digits, vec![0, 3, 4, 6]);
        x.reset();
        for _ in 0..27 {
            x.next_combination_mut();
        }
        assert_eq!(x.count_remaining(), 8);
        assert_eq!(x.digits, vec![1, 3, 4, 6]);
        // Then, sweep the whole set
        for n in 0..35 {
            x.reset();
            for _ in 0..n {
                x.next_combination_mut();
            }
            assert_eq!(x.count_remaining(), 35 - n)
        }
        x.reset();
        for _ in 0..36 {
            x.next_combination_mut();
        }
        assert_eq!(x.count_remaining(), 0);
        assert_eq!(x.digits, vec![4, 5, 6, 7]);
    }
    #[test]
    fn linear_index() {
        let mut x = Combinations::new(7, 4);
        for n in 0..36 {
            x.reset();
            for _ in 0..n {
                x.next_combination_mut();
            }
            assert_eq!(x.linear_index(), n);
        }
    }
}
