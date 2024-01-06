use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::ops::IndexMut;
use std::path::Path;
use std::str::FromStr;

/// A column-major 2-dimensional grid
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    inner: Vec<Tile>,
    n_rows: usize,
    n_cols: usize,
    start: (usize, usize),
}

impl Grid {
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

    #[inline]
    fn is_inbounds(&self, i: usize, j: usize) -> bool {
        (i < self.n_rows) & (j < self.n_cols)
    }
    pub fn transpose(&self) -> Grid {
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

    pub fn from_vec(v: Vec<Tile>, n_rows: usize, n_cols: usize) -> Self {
        assert_eq!(v.len(), n_rows * n_cols);
        let start = v
            .iter()
            .position(|x| *x == Tile::Start)
            .map(|i| Grid::cartesian_index(n_rows, i))
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

impl Index<(usize, usize)> for Grid {
    type Output = Tile;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[self.linear_index(index.0, index.1)]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    // type Output = Tile;
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.linear_index(index.0, index.1);
        &mut self.inner[idx]
    }
}

impl FromStr for Grid {
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        for i in 0..n_rows {
            for j in 0..n_cols {
                write!(f, "{}", self[(i, j)])?;
            }
            if i != n_rows - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
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
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Vert => '|',
            Horz => '-',
            NE => 'L',
            NW => 'J',
            SW => '7',
            SE => 'F',
            Ground => '.',
            Start => 'S',
        };
        write!(f, "{}", c)
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
    grid: &'a Grid,
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
            Bottom => self.idx.0 != self.last_row,
            Left => self.idx.1 != 0,
            Right => self.idx.1 != self.last_col,
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

impl<'a> TryFrom<&'a Grid> for Visitor<'a> {
    type Error = String;
    fn try_from(grid: &'a Grid) -> Result<Self, String> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    MainLoop,
    Inside,
    Outside,
    Null,
}
use State::*;

#[derive(Debug)]
pub struct StatefulVisitor<'a> {
    vis: Visitor<'a>,
    states: Vec<State>,
}
impl<'a> TryFrom<&'a Grid> for StatefulVisitor<'a> {
    type Error = String;
    fn try_from(grid: &'a Grid) -> Result<Self, String> {
        let vis = Visitor::try_from(grid)?;
        let n = vis.grid.len();
        let mut states = Vec::with_capacity(n);
        states.resize(n, Null);
        states[vis.grid.linear_index(grid.start.0, grid.start.1)] = MainLoop;
        states[vis.grid.linear_index(vis.idx.0, vis.idx.1)] = MainLoop;
        Ok(Self { vis, states })
    }
}

impl StatefulVisitor<'_> {
    fn linear_index(&self, i: usize, j: usize) -> usize {
        self.vis.grid.linear_index(i, j)
    }
    fn vis_index(&self) -> usize {
        self.vis.grid.linear_index(self.vis.idx.0, self.vis.idx.1)
    }
    fn state(&self, i: usize, j: usize) -> State {
        let idx = self.linear_index(i, j);
        self.states[idx]
    }
    pub fn main_loop(&mut self) {
        let mut tile = self.vis.tile.clone();
        while tile != Start {
            let proposal = self.vis.propose();
            if self.vis.move_if_feasible(&proposal) {
                let idx = self.vis_index();
                self.states[idx] = MainLoop;
                tile = self.vis.tile.clone();
            }
        }
    }
    // N.B. idempotent.
    pub fn classify_perimeter(&mut self) {
        let n_rows = self.vis.grid.n_rows();
        let n_cols = self.vis.grid.n_cols();
        for j in [0, n_cols - 1] {
            for i in 0..n_rows {
                let idx = self.linear_index(i, j);
                match self.states[idx] {
                    MainLoop => (),
                    _ => {
                        self.states[idx] = Outside;
                    }
                }
            }
        }
        for i in [0, n_rows - 1] {
            for j in 1..n_cols - 1 {
                let idx = self.linear_index(i, j);
                match self.states[idx] {
                    MainLoop => (),
                    _ => {
                        self.states[idx] = Outside;
                    }
                }
            }
        }
    }
    fn try_left(&self, i: usize, j: usize) -> bool {
        let j = j - 1;
        if self.vis.grid.is_inbounds(i, j) {
            match self.state(i, j) {
                Outside => true,
                _ => self.try_left_escape_top(i, j) || self.try_left_escape_bottom(i, j),
            }
        } else {
            false
        }
    }
    fn try_left_escape_top(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            if self.state(i, j) == Outside {
                true
            } else if self.vis.grid.is_inbounds(i - 1, j) {
                let above = self.vis.grid[(i - 1, j)];
                let below = self.vis.grid[(i, j)];
                match (above, below) {
                    (Horz | NE | NW | Ground, Horz | SW | SE | Ground) => {
                        if self.state(i - 1, j) == Outside {
                            true
                        } else {
                            self.try_left_escape_top(i, j - 1)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn try_left_escape_bottom(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            if self.state(i, j) == Outside {
                true
            } else if self.vis.grid.is_inbounds(i + 1, j) {
                let above = self.vis.grid[(i, j)];
                let below = self.vis.grid[(i + 1, j)];
                match (above, below) {
                    (Horz | NE | NW | Ground, Horz | SW | SE | Ground) => {
                        if self.state(i + 1, j) == Outside {
                            true
                        } else {
                            self.try_left_escape_bottom(i, j - 1)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn try_right(&self, i: usize, j: usize) -> bool {
        let j = j + 1;
        if self.vis.grid.is_inbounds(i, j) {
            match self.state(i, j) {
                Outside => true,
                _ => self.try_right_escape_top(i, j) || self.try_right_escape_bottom(i, j),
            }
        } else {
            false
        }
    }
    fn try_right_escape_top(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            if self.state(i, j) == Outside {
                true
            } else if self.vis.grid.is_inbounds(i - 1, j) {
                let above = self.vis.grid[(i - 1, j)];
                let below = self.vis.grid[(i, j)];
                match (above, below) {
                    (Horz | NE | NW | Ground, Horz | SW | SE | Ground) => {
                        if self.state(i - 1, j) == Outside {
                            true
                        } else {
                            self.try_right_escape_top(i, j + 1)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn try_right_escape_bottom(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            if self.state(i, j) == Outside {
                true
            } else if self.vis.grid.is_inbounds(i + 1, j) {
                let above = self.vis.grid[(i, j)];
                let below = self.vis.grid[(i + 1, j)];
                match (above, below) {
                    (Horz | NE | NW | Ground, Horz | SW | SE | Ground) => {
                        if self.state(i + 1, j) == Outside {
                            true
                        } else {
                            self.try_right_escape_bottom(i, j + 1)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn try_top(&self, i: usize, j: usize) -> bool {
        let i = i - 1;
        if self.vis.grid.is_inbounds(i, j) {
            match self.state(i, j) {
                Outside => true,
                _ => self.try_top_escape_right(i, j) || self.try_top_escape_left(i, j),
            }
        } else {
            false
        }
    }
    fn try_top_escape_right(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            if self.state(i, j) == Outside {
                true
            } else if self.vis.grid.is_inbounds(i, j + 1) {
                let left = self.vis.grid[(i, j)];
                let right = self.vis.grid[(i, j + 1)];
                match (left, right) {
                    (Vert | SW | NW | Ground, Vert | SE | NE | Ground) => {
                        if self.state(i, j + 1) == Outside {
                            true
                        } else {
                            self.try_top_escape_right(i - 1, j)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn try_top_escape_left(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            if self.state(i, j) == Outside {
                true
            } else if self.vis.grid.is_inbounds(i, j - 1) {
                let left = self.vis.grid[(i, j - 1)];
                let right = self.vis.grid[(i, j)];
                match (left, right) {
                    (Vert | SW | NW | Ground, Vert | SE | NE | Ground) => {
                        if self.state(i, j - 1) == Outside {
                            true
                        } else {
                            self.try_top_escape_left(i - 1, j)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn try_bottom(&self, i: usize, j: usize) -> bool {
        let i = i + 1;
        if self.vis.grid.is_inbounds(i, j) {
            match self.state(i, j) {
                Outside => true,
                _ => self.try_bottom_escape_right(i, j) || self.try_bottom_escape_left(i, j),
            }
        } else {
            false
        }
    }
    fn try_bottom_escape_right(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            if self.state(i, j) == Outside {
                true
            } else if self.vis.grid.is_inbounds(i, j + 1) {
                let left = self.vis.grid[(i, j)];
                let right = self.vis.grid[(i, j + 1)];
                match (left, right) {
                    (Vert | SW | NW | Ground, Vert | SE | NE | Ground) => {
                        if self.state(i, j + 1) == Outside {
                            true
                        } else {
                            self.try_bottom_escape_right(i + 1, j)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn try_bottom_escape_left(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            if self.state(i, j) == Outside {
                true
            } else if self.vis.grid.is_inbounds(i, j - 1) {
                let left = self.vis.grid[(i, j - 1)];
                let right = self.vis.grid[(i, j)];
                match (left, right) {
                    (Vert | SW | NW | Ground, Vert | SE | NE | Ground) => {
                        if self.state(i, j - 1) == Outside {
                            true
                        } else {
                            self.try_bottom_escape_left(i + 1, j)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    // fn try_bottom_escape(&self, i: usize, j: usize, dir: Direction) -> bool {
    //     let j_other = match dir {
    //         Left => j - 1,
    //         Right => j + 1,
    //         _ => unreachable!(),
    //     };
    //     let (left, right) = match dir {
    //         Left => (self.vis.grid[(i, j - 1)], self.vis.grid[(i, j)]),
    //         Right => (self.vis.grid[(i, j)], self.vis.grid[(i, j + 1)]),
    //         _ => unreachable!(),
    //     };
    //     if self.vis.grid.is_inbounds(i, j) {
    //         if self.state(i, j) == Outside {
    //             true
    //         } else if self.vis.grid.is_inbounds(i, j_other) {
    //             match (left, right) {
    //                 (Vert | SW | NW | Ground, Vert | SE | NE | Ground) => {
    //                     if self.state(i, j_other) == Outside {
    //                         true
    //                     } else {
    //                         self.try_bottom_escape_left(i + 1, j)
    //                     }
    //                 }
    //                 _ => false,
    //             }
    //         } else {
    //             false
    //         }
    //     } else {
    //         false
    //     }
    // }
    fn is_not_null(&self, i: usize, j: usize) -> bool {
        if self.vis.grid.is_inbounds(i, j) {
            self.state(i, j) != Null
        } else {
            true
        }
    }
    pub fn all_neighbors_classified(&self, i: usize, j: usize) -> bool {
        self.is_not_null(i.wrapping_sub(1), j)
            && self.is_not_null(i + 1, j)
            && self.is_not_null(i, j + 1)
            && self.is_not_null(i, j.wrapping_sub(1))
    }
    pub fn classify_interior(&mut self, i: usize, j: usize) -> bool {
        let idx = self.linear_index(i, j);
        match self.state(i, j) {
            Null => {
                if self.try_left(i, j)
                    || self.try_right(i, j)
                    || self.try_top(i, j)
                    || self.try_bottom(i, j)
                {
                    self.states[idx] = Outside;
                    true
                } else {
                    if self.all_neighbors_classified(i, j) {
                        self.states[idx] = Inside;
                        true
                    } else {
                        false
                    }
                }
            }
            Inside => true,
            Outside => true,
            MainLoop => true,
        }
    }
    fn try_connect_outside(&mut self, i: usize, j: usize, dir: Direction) -> bool {
        match self.state(i, j) {
            Outside => {
                let (i, j) = match dir {
                    Top => (i.wrapping_sub(1), j),
                    Bottom => (i + 1, j),
                    Right => (i, j + 1),
                    Left => (i, j.wrapping_sub(1)),
                };
                if self.vis.grid.is_inbounds(i, j) {
                    match self.state(i, j) {
                        MainLoop | Inside => false,
                        Null => {
                            let idx = self.linear_index(i, j);
                            self.states[idx] = Outside;
                            self.try_connect_outside(i, j, dir)
                        }
                        Outside => self.try_connect_outside(i, j, dir),
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    pub fn try_connect(&mut self, i: usize, j: usize) {
        self.try_connect_outside(i, j, Top);
        self.try_connect_outside(i, j, Bottom);
        self.try_connect_outside(i, j, Left);
        self.try_connect_outside(i, j, Right);
    }
    // Idempotent
    pub fn extend_outside(&mut self) {
        let mut outside = self.indices(Outside);
        let mut n_old = outside.len();
        loop {
            while let Some((i, j)) = outside.pop() {
                self.try_connect(i, j);
            }
            self.indices_mut(&mut outside, Outside);
            let n = outside.len();
            if n == n_old {
                break;
            } else {
                n_old = n;
            }
        }
    }

    pub fn enclosed(&self) -> usize {
        self.states.iter().filter(|x| **x == Inside).count()
    }
    fn indices_mut(&self, v: &mut Vec<(usize, usize)>, s: State) {
        let n_rows = self.vis.grid.n_rows();
        v.clear();
        self.states
            .iter()
            .enumerate()
            .filter(move |(_, state)| **state == s)
            .for_each(move |(idx, _)| {
                v.push(Grid::cartesian_index(n_rows, idx));
            });
    }
    fn indices(&self, state: State) -> Vec<(usize, usize)> {
        let mut v = Vec::with_capacity(self.states.len());
        self.indices_mut(&mut v, state);
        v
    }
    pub fn classify_states(&mut self) {
        if self.vis.tile != Start {
            self.main_loop();
        }
        self.classify_perimeter();
        self.extend_outside();
        let mut unclassified = self.indices(Null);
        let mut n_old = unclassified.len();
        loop {
            while let Some((i, j)) = unclassified.pop() {
                self.classify_interior(i, j);
            }
            self.extend_outside();
            self.indices_mut(&mut unclassified, Null);
            let n = unclassified.len();
            if n == n_old {
                break;
            } else {
                n_old = n;
            }
        }
        for (i, j) in unclassified {
            let idx = self.linear_index(i, j);
            self.states[idx] = Inside;
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
        let lhs = s.parse::<Grid>().unwrap();
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
        let grid = TEST1.parse::<Grid>().unwrap();
        let vis = Visitor::try_from(&grid);
        assert!(vis.is_ok());
        let grid = TEST2.parse::<Grid>().unwrap();
        let vis = Visitor::try_from(&grid);
        assert!(vis.is_ok());
    }

    #[test]
    fn traverse() {
        let grid = TEST1.parse::<Grid>().unwrap();
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

        let grid = TEST2.parse::<Grid>().unwrap();
        let mut vis = Visitor::try_from(&grid).unwrap();
        vis.traverse();
        assert_eq!(vis.steps, 16);
    }

    static TEST3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    static TEST4: &str = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    static TEST5: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    static TEST6: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    #[test]
    fn try_bottom() {
        let grid = TEST4.parse::<Grid>().unwrap();
        let mut vis = StatefulVisitor::try_from(&grid).unwrap();
        vis.main_loop();
        vis.classify_perimeter();
        assert!(vis.try_bottom(4, 4));
        vis.classify_interior(4, 4);
        assert_eq!(vis.state(4, 4), Outside);
        assert!(vis.try_bottom(4, 5));
        vis.classify_interior(4, 5);
        assert_eq!(vis.state(4, 5), Outside);
        assert!(!vis.classify_interior(6, 3));

        let grid = TEST3.parse::<Grid>().unwrap();
        let mut vis = StatefulVisitor::try_from(&grid).unwrap();
        vis.main_loop();
        // vis.vis.traverse();
        vis.classify_perimeter();
    }
    #[test]
    fn classify_states() {
        let grid = TEST3.parse::<Grid>().unwrap();
        let mut vis = StatefulVisitor::try_from(&grid).unwrap();
        vis.main_loop();
        vis.classify_states();
        assert_eq!(vis.enclosed(), 4);

        let grid = TEST4.parse::<Grid>().unwrap();
        let mut vis = StatefulVisitor::try_from(&grid).unwrap();
        vis.main_loop();
        vis.classify_states();
        assert_eq!(vis.enclosed(), 4);

        let grid = TEST5.parse::<Grid>().unwrap();
        let mut vis = StatefulVisitor::try_from(&grid).unwrap();
        vis.main_loop();
        vis.classify_states();
        assert_eq!(vis.enclosed(), 8);

        let grid = TEST6.parse::<Grid>().unwrap();
        let mut vis = StatefulVisitor::try_from(&grid).unwrap();
        vis.main_loop();
        vis.classify_states();
        assert_eq!(vis.enclosed(), 10);
    }
}
