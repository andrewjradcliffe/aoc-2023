use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    value: u32,
    pos: Range<usize>,
}
impl Number {
    pub fn new(value: u32, pos: Range<usize>) -> Self {
        Self { value, pos }
    }

    /// Other row means immediately above or below, i.e. if the row index is `i`, then either `i - 1` or `i + 1`
    pub fn is_adjacent_other_row(&self, j: usize) -> bool {
        self.pos.start == j + 1 || self.pos.end == j || self.pos.contains(&j)
        // Alt 1
        // self.pos.start <= j + 1 && j < self.pos.end + 1
        // Alt 2
        // let inc = (self.pos.start != 0) as usize;
        // self.pos.start - inc <= j && j <= self.pos.end
        // let x = j.wrapping_sub(self.pos.start);
        // Alt 3
        // x < self.pos.len() || self.pos.start - 1 == j || self.pos.end == j
        // Alt 4 (not equivalent to any of the above)
        // self.pos.start == j + 1 || self.pos.start == j || self.pos.end - 1 == j || self.pos.end == j
    }
    pub fn is_adjacent_same_row(&self, j: usize) -> bool {
        self.pos.start == j + 1 || self.pos.end == j
    }
    pub fn is_adjacent(&self, i_self: usize, i: usize, j: usize) -> bool {
        if i_self == i {
            self.is_adjacent_same_row(j)
        } else {
            self.is_adjacent_other_row(j)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scan {
    prev_nums: Vec<Number>,
    prev_syms: Vec<usize>,
    curr_nums: Vec<Number>,
    curr_syms: Vec<usize>,
    sum: u32,
}

const OFFSET: u32 = '0' as u32;

impl Scan {
    pub fn consume_line(&mut self, s: &str) {
        // Acquire the current (from this line) numbers and symols
        let mut iter = s.trim_end_matches('\n').char_indices();
        while let Some((i, c)) = iter.next() {
            if c.is_ascii_digit() {
                let mut val = c as u32 - OFFSET;
                let left = i;
                let mut right = i + 1;
                while let Some((i, c)) = iter.next() {
                    if c.is_ascii_digit() {
                        val = val * 10 + (c as u32 - OFFSET);
                        right += 1;
                    } else {
                        if c != '.' {
                            self.curr_syms.push(i);
                        }
                        break;
                    }
                }
                let pos = left..right;
                self.curr_nums.push(Number::new(val, pos));
            } else if c != '.' {
                self.curr_syms.push(i);
            }
        }
        // Then, attempt to validate
        // Previous numbers against current symbols
        while let Some(num) = self.prev_nums.pop() {
            for sym in self.curr_syms.iter() {
                if num.is_adjacent_other_row(*sym) {
                    self.sum += num.value;
                    break;
                }
            }
        }
        // Eliminate any current numbers against previous symbols and current symbols
        'outer: while let Some(num) = self.curr_nums.pop() {
            for sym in self.prev_syms.iter() {
                if num.is_adjacent_other_row(*sym) {
                    self.sum += num.value;
                    continue 'outer;
                }
            }
            for sym in self.curr_syms.iter() {
                if num.is_adjacent_same_row(*sym) {
                    self.sum += num.value;
                    continue 'outer;
                }
            }
            self.prev_nums.push(num);
        }
        // Then, swap out the symbol contents
        self.prev_syms.clear();
        self.prev_syms.append(&mut self.curr_syms);
    }

    pub fn new() -> Self {
        Self {
            prev_nums: Vec::new(),
            prev_syms: Vec::new(),
            curr_nums: Vec::new(),
            curr_syms: Vec::new(),
            sum: 0,
        }
    }
    pub fn clear(&mut self) {
        self.prev_nums.clear();
        self.prev_syms.clear();
        self.curr_nums.clear();
        self.curr_syms.clear();
        self.sum = 0;
    }
}

pub fn sum_schematic<T: AsRef<Path>>(path: T) -> io::Result<u32> {
    let f = File::open(path.as_ref())?;
    let mut f = BufReader::new(f);
    // 1 KiB, as usual.
    let mut s = String::with_capacity(1024);
    let mut scan = Scan::new();
    while f.read_line(&mut s)? != 0 {
        scan.consume_line(&s);
        s.clear();
    }
    Ok(scan.sum)
}

pub fn gear_sum<T: AsRef<Path>>(path: T) -> io::Result<u32> {
    let f = File::open(path.as_ref())?;
    let mut f = BufReader::new(f);
    // 1 KiB, as usual.
    let mut s = String::with_capacity(1024);
    let mut scan = GearScan::new();
    while f.read_line(&mut s)? != 0 {
        scan.consume_line(&s);
        s.clear();
    }
    Ok(scan.gear_sum())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GearScan {
    lines: Vec<Vec<Number>>,
    loci: Vec<Locus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locus {
    line_num: usize,
    col_num: usize,
}
impl Locus {
    pub fn new(line_num: usize, col_num: usize) -> Self {
        Self { line_num, col_num }
    }
}

impl GearScan {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            loci: Vec::new(),
        }
    }
    pub fn consume_line(&mut self, s: &str) {
        let line_num = self.lines.len();
        let mut line: Vec<Number> = Vec::new();
        let mut iter = s.trim_end_matches('\n').char_indices();
        while let Some((i, c)) = iter.next() {
            if c.is_ascii_digit() {
                let mut val = c as u32 - OFFSET;
                let left = i;
                let mut right = i + 1;
                while let Some((i, c)) = iter.next() {
                    if c.is_ascii_digit() {
                        val = val * 10 + (c as u32 - OFFSET);
                        right += 1;
                    } else {
                        if c == '*' {
                            self.loci.push(Locus::new(line_num, i));
                        }
                        break;
                    }
                }
                let pos = left..right;
                line.push(Number::new(val, pos));
            } else if c == '*' {
                self.loci.push(Locus::new(line_num, i));
            }
        }
        self.lines.push(line);
    }

    pub fn gear_sum(&self) -> u32 {
        let m = self.lines.len();
        self.loci
            .iter()
            .filter_map(|locus| {
                let i = locus.line_num.clone();
                let j = locus.col_num.clone();
                let start = if i == 0 { 0 } else { i - 1 };
                let end = m.min(i + 2);
                let mut lhs: u32 = 0;
                let mut rhs: u32 = 0;
                let mut n: u8 = 0;
                for (i_self, line) in (start..end).zip(self.lines[start..end].iter()) {
                    for num in line {
                        if num.is_adjacent(i_self, i, j) {
                            if n == 2 {
                                return None;
                            } else if n == 0 {
                                lhs = num.value.clone();
                                n = 1;
                            } else if n == 1 {
                                rhs = num.value.clone();
                                n = 2;
                            }
                        }
                    }
                }
                if n == 2 {
                    Some(lhs * rhs)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_line_works() {
        let mut scan = Scan::new();
        let s = "467..114..";
        scan.consume_line(s);
        assert_eq!(
            scan.prev_nums,
            vec![Number::new(114, 5..8), Number::new(467, 0..3)]
        );
        assert_eq!(scan.prev_syms, vec![]);
        assert_eq!(scan.curr_nums, vec![]);
        assert_eq!(scan.curr_syms, vec![]);

        let s = "...*......";
        scan.consume_line(s);
        assert_eq!(scan.prev_nums, vec![]);
        assert_eq!(scan.prev_syms, vec![3]);
        assert_eq!(scan.curr_nums, vec![]);
        assert_eq!(scan.curr_syms, vec![]);
        assert_eq!(scan.sum, 467);
        let s = "..35..633.";
        scan.consume_line(s);

        assert_eq!(scan.prev_nums, vec![Number::new(633, 6..9)]);
        assert_eq!(scan.prev_syms, vec![]);
        assert_eq!(scan.curr_nums, vec![]);
        assert_eq!(scan.curr_syms, vec![]);
        assert_eq!(scan.sum, 467 + 35);
    }

    static TEST: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn scan_works() {
        let mut scan = Scan::new();
        for line in TEST.lines() {
            scan.consume_line(line);
        }
        assert_eq!(scan.sum, 4361);
    }

    #[test]
    fn gear_scan_works() {
        let mut scan = GearScan::new();
        for line in TEST.lines() {
            scan.consume_line(line);
        }
        assert_eq!(scan.gear_sum(), 467835);
    }
}
