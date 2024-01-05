use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::ops::IndexMut;
use std::path::Path;
use std::str::FromStr;

/// A column-major 2-dimensional grid
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    inner: Vec<T>,
    n_rows: usize,
    n_cols: usize,
    start: (usize, usize),
}

impl<T> Grid<T> {
    #[inline]
    fn linear_index(&self, i: usize, j: usize) -> usize {
        i + self.n_rows * j
    }
    #[inline]
    fn cartesian_index(n_rows: usize, idx: usize) -> (usize, usize) {
        (idx % n_rows, idx / n_rows)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    #[inline]
    pub fn n_rows(&self) -> usize {
        self.n_rows
    }
    #[inline]
    pub fn n_cols(&self) -> usize {
        self.n_cols
    }
}

impl<T: Clone> Grid<T> {
    pub fn transpose(&self) -> Grid<T> {
        let mut other = Vec::with_capacity(self.len());
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                other.push(self[(i, j)].clone());
            }
        }
        Grid {
            inner: other,
            n_rows: self.n_cols,
            n_cols: self.n_rows,
            start: (self.start.1, self.start.0),
        }
    }
}

impl Grid<Tile> {
    pub fn from_vec(v: Vec<Tile>, n_rows: usize, n_cols: usize) -> Self {
        assert_eq!(v.len(), n_rows * n_cols);
        let start = v
            .iter()
            .position(|x| *x == Tile::Start)
            .map(|i| Grid::<Tile>::cartesian_index(n_rows, i))
            .expect("`v` must contain a `Start` tile");
        Grid {
            inner: v,
            n_rows,
            n_cols,
            start,
        }
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let f = File::open(path.as_ref()).map_err(|e| e.to_string())?;
        let mut f = BufReader::new(f);
        let mut s = String::with_capacity(1024);
        let mut inner = Vec::new();
        let mut n_rows: usize = 0;
        while f.read_line(&mut s).map_err(|e| e.to_string())? != 0 {
            s.pop();
            Grid::consume_line(&mut inner, &s)?;
            n_rows += 1;
            s.clear();
        }
        let n_cols = inner.len() / n_rows;
        if inner.len() % n_rows != 0 {
            Err("unbalanced rows and columns".to_string())
        } else {
            let x = Grid::from_vec(inner, n_cols, n_rows);
            Ok(x.transpose())
        }
    }
    // A common convenience function
    fn consume_line(inner: &mut Vec<Tile>, s: &str) -> Result<(), String> {
        for c in s.chars() {
            inner.push(c.try_into()?);
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[self.linear_index(index.0, index.1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    // type Output = Tile;
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.linear_index(index.0, index.1);
        &mut self.inner[idx]
    }
}

impl FromStr for Grid<Tile> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = Vec::new();
        let mut iter = s.lines();
        if let Some(line) = iter.next() {
            Grid::consume_line(&mut inner, line)?;
            let n_cols = inner.len();
            for line in iter {
                Grid::consume_line(&mut inner, line)?;
                if inner.len() % n_cols != 0 {
                    return Err(s.to_string());
                }
            }
            let n_rows = inner.len() / n_cols;
            // We store the grid in column-major order, but `inner` is its
            // row-major representation. We transpose the dimensions on construction,
            // then transpose the grid to obtain the original.
            let x = Grid::from_vec(inner, n_cols, n_rows);
            Ok(x.transpose())
        } else {
            Err(s.to_string())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Vert,   // | is a vertical pipe connecting north and south.
    Horz,   // - is a horizontal pipe connecting east and west.
    NE,     // L is a 90-degree bend connecting north and east.
    NW,     // J is a 90-degree bend connecting north and west.
    SW,     // 7 is a 90-degree bend connecting south and west.
    SE,     // F is a 90-degree bend connecting south and east.
    Ground, // . is ground; there is no pipe in this tile.
    Start,  // S is the starting position of the animal
}
use Tile::*;
impl Tile {
    pub fn is_connector(&self) -> bool {
        match *self {
            Ground | Start => false,
            _ => true,
        }
    }
    pub fn is_start(&self) -> bool {
        *self == Start
    }
    /*
    `approach` is the proposed receiver on `other`.

    To illustrate the predicate, let `s` denote `self`, `o` denote `other`
    in the sketch below.


    s <- o    : form connection on left
    o -> s    : form connection on right

    s
    ^         : form connection on top
    |
    o

    o
    |         : form connection on bottom
    v
    s

    That is, this predicate tests whether the sender (`self`) can form a connection
    with the receiver (`other`) via its port (specified by `approach`).
     */
    pub fn is_valid_connection(&self, other: &Self, approach: &Direction) -> bool {
        match (self, other, approach) {
            (Vert, Vert, Bottom | Top) | (Vert, NW | NE, Top) | (Vert, SW | SE, Bottom) => true,
            (Horz, Horz, Left | Right) | (Horz, NE | SE, Right) | (Horz, NW | SW, Left) => true,
            (NE, NW | SW | Horz, Left) => true,
            (NE, SW | SE | Vert, Bottom) => true,
            (NW, NE | SE | Horz, Right) => true,
            (NW, SW | SE | Vert, Bottom) => true,
            (SW, NE | SE | Horz, Right) => true,
            (SW, NE | NW | Vert, Top) => true,
            (SE, SW | NW | Horz, Left) => true,
            (SE, NE | NW | Vert, Top) => true,
            // Start special cases
            (Start, Vert, Top | Bottom) => true,
            (Start, Horz, Left | Right) => true,
            (Start, NE | NW, Top) => true,
            (Start, NE | SE, Right) => true,
            (Start, SW | SE, Bottom) => true,
            (Start, NW | SW, Left) => true,
            // And their transposes
            (Vert, Start, Top | Bottom) => true,
            (Horz, Start, Left | Right) => true,
            (NE | NW, Start, Bottom) => true,
            (NE | SE, Start, Left) => true,
            (SW | SE, Start, Top) => true,
            (NW | SW, Start, Right) => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Vert),
            '-' => Ok(Horz),
            'L' => Ok(NE),
            'J' => Ok(NW),
            '7' => Ok(SW),
            'F' => Ok(SE),
            '.' => Ok(Ground),
            'S' => Ok(Start),
            _ => Err(c.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    // Entry,
}
use Direction::*;

impl Direction {
    pub fn inverse(&self) -> Self {
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
    }
}
#[derive(Debug)]
pub struct Visitor<'a> {
    idx: (usize, usize),
    tile: Tile,
    approach: Direction,
    last_row: usize,
    last_col: usize,
    steps: usize,
    // path: Vec<Tile>,
    grid: &'a Grid<Tile>,
}
impl Visitor<'_> {
    pub fn propose(&self) -> Direction {
        match (self.tile, self.approach) {
            (NE, Top) => Right,
            (NE, Right) => Top,
            (SW, Left) => Bottom,
            (SW, Bottom) => Left,
            (NW, Left) => Top,
            (NW, Top) => Left,
            (SE, Bottom) => Right,
            (SE, Right) => Bottom,
            (Vert, Top) => Bottom,
            (Vert, Bottom) => Top,
            (Horz, Left) => Right,
            (Horz, Right) => Left,
            _ => unreachable!(),
        }
    }
    pub fn is_proposal_inbounds(&self, dir: &Direction) -> bool {
        match dir {
            Top => self.idx.0 != 0,
            Bottom => self.idx.0 != self.last_col,
            Left => self.idx.1 != 0,
            Right => self.idx.1 != self.last_row,
        }
    }
    pub fn is_feasible(&self, dir: &Direction) -> bool {
        if self.is_proposal_inbounds(dir) {
            let (i, j) = self.idx.clone();
            let idx = match dir {
                Top => (i - 1, j),
                Bottom => (i + 1, j),
                Left => (i, j - 1),
                Right => (i, j + 1),
            };
            self.tile
                .is_valid_connection(&self.grid[idx], &dir.inverse())
        } else {
            false
        }
    }
    pub fn move_if_feasible(&mut self, dir: &Direction) -> bool {
        if self.is_proposal_inbounds(dir) {
            let (i, j) = self.idx.clone();
            let idx = match dir {
                Top => (i - 1, j),
                Bottom => (i + 1, j),
                Left => (i, j - 1),
                Right => (i, j + 1),
            };
            let approach = dir.inverse();
            if self.tile.is_valid_connection(&self.grid[idx], &approach) {
                self.tile = self.grid[idx].clone();
                self.approach = approach;
                self.idx = idx;
                self.steps += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn traverse(&mut self) {
        let mut tile = self.tile.clone();
        while tile != Start {
            let proposal = self.propose();
            self.move_if_feasible(&proposal);
            tile = self.tile.clone();
        }
    }
    pub fn farthest(&self) -> usize {
        self.steps / 2
    }
}

impl<'a> TryFrom<&'a Grid<Tile>> for Visitor<'a> {
    type Error = String;
    fn try_from(grid: &'a Grid<Tile>) -> Result<Self, String> {
        let last_row = grid.n_rows() - 1;
        let last_col = grid.n_cols() - 1;
        let idx = grid.start.clone();

        let mut vis = Visitor {
            idx,
            tile: Start,
            approach: Top,
            last_row,
            last_col,
            steps: 0,
            grid,
        };
        if vis.move_if_feasible(&Top)
            || vis.move_if_feasible(&Bottom)
            || vis.move_if_feasible(&Left)
            || vis.move_if_feasible(&Right)
        {
            Ok(vis)
        } else {
            Err("cannot initialize visitor".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    MainLoop,
    Inside,
    Outside,
    Null,
}
use State::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatefulTile {
    tile: Tile,
    state: State,
}

type StateGrid = Grid<Rc<RefCell<StatefulTile>>>;

impl From<Grid<Tile>> for StateGrid {
    fn from(grid: Grid<Tile>) -> Self {
        let mut inner = Vec::with_capacity(grid.len());
        for tile in grid.inner {
            inner.push(Rc::new(RefCell::new(StatefulTile { tile, state: Null })));
        }
        Self {
            inner,
            n_rows: grid.n_rows,
            n_cols: grid.n_cols,
            start: grid.start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose() {
        let x = Grid::from_vec(vec![Vert, Horz, NE, NW, SW, SE, Ground, Start, NE], 3, 3);
        let lhs = x.transpose();
        let rhs = Grid::from_vec(vec![Vert, NW, Ground, Horz, SW, Start, NE, SE, NE], 3, 3);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn from_str() {
        let s = "\
.S-7.
.|.|.
.L-J.
";
        let lhs = s.parse::<Grid<Tile>>().unwrap();
        let rhs = Grid::from_vec(
            vec![
                Ground, Ground, Ground, Start, Vert, NE, Horz, Ground, Horz, SW, Vert, NW, Ground,
                Ground, Ground,
            ],
            3,
            5,
        );
        assert_eq!(lhs, rhs);
    }

    static TEST1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

    static TEST2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn visitor_try_from() {
        let grid = TEST1.parse::<Grid<Tile>>().unwrap();
        let vis = Visitor::try_from(&grid);
        assert!(vis.is_ok());
        let grid = TEST2.parse::<Grid<Tile>>().unwrap();
        let vis = Visitor::try_from(&grid);
        assert!(vis.is_ok());
    }

    #[test]
    fn traverse() {
        let grid = TEST1.parse::<Grid<Tile>>().unwrap();
        let mut vis = Visitor::try_from(&grid).unwrap();
        assert_eq!(vis.idx, (2, 1));
        assert_eq!(vis.tile, Vert);
        assert_eq!(vis.approach, Top);
        assert!(vis.move_if_feasible(&vis.propose()));

        assert_eq!(vis.idx, (3, 1));
        assert_eq!(vis.tile, NE);
        assert_eq!(vis.approach, Top);
        assert!(vis.move_if_feasible(&vis.propose()));
        assert_eq!(vis.idx, (3, 2));
        assert_eq!(vis.tile, Horz);
        assert_eq!(vis.approach, Left);
        assert!(vis.move_if_feasible(&vis.propose()));
        assert_eq!(vis.idx, (3, 3));
        assert_eq!(vis.tile, NW);
        assert_eq!(vis.approach, Left);
        assert!(vis.move_if_feasible(&vis.propose()));
        assert_eq!(vis.idx, (2, 3));
        assert_eq!(vis.tile, Vert);
        assert_eq!(vis.approach, Bottom);
        assert!(vis.move_if_feasible(&vis.propose()));
        assert_eq!(vis.idx, (1, 3));
        assert_eq!(vis.tile, SW);
        assert_eq!(vis.approach, Bottom);
        assert!(vis.move_if_feasible(&vis.propose()));
        assert_eq!(vis.idx, (1, 2));
        assert_eq!(vis.tile, Horz);
        assert_eq!(vis.approach, Right);
        assert!(vis.move_if_feasible(&vis.propose()));
        assert_eq!(vis.idx, (1, 1));
        assert_eq!(vis.tile, Start);
        assert_eq!(vis.approach, Right);
        assert_eq!(vis.steps, 8);

        let grid = TEST2.parse::<Grid<Tile>>().unwrap();
        let mut vis = Visitor::try_from(&grid).unwrap();
        vis.traverse();
        assert_eq!(vis.steps, 16);
    }
}
