//! A column-major grid implemented on a single `Vec<T>`.

use std::convert::TryFrom;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    // Public within this crate since I may (ab)use these in multiple places.
    pub(crate) inner: Vec<T>,
    pub(crate) n_rows: usize,
    pub(crate) n_cols: usize,
}

impl<T> Grid<T> {
    #[inline]
    fn linear_index(&self, i: usize, j: usize) -> usize {
        i + self.n_rows * j
    }
    #[inline]
    fn linear_index_tr(&self, i: usize, j: usize) -> usize {
        i * self.n_cols + j
    }
    #[inline]
    fn cartesian_index(n_rows: usize, idx: usize) -> (usize, usize) {
        (idx % n_rows, idx / n_rows)
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
    #[inline]
    pub fn shape(&self) -> (usize, usize) {
        (self.n_rows, self.n_cols)
    }

    pub fn transpose_mut(&mut self) {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        for j in 0..n_cols {
            for i in 0..j {
                let src = self.linear_index(i, j);
                let dst = self.linear_index_tr(i, j);
                self.inner.swap(src, dst);
            }
        }
        self.n_rows = n_cols;
        self.n_cols = n_rows;
    }
    pub fn from_vec(v: Vec<T>, n_rows: usize, n_cols: usize) -> Self {
        assert_eq!(v.len(), n_cols * n_rows);
        Self {
            inner: v,
            n_cols,
            n_rows,
        }
    }
}

impl<T: Clone> Grid<T> {
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
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        for i in 0..n_rows {
            for j in 0..n_cols {
                write!(f, "{}", self[(i, j)])?;
            }
            if i != n_rows - 1 {
                write!(f, "{}", '\n')?;
            }
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    #[inline]
    fn index(&self, cartesian: (usize, usize)) -> &Self::Output {
        let idx = self.linear_index(cartesian.0, cartesian.1);
        &self.inner[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, cartesian: (usize, usize)) -> &mut Self::Output {
        let idx = self.linear_index(cartesian.0, cartesian.1);
        &mut self.inner[idx]
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char, Error = String>,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        let mut n_rows: usize = 0;
        for line in s.lines() {
            n_rows += 1;
            for c in line.chars() {
                v.push(T::try_from(c)?);
            }
        }
        let n = v.len();
        let n_cols = n / n_rows;
        if n % n_rows != 0 {
            Err(s.to_string())
        } else {
            let mut x = Grid::from_vec(v, n_cols, n_rows);
            x.transpose_mut();
            Ok(x)
        }
    }
}
