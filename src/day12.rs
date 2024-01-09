use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::NonZeroUsize;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition {
    Damaged,
    Operational,
    Unknown,
}
use Condition::*;
impl TryFrom<char> for Condition {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Damaged),
            '.' => Ok(Operational),
            '?' => Ok(Unknown),
            _ => Err(c.to_string()),
        }
    }
}
impl Condition {
    pub fn is_damaged(&self) -> bool {
        *self == Damaged
    }
    pub fn is_operational(&self) -> bool {
        *self == Operational
    }
    pub fn is_unknown(&self) -> bool {
        *self == Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    left: Vec<Condition>,
    right: Vec<usize>,
}

impl FromStr for Row {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((lhs, rhs)) = s.split_once(' ') {
            let mut left = Vec::new();
            for c in lhs.chars() {
                left.push(Condition::try_from(c)?);
            }
            let mut right = Vec::new();
            for num in rhs.split(',') {
                right.push(num.parse::<usize>().map_err(|e| e.to_string())?);
            }
            Ok(Self { left, right })
        } else {
            Err(s.to_string())
        }
    }
}

impl Row {
    pub fn is_feasible(&self) -> bool {
        // let mut contig = Vec::with_capacity(self.right.len());
        let mut contig_iter = self.right.iter();
        let mut iter = self.left.iter().enumerate();
        while let Some((i, cond)) = iter.next() {
            match cond {
                Damaged => {
                    let left = i;
                    let mut right = i + 1;
                    while let Some((_, cond)) = iter.next() {
                        match cond {
                            Damaged => {
                                right += 1;
                            }
                            Operational => {
                                break;
                            }
                            Unknown => return false,
                        }
                    }
                    // Early exit, and avoid allocation
                    if let Some(size) = contig_iter.next() {
                        if right - left != *size {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                Operational => (),
                Unknown => return false,
            }
        }
        true
        // contig == self.right
    }
    pub fn unfold(&self, m: NonZeroUsize) -> Self {
        let m = m.get();
        let n = self.right.len();
        let mut right = Vec::with_capacity(m * n);
        for _ in 0..m {
            for item in self.right.iter() {
                right.push(item.clone());
            }
        }
        let n = self.left.len();
        let mut left = Vec::with_capacity(m * n + m - 1);
        for _ in 0..m - 1 {
            for item in self.left.iter() {
                left.push(item.clone());
            }
            left.push(Unknown);
        }
        for item in self.left.iter() {
            left.push(item.clone());
        }
        Self { left, right }
    }
    pub fn count_condition(&self, cond: Condition) -> usize {
        match cond {
            Damaged => self.left.iter().filter(|cond| cond.is_damaged()).count(),
            Operational => self
                .left
                .iter()
                .filter(|cond| cond.is_operational())
                .count(),
            Unknown => self.left.iter().filter(|cond| cond.is_unknown()).count(),
        }
    }
    pub fn n_damaged(&self) -> usize {
        self.right.iter().sum()
    }
    pub fn starts_with(&self, cond: Condition) -> bool {
        let n = self.left.len();
        if n == 0 {
            false
        } else {
            self.left[0] == cond
        }
    }
    pub fn ends_with(&self, cond: Condition) -> bool {
        let n = self.left.len();
        if n == 0 {
            false
        } else {
            self.left[n - 1] == cond
        }
    }
    // pub fn trim_contiguous_end(&self) -> Option<Self> {
    //     if self.ends_with(Damaged) {
    //         let n = self.left.len();
    //         let m = self.right.len();
    //         if m != 0 {
    //             let contig = self.right[m - 1].clone();
    //             let a = &self.left[n - contig..n];
    //             if a.into_iter().all(|x| x.is_damaged()) {
    //                 let left = self.left[0..n - contig].to_vec();
    //                 let right = self.right[0..m - 1].to_vec();
    //                 Some(Self { left, right })
    //             } else {
    //                 None
    //             }
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
    // pub fn has_contiguous_end(&self) -> bool {
    //     if self.ends_with(Damaged) {
    //         let n = self.left.len();
    //         let m = self.right.len();
    //         if m != 0 {
    //             let contig = self.right[m - 1].clone();
    //             let a = &self.left[n - contig..n];
    //             a.into_iter().all(|x| x.is_damaged())
    //         } else {
    //             false
    //         }
    //     } else {
    //         false
    //     }
    // }
    pub fn count_damaged_front(&self) -> usize {
        self.left.iter().take_while(|x| **x == Damaged).count()
    }
    pub fn count_damaged_back(&self) -> usize {
        self.left
            .iter()
            .rev()
            .take_while(|x| **x == Damaged)
            .count()
    }
}
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

/*
Variable-base positional number system representation.
*/
#[derive(Debug, Clone)]
pub struct Combinations {
    n: usize,
    k: usize,
    inner: Vec<usize>,
    initial: bool,
}
impl Combinations {
    /// Panics if `k > n`.
    pub fn new(n: usize, k: usize) -> Self {
        assert!(k <= n);
        let mut inner = Vec::with_capacity(k);
        for i in 0..k {
            inner.push(i);
        }
        Self {
            n,
            k,
            inner,
            initial: true,
        }
    }
    pub fn is_done(&self) -> bool {
        self.k == 0 || self.n == 0 || self.inner[0] > self.n - self.k
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
        self.inner[j] += 1;
        while self.inner[j] == len + j && j != 0 {
            j -= 1;
            self.inner[j] += 1;
        }
        j += 1;
        while j < self.k {
            self.inner[j] = self.inner[j - 1] + 1;
            j += 1;
        }
    }
    pub fn next_combination(&mut self) -> Option<Vec<usize>> {
        if self.initial {
            self.initial = false;
            Some(self.inner.clone())
        } else if self.is_done() {
            None
        } else {
            self.next_combination_mut();
            if self.is_done() {
                None
            } else {
                Some(self.inner.clone())
            }
        }
    }
    pub fn reset(&mut self) {
        self.initial = true;
        // self.inner.iter_mut().enumerate().for_each(|(i, v)| *v = i);
        for i in 0..self.k {
            self.inner[i] = i;
        }
    }
}
impl Iterator for Combinations {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_combination()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowAnalyzer {
    row: Row,
    // n_unknown: usize,
    // n_damaged: usize,
    k_damaged: usize,
    unknowns: Vec<usize>,
}
impl From<Row> for RowAnalyzer {
    fn from(row: Row) -> Self {
        let n_damaged = row.right.iter().sum::<usize>();
        let unknowns: Vec<_> = row
            .left
            .iter()
            .enumerate()
            .filter(|(_, cond)| cond.is_unknown())
            .map(|(i, _)| i)
            .collect();
        let k_damaged = n_damaged - row.left.iter().filter(|cond| cond.is_damaged()).count();
        // let n_unknown = unknowns.len();
        Self {
            row,
            // n_unknown,
            // n_damaged,
            k_damaged,
            unknowns,
        }
    }
}

impl RowAnalyzer {
    pub fn count_arrangements(&mut self) -> usize {
        let n_unknown = self.unknowns.len();
        let mut comb = Combinations::new(n_unknown, self.k_damaged);
        let mut sum: usize = 0;
        while !comb.is_done() {
            // Set base state
            for i in self.unknowns.iter() {
                self.row.left[*i] = Operational;
            }
            // Set flag
            for i in comb.inner.iter() {
                let idx = self.unknowns[*i];
                self.row.left[idx] = Damaged;
            }
            sum += self.row.is_feasible() as usize;
            comb.next_combination_mut();
        }
        // Reset to original state
        for i in self.unknowns.iter() {
            self.row.left[*i] = Unknown;
        }
        if sum == 0 {
            1
        } else {
            sum
        }
    }

    // Too high
    // pub fn count_arrangements_with_unfold(&mut self) -> usize {
    //     let mut row = self.row.clone();
    //     row.left.push(Unknown);
    //     let mut tmp = RowAnalyzer::from(row);
    //     let first = tmp.count_arrangements();
    //     let mut row = tmp.row;
    //     row.left.insert(0, Unknown);
    //     let end = self.row.count_damaged_back() != 0;
    //     if end {
    //         row.left.insert(0, Damaged);
    //         row.right.insert(0, 1);
    //     }
    //     let start = self.row.count_damaged_front() != 0;
    //     if start {
    //         row.left.push(Damaged);
    //         row.right.push(1);
    //     }
    //     let mut tmp = RowAnalyzer::from(row);
    //     let mid = tmp.count_arrangements();
    //     let mut row = tmp.row;
    //     if start {
    //         row.left.pop();
    //         row.right.pop();
    //     }
    //     row.left.pop();
    //     let mut tmp = RowAnalyzer::from(row);
    //     let last = tmp.count_arrangements();
    //     first * mid * mid * mid * last
    // }

    // More nuanced attempt
    pub fn count_arrangements_with_unfold(&mut self) -> usize {
        let mut row = self.row.clone();
        let m = row.right.len();
        row.left.push(Unknown);
        let mut tmp = RowAnalyzer::from(row);
        let mut first = tmp.count_arrangements();
        let mut row = tmp.row;
        row.left.insert(0, Unknown);
        let has = self.row.count_damaged_back();
        if has != 0 {
            let need = self.row.right[m - 1];
            let size = need - has;
            if size != 0 {
                for _ in 0..has {
                    row.left.insert(0, Damaged);
                }
                for _ in 0..size {
                    row.left.insert(0, Unknown);
                }
                row.right.insert(0, need);
                let mut r = self.row.clone();
                for _ in 0..need {
                    r.left.pop();
                }
                r.right.pop();
                let mut tmp = RowAnalyzer::from(r);
                first = tmp.count_arrangements();
            } else {
                row.left.insert(0, Damaged);
                row.right.insert(0, 1);
            }
        }
        row.left.pop();
        let mut tmp = RowAnalyzer::from(row);
        let mut last = tmp.count_arrangements();
        let mut row = tmp.row;
        row.left.push(Unknown);
        let has = self.row.count_damaged_front();
        if has != 0 {
            let need = self.row.right[0];
            let size = need - has;
            if size != 0 {
                for _ in 0..has {
                    row.left.push(Damaged);
                }
                for _ in 0..size {
                    row.left.push(Unknown);
                }
                row.right.push(need);
                let mut r = self.row.clone();
                for _ in 0..need {
                    r.left.remove(0);
                }
                r.right.remove(0);
                let mut tmp = RowAnalyzer::from(r);
                last = tmp.count_arrangements();
            } else {
                row.left.push(Damaged);
                row.right.push(1);
            }
        }
        let mut tmp = RowAnalyzer::from(row);
        let mid = tmp.count_arrangements();
        first * mid * mid * mid * last
    }
}
impl FromStr for RowAnalyzer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s.parse::<Row>()?;
        Ok(Self::from(row))
    }
}

pub fn rows_from_path<T: AsRef<Path>>(path: T) -> Result<Vec<Row>, String> {
    let f = File::open(path).map_err(|e| e.to_string())?;
    let mut f = BufReader::new(f);
    let mut s = String::with_capacity(256);
    let mut v = Vec::new();
    while f.read_line(&mut s).map_err(|e| e.to_string())? != 0 {
        s.pop();
        v.push(s.parse::<Row>()?);
        s.clear();
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let s = "???.### 1,1,3";
        let lhs = s.parse::<Row>().unwrap();
        let rhs = Row {
            left: vec![
                Unknown,
                Unknown,
                Unknown,
                Operational,
                Damaged,
                Damaged,
                Damaged,
            ],
            right: vec![1, 1, 3],
        };
        assert_eq!(lhs, rhs);
    }
    #[test]
    fn is_feasible() {
        let s = "#.#.### 1,1,3";
        let lhs = s.parse::<Row>().unwrap();
        assert!(lhs.is_feasible());

        let s = ".#...#....###. 1,1,3";
        assert!(s.parse::<Row>().unwrap().is_feasible());
        let s = "..#...#...###. 1,1,3";
        assert!(s.parse::<Row>().unwrap().is_feasible());
        let s = "..#..#....###. 1,1,3";
        assert!(s.parse::<Row>().unwrap().is_feasible());
        let s = ".#...#....###. 1,1,3";
        assert!(s.parse::<Row>().unwrap().is_feasible());

        let s = "....##....###. 1,1,3";
        assert!(!s.parse::<Row>().unwrap().is_feasible());
        let s = ".##.......###. 1,1,3";
        assert!(!s.parse::<Row>().unwrap().is_feasible());
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
    fn count_arrangements() {
        let s = "???.### 1,1,3";
        let x = s.parse::<Row>().unwrap();
        let mut x = RowAnalyzer::from(x);
        assert_eq!(x.count_arrangements(), 1);

        let s = ".??..??...?##. 1,1,3";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 4);

        let s = "?#?#?#?#?#?#?#? 1,3,1,6";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 1);

        let s = "????.#...#... 4,1,1";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 1);

        let s = "????.######..#####. 1,6,5";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 4);

        let s = "?###???????? 3,2,1";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 10);

        let s = "?###??????????###???????? 3,2,1,3,2,1";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 150);

        // let s = "?###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1";
        // let mut x = s.parse::<RowAnalyzer>().unwrap();
        // assert_eq!(x.count_arrangements(), 2250, "{:#?}", x);
    }
    #[test]
    fn count_arrangements_with_unfold() {
        let s = "???.### 1,1,3";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements_with_unfold(), 1);

        let s = ".??..??...?##. 1,1,3";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements_with_unfold(), 16384);

        let s = "?#?#?#?#?#?#?#? 1,3,1,6";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements_with_unfold(), 1);

        let s = "????.#...#... 4,1,1";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements_with_unfold(), 16);

        let s = "????.######..#####. 1,6,5";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements_with_unfold(), 2500);

        let s = "?###???????? 3,2,1";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements_with_unfold(), 506250);
    }
}
