use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

#[derive(Debug, Clone)]
pub struct Combinations {
    n: usize,
    k: usize,
    inner: Vec<usize>,
    initial: bool,
}
impl Combinations {
    pub fn new(n: usize, k: usize) -> Self {
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
    pub fn next_combination_mut(&mut self) {
        if self.is_done() {
            return ();
        }
        let mut i = self.k - 1;
        let mut offset = 1;
        self.inner[i] += 1;
        while self.inner[i] > self.n - offset && i != 0 {
            self.inner[i] = 0;
            offset += 1;
            i -= 1;
            self.inner[i] += 1;
        }
        i += 1;
        while i < self.k {
            self.inner[i] = self.inner[i - 1] + 1;
            i += 1;
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
        sum
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
        assert_eq!(x.count_arrangements(), 4, "{:#?}", x);

        let s = "?#?#?#?#?#?#?#? 1,3,1,6";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 1, "{:#?}", x);

        let s = "????.#...#... 4,1,1";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 1, "{:#?}", x);

        let s = "????.######..#####. 1,6,5";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 4, "{:#?}", x);

        let s = "?###???????? 3,2,1";
        let mut x = s.parse::<RowAnalyzer>().unwrap();
        assert_eq!(x.count_arrangements(), 10, "{:#?}", x);
    }
}
