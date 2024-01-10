use crate::grid::*;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Square {
    Cube,
    Ground,
    Sphere,
}
use Square::*;
impl TryFrom<char> for Square {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Cube),
            '.' => Ok(Ground),
            'O' => Ok(Sphere),
            _ => Err(c.to_string()),
        }
    }
}
impl From<Square> for char {
    fn from(sq: Square) -> char {
        match sq {
            Cube => '#',
            Ground => '.',
            Sphere => 'O',
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Platform(Grid<Square>);

impl Platform {
    pub fn total_load(&self) -> usize {
        let n_rows = self.0.n_rows();
        let n_cols = self.0.n_cols();
        let mut sum: usize = 0;
        for j in 0..n_cols {
            for i in 0..n_rows {
                match self.0[(i, j)] {
                    Sphere => sum += n_rows - i,
                    _ => (),
                }
            }
        }
        sum
    }
    pub fn tilt_north(&mut self) {
        let grid = &mut self.0;
        let (n_rows, n_cols) = grid.shape();
        for j in 0..n_cols {
            for i in 0..n_rows {
                match grid[(i, j)] {
                    Cube => (),
                    Ground => (),
                    Sphere => {
                        let mut i = i;
                        let mut idx = grid.linear_index(i, j);
                        while i != 0 {
                            i -= 1;
                            idx -= 1;
                            match grid.inner[idx] {
                                Cube | Sphere => break,
                                Ground => {
                                    grid.inner[idx] = Sphere;
                                    grid.inner[idx + 1] = Ground;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn tilt_south(&mut self) {
        let grid = &mut self.0;
        let (n_rows, n_cols) = grid.shape();
        let last = if n_rows == 0 { 0 } else { n_rows - 1 };
        for j in 0..n_cols {
            for i in (0..n_rows).rev() {
                match grid[(i, j)] {
                    Cube => (),
                    Ground => (),
                    Sphere => {
                        let mut i = i;
                        let mut idx = grid.linear_index(i, j);
                        while i != last {
                            i += 1;
                            idx += 1;
                            match grid.inner[idx] {
                                Cube | Sphere => break,
                                Ground => {
                                    grid.inner[idx] = Sphere;
                                    grid.inner[idx - 1] = Ground;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn tilt_west(&mut self) {
        let grid = &mut self.0;
        let (n_rows, n_cols) = grid.shape();
        for i in 0..n_rows {
            for j in 0..n_cols {
                match grid[(i, j)] {
                    Cube => (),
                    Ground => (),
                    Sphere => {
                        let mut j = j;
                        while j != 0 {
                            j -= 1;
                            match grid[(i, j)] {
                                Cube | Sphere => break,
                                Ground => {
                                    grid[(i, j)] = Sphere;
                                    grid[(i, j + 1)] = Ground;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn tilt_east(&mut self) {
        let grid = &mut self.0;
        let (n_rows, n_cols) = grid.shape();
        let last = if n_cols == 0 { 0 } else { n_cols - 1 };
        for i in 0..n_rows {
            for j in (0..n_cols).rev() {
                match grid[(i, j)] {
                    Cube => (),
                    Ground => (),
                    Sphere => {
                        let mut j = j;
                        while j != last {
                            j += 1;
                            match grid[(i, j)] {
                                Cube | Sphere => break,
                                Ground => {
                                    grid[(i, j)] = Sphere;
                                    grid[(i, j - 1)] = Ground;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn cycle_and_compute_load(&mut self, n: usize) -> usize {
        if n != 0 {
            let mut cache = HashSet::new();
            let mut i: usize = 0;
            // Find the first cycle (if it exists)
            while i < n {
                let mut state = DefaultHasher::new();
                self.0.inner.hash(&mut state);
                if !cache.insert(state.finish()) {
                    break;
                }
                self.spin_cycle();
                i += 1;
            }
            // Then, iterate until the cycle length is unchanging.
            let mut m: usize = 0;
            while m != cache.len() && i < n {
                m = cache.len();
                cache.clear();
                while i < n {
                    let mut state = DefaultHasher::new();
                    self.0.inner.hash(&mut state);
                    if !cache.insert(state.finish()) {
                        break;
                    }
                    self.spin_cycle();
                    i += 1;
                }
            }
            let rem = (n - i) % m;
            for _ in 0..rem {
                self.spin_cycle();
            }
        }
        self.total_load()
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
        s.parse::<Self>()
    }
}

impl FromStr for Platform {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Platform(s.parse::<Grid<Square>>()?))
    }
}
impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    #[test]
    fn tilt_north() {
        let mut x = TEST.parse::<Platform>().unwrap();
        x.tilt_north();
        assert_eq!(x.to_string(), NORTH);
    }

    static NORTH: &str = "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    #[test]
    fn total_load() {
        let x = NORTH.parse::<Platform>().unwrap();
        assert_eq!(x.total_load(), 136);
    }

    static CYCLE1: &str = "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    static CYCLE2: &str = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";

    static CYCLE3: &str = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
    #[test]
    fn spin_cycle() {
        let mut x = TEST.parse::<Platform>().unwrap();
        x.spin_cycle();
        assert_eq!(x.to_string(), CYCLE1);
        x.spin_cycle();
        assert_eq!(x.to_string(), CYCLE2);
        x.spin_cycle();
        assert_eq!(x.to_string(), CYCLE3);
    }
    #[test]
    fn cycle_and_compute_load() {
        let mut x = TEST.parse::<Platform>().unwrap();
        let lhs = x.cycle_and_compute_load(1_000_000_000);
        assert_eq!(lhs, 64);
    }
}
