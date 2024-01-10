use crate::grid::*;
use std::convert::TryFrom;
use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}
