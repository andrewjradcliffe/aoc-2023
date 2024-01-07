use std::num::NonZeroUsize;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    // Galaxy: true, EmptySpace: false
    inner: Vec<bool>,
    n_rows: usize,
    n_cols: usize,
}
impl Grid {
    #[inline]
    fn linear_index(&self, i: usize, j: usize) -> usize {
        i + self.n_rows * j
    }
    // #[inline]
    // fn cartesian_index(n_rows: usize, idx: usize) -> (usize, usize) {
    //     (idx % n_rows, idx / n_rows)
    // }
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
    pub fn insert_empty_column(&mut self, j: usize) {
        let n_rows = self.n_rows();
        let idx = n_rows * j;
        for i in idx..idx + n_rows {
            self.inner.insert(i, false);
        }
        self.n_cols += 1;
    }
    pub fn insert_empty_row(&mut self, i: usize) {
        let n_cols = self.n_cols();
        let n_rows_new = self.n_rows() + 1;
        for j in 0..n_cols {
            self.inner.insert(i + j * n_rows_new, false);
        }
        self.n_rows = n_rows_new;
    }
    pub fn from_vec(v: Vec<bool>, n_rows: usize, n_cols: usize) -> Self {
        assert_eq!(v.len(), n_rows * n_cols);
        Grid {
            inner: v,
            n_rows,
            n_cols,
        }
    }
    pub fn is_row_empty(&self, i: usize) -> bool {
        let n_cols = self.n_cols();
        !(0..n_cols).any(|j| self[(i, j)])
    }
    pub fn is_column_empty(&self, j: usize) -> bool {
        let n_rows = self.n_rows();
        let idx = n_rows * j;
        !self.inner[idx..idx + n_rows].iter().any(|x| *x)
    }
    pub fn empty_rows(&self) -> Vec<usize> {
        let n_rows = self.n_rows();
        let mut v = Vec::with_capacity(n_rows);
        for i in 0..n_rows {
            if self.is_row_empty(i) {
                v.push(i);
            }
        }
        v
    }
    pub fn empty_columns(&self) -> Vec<usize> {
        let n_cols = self.n_cols();
        let mut v = Vec::with_capacity(n_cols);
        for j in 0..n_cols {
            if self.is_column_empty(j) {
                v.push(j);
            }
        }
        v
    }
    pub fn expand_empty_columns(&mut self) {
        let v = self.empty_columns();
        for (offset, j) in v.into_iter().enumerate() {
            self.insert_empty_column(j + offset);
        }
    }
    pub fn expand_empty_rows(&mut self) {
        let v = self.empty_rows();
        for (offset, i) in v.into_iter().enumerate() {
            self.insert_empty_row(i + offset);
        }
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
        s.parse::<Self>()
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
                match c {
                    '.' => {
                        inner.push(false);
                    }
                    '#' => {
                        inner.push(true);
                    }
                    _ => return Err(c.to_string()),
                }
            }
        }
        let n_cols = inner.len() / n_rows;
        if inner.len() % n_rows != 0 {
            return Err(s.to_string());
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
                let c = if self[(i, j)] { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            if i != n_rows - 1 {
                write!(f, "{}", '\n')?;
            }
        }
        Ok(())
    }
}
pub struct Galaxies {
    inner: Vec<(usize, usize)>,
}
impl<'a> From<&'a Grid> for Galaxies {
    fn from(grid: &'a Grid) -> Self {
        let mut inner = Vec::new();
        let n_rows = grid.n_rows();
        let n_cols = grid.n_cols();
        for j in 0..n_cols {
            for i in 0..n_rows {
                if grid[(i, j)] {
                    inner.push((i, j));
                }
            }
        }
        Self { inner }
    }
}

impl Galaxies {
    pub fn manhattan_distances(&self) -> Vec<usize> {
        let n = self.inner.len();
        if n > 1 {
            let mut v = Vec::with_capacity(n * n - 1);
            for i in 0..n {
                let x = self.inner[i].clone();
                for j in i + 1..n {
                    let y = self.inner[j].clone();
                    let d = x.0.abs_diff(y.0) + x.1.abs_diff(y.1);
                    v.push(d);
                }
            }
            v
        } else {
            Vec::new()
        }
    }
    pub fn sum_manhattan_distances(&self) -> usize {
        self.manhattan_distances().into_iter().sum()
    }
}

pub fn expanded_universe(grid: &Grid, factor: NonZeroUsize) -> Galaxies {
    let factor = factor.get();
    let f = factor - 1;
    let empty_rows = grid.empty_rows();
    let empty_cols = grid.empty_columns();
    let galaxies = Galaxies::from(grid);
    let mut inner = galaxies.inner.clone();
    for j in empty_cols {
        for (orig, new) in galaxies.inner.iter().zip(inner.iter_mut()) {
            if orig.1 > j {
                new.1 += f;
            }
        }
    }
    for i in empty_rows {
        for (orig, new) in galaxies.inner.iter().zip(inner.iter_mut()) {
            if orig.0 > i {
                new.0 += f;
            }
        }
    }
    Galaxies { inner }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let s = "\
..#.
#...
...#";
        let lhs = s.parse::<Grid>().unwrap();
        let rhs = Grid::from_vec(
            vec![
                false, true, false, false, false, false, true, false, false, false, false, true,
            ],
            3,
            4,
        );
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn to_string() {
        let s = "\
..#.
#...
...#";
        let lhs = s.parse::<Grid>().unwrap().to_string();
        assert_eq!(lhs, s);
        let lhs = TEST.parse::<Grid>().unwrap().to_string();
        assert_eq!(lhs, TEST);
    }

    static TEST: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    static EXPAND: &str = "\
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    #[test]
    fn expand() {
        let mut grid = TEST.parse::<Grid>().unwrap();
        grid.expand_empty_rows();
        grid.expand_empty_columns();
        let lhs = grid.to_string();
        assert_eq!(lhs, EXPAND);
    }

    #[test]
    fn sum_manhattan_distances() {
        let mut grid = TEST.parse::<Grid>().unwrap();
        grid.expand_empty_rows();
        grid.expand_empty_columns();
        let galaxies = Galaxies::from(&grid);
        assert_eq!(galaxies.sum_manhattan_distances(), 374);
    }
    #[test]
    fn expanded_universe_works() {
        let grid = TEST.parse::<Grid>().unwrap();
        let galaxies = expanded_universe(&grid, NonZeroUsize::new(2).unwrap());
        assert_eq!(galaxies.sum_manhattan_distances(), 374);
        let galaxies = expanded_universe(&grid, NonZeroUsize::new(10).unwrap());
        assert_eq!(galaxies.sum_manhattan_distances(), 1030);
        let galaxies = expanded_universe(&grid, NonZeroUsize::new(100).unwrap());
        assert_eq!(galaxies.sum_manhattan_distances(), 8410);
    }
}
