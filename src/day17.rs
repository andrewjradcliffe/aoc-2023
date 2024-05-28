use crate::grid::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

pub struct City(Grid<u16>);

impl City {
    fn initial_direction(&self) -> Direction {
        if self.0[(0, 1)] < self.0[(1, 0)] {
            Right
        } else {
            Down
        }
    }
    pub fn minimal_heat_loss(&self) -> u16 {
        let (n_rows, n_cols) = self.0.shape();
        let mut vis = Visitor {
            current: (0, 0),
            endpoint: (n_rows - 1, n_cols - 1),
            dir: self.initial_direction(),
            n_blocks: 0,
            heat_loss: 0,
            grid: &self.0,
        };
        vis.visit();
        vis.heat_loss
    }
}

const OFFSET: u16 = '0' as u16;
const BASE: u16 = 10;
impl FromStr for City {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        let mut n_rows: usize = 0;
        for line in s.lines() {
            n_rows += 1;
            for c in line.chars() {
                let x = c as u16 - OFFSET;
                if x < BASE {
                    v.push(x);
                } else {
                    return Err(c.to_string());
                }
            }
        }
        let n = v.len();
        let n_cols = n / n_rows;
        if n % n_rows != 0 {
            Err(s.to_string())
        } else {
            let mut x = Grid::from_vec(v, n_cols, n_rows);
            x.transpose_mut();
            Ok(City(x))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Visitor<'a> {
    current: (usize, usize),
    endpoint: (usize, usize),
    dir: Direction,
    n_blocks: u8,
    heat_loss: u16,
    grid: &'a Grid<u16>,
}

impl Visitor<'_> {
    pub fn up(&self) -> u16 {
        if self.current.0 == 0 {
            u16::MAX
        } else {
            self.grid[(self.current.0 - 1, self.current.1)]
        }
    }
    pub fn down(&self) -> u16 {
        if self.current.0 + 1 == self.grid.n_rows {
            u16::MAX
        } else {
            self.grid[(self.current.0 + 1, self.current.1)]
        }
    }
    pub fn left(&self) -> u16 {
        if self.current.1 == 0 {
            u16::MAX
        } else {
            self.grid[(self.current.0, self.current.1 - 1)]
        }
    }
    pub fn right(&self) -> u16 {
        if self.current.1 + 1 == self.grid.n_rows {
            u16::MAX
        } else {
            self.grid[(self.current.0, self.current.1 + 1)]
        }
    }
    pub fn optimal_direction(&self) -> Direction {
        if self.n_blocks < 3 {
            let (lhs, mid, rhs) = match self.dir {
                Up => ((Left, self.left()), (Up, self.up()), (Right, self.right())),
                Down => (
                    (Left, self.left()),
                    (Down, self.down()),
                    (Right, self.right()),
                ),
                Left => ((Down, self.down()), (Left, self.left()), (Up, self.up())),
                Right => ((Down, self.down()), (Right, self.right()), (Up, self.up())),
            };
            let lm = lhs.1 <= mid.1;
            let mr = mid.1 <= rhs.1;
            let lr = lhs.1 <= rhs.1;
            if lm {
                if mr {
                    lhs.0
                } else if lr {
                    lhs.0
                } else {
                    rhs.0
                }
            } else if mr {
                mid.0
            } else if lr {
                mid.0
            } else {
                rhs.0
            }
        } else {
            let (lhs, rhs) = match self.dir {
                Up | Down => ((Left, self.left()), (Right, self.right())),
                Left | Right => ((Up, self.up()), (Down, self.down())),
            };
            if lhs.1 < rhs.1 {
                lhs.0
            } else {
                rhs.0
            }
        }
    }
    fn move_up(&mut self) {
        self.dir = Up;
        self.current.0 -= 1;
    }
    fn move_down(&mut self) {
        self.dir = Down;
        self.current.0 += 1;
    }
    fn move_left(&mut self) {
        self.dir = Left;
        self.current.1 -= 1;
    }
    fn move_right(&mut self) {
        self.dir = Right;
        self.current.1 += 1;
    }
    pub fn advance(&mut self) -> bool {
        if self.current == self.endpoint {
            false
        } else {
            let dir = self.optimal_direction();
            match dir {
                Up => self.move_up(),
                Down => self.move_down(),
                Left => self.move_left(),
                Right => self.move_right(),
            }
            if self.n_blocks == 3 {
                self.n_blocks = 0;
            } else {
                self.n_blocks += 1;
            }
            self.heat_loss += self.grid[self.current];
            true
        }
    }
    pub fn visit(&mut self) {
        // while self.advance() {}
        loop {
            if !self.advance() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    // #[test]
    // fn visit() {
    //     let city = TEST.parse::<City>().unwrap();
    //     assert_eq!(city.minimal_heat_loss(), 102);
    // }
}
