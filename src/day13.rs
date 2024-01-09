use std::fmt;
use std::fs;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    inner: Vec<bool>,
    n_rows: usize,
    n_cols: usize,
}
impl Grid {
    #[inline]
    fn linear_index(&self, i: usize, j: usize) -> usize {
        i + self.n_rows * j
    }
    #[inline]
    pub fn n_rows(&self) -> usize {
        self.n_rows
    }
    #[inline]
    pub fn n_cols(&self) -> usize {
        self.n_cols
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn transpose(&self) -> Self {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        let mut other = Vec::with_capacity(self.len());
        for i in 0..n_rows {
            for j in 0..n_cols {
                other.push(self[(i, j)].clone());
            }
        }
        Self {
            inner: other,
            n_rows: n_cols,
            n_cols: n_rows,
        }
    }
    pub fn from_vec(v: Vec<bool>, n_rows: usize, n_cols: usize) -> Self {
        assert_eq!(v.len(), n_rows * n_cols);
        Grid {
            inner: v,
            n_rows,
            n_cols,
        }
    }

    pub fn are_columns_equal(&self, j0: usize, j1: usize) -> bool {
        let n_rows = self.n_rows();
        let idx0 = j0 * n_rows;
        let idx1 = j1 * n_rows;
        self.inner[idx0..idx0 + n_rows] == self.inner[idx1..idx1 + n_rows]
    }
    pub fn are_rows_equal(&self, i0: usize, i1: usize) -> bool {
        let n_cols = self.n_cols();
        for j in 0..n_cols {
            if self[(i0, j)] != self[(i1, j)] {
                return false;
            }
        }
        true
    }

    /*
    These are O(n^2) themselves, with O(n) `are_columns_equal`, `are_rows_equal`
    yielding O(n^3). A simple way to keep it O(n^2) is to compute a of each column,
    which would be an O(n^2) operation by itself, but, obviously, worthwhile.
     */
    pub fn find_reflection_vertical(&self) -> Option<usize> {
        let n_cols = self.n_cols();
        let mut start: usize = 0;
        let end = n_cols - 1;
        while start < end {
            if let Some(j) = (start..end).find(|&j| self.are_columns_equal(j, j + 1)) {
                let left = (0..j).rev();
                let right = j + 2..n_cols;
                if left
                    .zip(right)
                    .all(|(left, right)| self.are_columns_equal(left, right))
                {
                    return Some(j);
                }
            }
            start += 1;
        }
        None
    }
    pub fn find_reflection_horizontal(&self) -> Option<usize> {
        let n_rows = self.n_rows();
        let mut start: usize = 0;
        let end = n_rows - 1;
        while start < end {
            if let Some(i) = (start..end).find(|&i| self.are_rows_equal(i, i + 1)) {
                let above = (0..i).rev();
                let below = i + 2..n_rows;
                if above
                    .zip(below)
                    .all(|(above, below)| self.are_rows_equal(above, below))
                {
                    return Some(i);
                }
            }
            start += 1;
        }
        None
    }

    pub fn find_reflection(&self) -> Option<Reflection> {
        if let Some(n) = self.find_reflection_vertical() {
            Some(Reflection::Vertical(n + 1))
        } else if let Some(n) = self.find_reflection_horizontal() {
            Some(Reflection::Horizontal(n + 1))
        } else {
            None
        }
    }
}
impl Index<(usize, usize)> for Grid {
    type Output = bool;
    #[inline]
    fn index(&self, cartesian: (usize, usize)) -> &Self::Output {
        let idx = self.linear_index(cartesian.0, cartesian.1);
        &self.inner[idx]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    #[inline]
    fn index_mut(&mut self, cartesian: (usize, usize)) -> &mut Self::Output {
        let idx = self.linear_index(cartesian.0, cartesian.1);
        &mut self.inner[idx]
    }
}

impl FromStr for Grid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = Vec::new();
        let mut n_rows: usize = 0;
        for line in s.lines() {
            n_rows += 1;
            for c in line.chars() {
                let e = match c {
                    '#' => true,
                    '.' => false,
                    _ => return Err(c.to_string()),
                };
                inner.push(e);
            }
        }
        let n_cols = inner.len() / n_rows;
        if inner.len() % n_rows != 0 {
            Err(s.to_string())
        } else {
            let x = Grid {
                inner,
                n_rows: n_cols,
                n_cols: n_rows,
            };
            Ok(x.transpose())
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        for i in 0..n_rows {
            for j in 0..n_cols {
                let c = match self[(i, j)] {
                    true => '#',
                    false => '.',
                };
                write!(f, "{}", c)?;
            }
            if i != n_rows - 1 {
                write!(f, "{}", '\n')?;
            }
        }
        Ok(())
    }
}

pub fn grids_from_str(s: &str) -> Result<Vec<Grid>, String> {
    let mut v = Vec::new();
    for chunk in s.split("\n\n") {
        v.push(chunk.parse::<Grid>()?);
    }
    Ok(v)
}
pub fn grids_from_path<T: AsRef<Path>>(path: T) -> Result<Vec<Grid>, String> {
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    grids_from_str(&s)
}

pub fn sum_reflections(grids: &[Grid]) -> usize {
    grids
        .into_iter()
        .filter_map(|x| x.find_reflection())
        .fold(0usize, |acc, x| match x {
            Reflection::Vertical(n) => acc + n,
            Reflection::Horizontal(n) => acc + 100 * n,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    static VERT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    static HORZ: &str = "\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    static VERT2: &str = "\
.#....###
......#..
..##.#...
##..#....
#####.#..
#####.###
##..#....";

    #[test]
    fn grids_from_str_works() {
        assert_eq!(grids_from_str(TEST).unwrap().len(), 2);
    }

    #[test]
    fn are_columns_equal() {
        let x = VERT2.parse::<Grid>().unwrap();
        assert!(x.are_columns_equal(7, 8));
    }

    #[test]
    fn find_reflection_vertical() {
        let x = VERT.parse::<Grid>().unwrap();
        let idx = x.find_reflection_vertical().unwrap();
        assert_eq!(idx, 4);

        let x = VERT2.parse::<Grid>().unwrap();
        let idx = x.find_reflection_vertical().unwrap();
        assert_eq!(idx, 7);
    }

    #[test]
    fn find_reflection_horizontal() {
        let x = HORZ.parse::<Grid>().unwrap();
        let idx = x.find_reflection_horizontal().unwrap();
        assert_eq!(idx, 3);
    }

    #[test]
    fn sum_reflections_works() {
        let grids = grids_from_str(TEST).unwrap();
        assert_eq!(sum_reflections(&grids), 405);
    }
}
